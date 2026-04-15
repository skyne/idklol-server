mod prompt_store;

use std::path::Path;
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use futures::StreamExt;
use idklol_common::cli::service_command_interface::{
    build_help_text, run_stdin_command_loop, CustomCommandSpec, ServiceCommandEvent,
    ServiceControlCommand,
};
use idklol_common::config::env_config::EnvConfig;
use idklol_common::logging::logger_service::LoggerService;
use prompt_store::PromptStore;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinSet;
use tracing::{error, info, info_span, warn, Instrument};

const SERVICE_NAME: &str = "npc-interactions-bridge";
const BRIDGE_CUSTOM_COMMANDS: &[CustomCommandSpec] = &[
    CustomCommandSpec {
        name: "chat",
        usage: "<free text>",
        description: "Send free text through the bridge using the active prompt.",
    },
    CustomCommandSpec {
        name: "prompts.list",
        usage: "",
        description: "List discovered prompt keys and indicate the active one.",
    },
    CustomCommandSpec {
        name: "prompts.reload",
        usage: "",
        description: "Reload prompt files from disk.",
    },
    CustomCommandSpec {
        name: "prompt.use",
        usage: "<prompt-key>",
        description: "Set active prompt key used by requests without inline prompt override.",
    },
    CustomCommandSpec {
        name: "prompt.show",
        usage: "",
        description: "Show active prompt key and a short preview.",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuntimeAction {
    Restart,
    Shutdown,
}

#[derive(Clone, Debug)]
struct BridgeConfig {
    nats_url: String,
    ollama_base_url: String,
    ollama_model: String,
    request_subject: String,
    response_subject: String,
    default_temperature: f32,
    default_max_tokens: u32,
    debug_npc_id: String,
    debug_player_id: String,
    prompts_dir: String,
    default_prompt_key: Option<String>,
}

fn discover_default_prompts_dir() -> String {
    let candidates = [
        "/srv/prompts",
        "npc-interactions-bridge/prompts",
        "./prompts",
        concat!(env!("CARGO_MANIFEST_DIR"), "/prompts"),
    ];

    for candidate in candidates {
        if Path::new(candidate).exists() {
            return candidate.to_string();
        }
    }

    concat!(env!("CARGO_MANIFEST_DIR"), "/prompts").to_string()
}

impl BridgeConfig {
    fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let nats_url = EnvConfig::get_optional("NATS_URL")?
            .unwrap_or_else(|| "nats://nats:4222".to_string());

        let ollama_base_url = EnvConfig::get_optional("OLLAMA_BASE_URL")?
            .unwrap_or_else(|| "http://ollama:11434".to_string());

        let ollama_model = EnvConfig::get_optional("OLLAMA_MODEL")?
            .unwrap_or_else(|| "llama3.2:3b".to_string());

        let request_subject = EnvConfig::get_optional("NPC_BRIDGE_REQUEST_SUBJECT")?
            .unwrap_or_else(|| "npc.interactions.request".to_string());

        let response_subject = EnvConfig::get_optional("NPC_BRIDGE_RESPONSE_SUBJECT")?
            .unwrap_or_else(|| "npc.interactions.response".to_string());

        let default_temperature = EnvConfig::get_optional_parsed::<f32>("NPC_BRIDGE_DEFAULT_TEMPERATURE")?
            .unwrap_or(0.7);

        let default_max_tokens = EnvConfig::get_optional_parsed::<u32>("NPC_BRIDGE_DEFAULT_MAX_TOKENS")?
            .unwrap_or(180);

        let debug_npc_id = EnvConfig::get_optional("NPC_BRIDGE_DEBUG_NPC_ID")?
            .unwrap_or_else(|| "debug-npc".to_string());

        let debug_player_id = EnvConfig::get_optional("NPC_BRIDGE_DEBUG_PLAYER_ID")?
            .unwrap_or_else(|| "debug-player".to_string());

        let prompts_dir = EnvConfig::get_optional("NPC_BRIDGE_PROMPTS_DIR")?
            .unwrap_or_else(discover_default_prompts_dir);

        let default_prompt_key = EnvConfig::get_optional("NPC_BRIDGE_DEFAULT_PROMPT")?;

        Ok(Self {
            nats_url,
            ollama_base_url,
            ollama_model,
            request_subject,
            response_subject,
            default_temperature,
            default_max_tokens,
            debug_npc_id,
            debug_player_id,
            prompts_dir,
            default_prompt_key,
        })
    }
}

#[derive(Clone)]
struct BridgeState {
    config: BridgeConfig,
    http_client: reqwest::Client,
    prompt_store: Arc<RwLock<PromptStore>>,
}

#[derive(Debug, Deserialize)]
struct NpcInteractionRequest {
    request_id: Option<String>,
    npc_id: String,
    npc_name: Option<String>,
    npc_role: Option<String>,
    player_id: Option<String>,
    player_name: Option<String>,
    nearby_player_names: Option<Vec<String>>,
    message: String,
    context: Option<String>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    prompt_key: Option<String>,
    system_prompt: Option<String>,
}

#[derive(Debug, Serialize)]
struct NpcInteractionResponse {
    request_id: Option<String>,
    npc_id: String,
    player_id: Option<String>,
    response: String,
    model: String,
    done: bool,
}

#[derive(Debug, Serialize)]
struct NpcInteractionError {
    request_id: Option<String>,
    npc_id: Option<String>,
    error: String,
}

#[derive(Debug, Serialize)]
struct OllamaGenerateRequest<'a> {
    model: &'a str,
    prompt: String,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<&'a str>,
    options: OllamaOptions,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    temperature: f32,
    num_predict: u32,
}

#[derive(Debug, Deserialize)]
struct OllamaGenerateResponse {
    model: String,
    response: String,
    done: bool,
}

#[derive(Debug)]
enum BridgeGenerateError {
    Prompt(String),
    Http(reqwest::Error),
}

impl std::fmt::Display for BridgeGenerateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Prompt(message) => write!(f, "{message}"),
            Self::Http(source) => write!(f, "{source}"),
        }
    }
}

impl std::error::Error for BridgeGenerateError {}

impl From<reqwest::Error> for BridgeGenerateError {
    fn from(value: reqwest::Error) -> Self {
        Self::Http(value)
    }
}

fn build_prompt(request: &NpcInteractionRequest) -> String {
    let mut sections = vec![format!("NPC ID: {}", request.npc_id)];

    if let Some(npc_name) = request.npc_name.as_ref() {
        sections.push(format!("NPC Name: {npc_name}"));
    }
    if let Some(npc_role) = request.npc_role.as_ref() {
        sections.push(format!("NPC Role: {npc_role}"));
    }

    sections.push(
        request
            .player_id
            .as_ref()
            .map(|player_id| format!("Player ID: {player_id}"))
            .unwrap_or_else(|| "Player ID: unknown".to_string()),
    );

    if let Some(player_name) = request.player_name.as_ref() {
        sections.push(format!("Player Name: {player_name}"));
    }

    if let Some(nearby) = request.nearby_player_names.as_ref() {
        if !nearby.is_empty() {
            sections.push(format!("Nearby Players: {}", nearby.join(", ")));
        }
    }

    if let Some(context) = request.context.as_ref() {
        sections.push(format!("World Context: {context}"));
    }

    sections.push(format!("Player Message: {}", request.message));
    sections.push("If nearby players are listed, greet them by name in the response.".to_string());
    sections.push("Respond as the NPC with concise, role-appropriate dialogue.".to_string());

    sections.join("\n")
}

fn render_system_prompt_template(template: &str, request: &NpcInteractionRequest) -> String {
    let player_id = request.player_id.as_deref().unwrap_or("unknown-player");
    let player_name = request.player_name.as_deref().unwrap_or(player_id);
    let npc_name = request.npc_name.as_deref().unwrap_or(request.npc_id.as_str());
    let npc_role = request.npc_role.as_deref().unwrap_or("");
    let world_context = request.context.as_deref().unwrap_or("");

    let nearby_players = request
        .nearby_player_names
        .as_ref()
        .map(|nearby| nearby.join(", "))
        .unwrap_or_default();

    let replacements = [
        ("{{npc-name}}", npc_name),
        ("{{npc_name}}", npc_name),
        ("{npc-name}", npc_name),
        ("{npc_name}", npc_name),
        ("{{npc-id}}", request.npc_id.as_str()),
        ("{{npc_id}}", request.npc_id.as_str()),
        ("{npc-id}", request.npc_id.as_str()),
        ("{npc_id}", request.npc_id.as_str()),
        ("{{npc-role}}", npc_role),
        ("{{npc_role}}", npc_role),
        ("{npc-role}", npc_role),
        ("{npc_role}", npc_role),
        ("{{player-name}}", player_name),
        ("{{player_name}}", player_name),
        ("{player-name}", player_name),
        ("{player_name}", player_name),
        ("{{player-id}}", player_id),
        ("{{player_id}}", player_id),
        ("{player-id}", player_id),
        ("{player_id}", player_id),
        ("{{nearby-players}}", nearby_players.as_str()),
        ("{{nearby_players}}", nearby_players.as_str()),
        ("{nearby-players}", nearby_players.as_str()),
        ("{nearby_players}", nearby_players.as_str()),
        ("{{world-context}}", world_context),
        ("{{world_context}}", world_context),
        ("{world-context}", world_context),
        ("{world_context}", world_context),
        ("{{player-message}}", request.message.as_str()),
        ("{{player_message}}", request.message.as_str()),
        ("{player-message}", request.message.as_str()),
        ("{player_message}", request.message.as_str()),
    ];

    let mut rendered = template.to_string();
    for (token, value) in replacements {
        rendered = rendered.replace(token, value);
    }

    rendered
}

async fn generate_npc_response(
    state: &BridgeState,
    request: &NpcInteractionRequest,
) -> Result<OllamaGenerateResponse, BridgeGenerateError> {
    let endpoint = format!(
        "{}/api/generate",
        state.config.ollama_base_url.trim_end_matches('/')
    );

    let (selected_prompt_key, selected_system_prompt) = if let Some(inline_system_prompt) = request.system_prompt.clone() {
        (Some("inline/system".to_string()), Some(inline_system_prompt))
    } else {
        let prompt_store = state.prompt_store.read().await;
        prompt_store
            .resolve_system_prompt(request.prompt_key.as_deref())
            .map_err(|error| BridgeGenerateError::Prompt(error.to_string()))?
    };

    if let Some(prompt_key) = selected_prompt_key.as_deref() {
        info!(%prompt_key, "resolved prompt for npc interaction");
    }

    let rendered_system_prompt = selected_system_prompt
        .as_deref()
        .map(|template| render_system_prompt_template(template, request));

    let payload = OllamaGenerateRequest {
        model: &state.config.ollama_model,
        prompt: build_prompt(request),
        stream: false,
        system: rendered_system_prompt.as_deref(),
        options: OllamaOptions {
            temperature: request
                .temperature
                .unwrap_or(state.config.default_temperature),
            num_predict: request.max_tokens.unwrap_or(state.config.default_max_tokens),
        },
    };

    let response = state
        .http_client
        .post(endpoint)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?
        .json::<OllamaGenerateResponse>()
        .await?;

    Ok(response)
}

async fn publish_json<T: Serialize>(
    client: &async_nats::Client,
    subject: &str,
    payload: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = serde_json::to_vec(payload)?;
    client.publish(subject.to_string(), bytes.into()).await?;
    client.flush().await?;
    Ok(())
}

async fn handle_message(
    message: async_nats::Message,
    state: Arc<BridgeState>,
    client: async_nats::Client,
) {
    let response_subject = message
        .reply
        .as_ref()
        .map(std::string::ToString::to_string)
        .unwrap_or_else(|| state.config.response_subject.clone());

    let request = match serde_json::from_slice::<NpcInteractionRequest>(&message.payload) {
        Ok(req) => req,
        Err(err) => {
            warn!(%err, "npc interaction request payload is invalid JSON");
            let error_payload = NpcInteractionError {
                request_id: None,
                npc_id: None,
                error: format!("invalid request payload: {err}"),
            };

            if let Err(pub_err) = publish_json(&client, &response_subject, &error_payload).await {
                error!(%pub_err, "failed to publish invalid-payload error response");
            }
            return;
        }
    };

    info!(
        request_id = request.request_id.as_deref().unwrap_or("none"),
        npc_id = %request.npc_id,
        "processing npc interaction"
    );

    let request_id_for_log = request
        .request_id
        .as_deref()
        .unwrap_or("none")
        .to_string();
    let npc_id_for_log = request.npc_id.clone();
    let ollama_started_at = Instant::now();

    match generate_npc_response(&state, &request).await {
        Ok(ollama_response) => {
            let ollama_latency_ms = ollama_started_at.elapsed().as_millis() as u64;
            info!(
                request_id = %request_id_for_log,
                npc_id = %npc_id_for_log,
                model = %ollama_response.model,
                ollama_latency_ms = ollama_latency_ms,
                "ollama response received"
            );

            let response_payload = NpcInteractionResponse {
                request_id: request.request_id,
                npc_id: request.npc_id,
                player_id: request.player_id,
                response: ollama_response.response.trim().to_string(),
                model: ollama_response.model,
                done: ollama_response.done,
            };

            if let Err(err) = publish_json(&client, &response_subject, &response_payload).await {
                error!(%err, "failed to publish npc interaction response");
            }
        }
        Err(err) => {
            let ollama_latency_ms = ollama_started_at.elapsed().as_millis() as u64;
            warn!(
                request_id = %request_id_for_log,
                npc_id = %npc_id_for_log,
                ollama_latency_ms = ollama_latency_ms,
                %err,
                "ollama request failed"
            );
            let error_payload = NpcInteractionError {
                request_id: request.request_id,
                npc_id: Some(request.npc_id),
                error: format!("ollama request failed: {err}"),
            };

            if let Err(pub_err) = publish_json(&client, &response_subject, &error_payload).await {
                error!(%pub_err, "failed to publish ollama failure response");
            }
        }
    }
}

fn build_cli_request_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0);

    format!("cli-{timestamp}")
}

async fn handle_prompts_list_command(state: &BridgeState) {
    let prompt_store = state.prompt_store.read().await;
    let active_prompt = prompt_store.active_prompt_key();
    let prompt_keys = prompt_store.list_prompt_keys();

    if prompt_keys.is_empty() {
        println!("no prompts discovered in {}", prompt_store.root_dir().display());
        return;
    }

    println!("discovered prompts in {}:", prompt_store.root_dir().display());
    for prompt_key in prompt_keys {
        let marker = if Some(prompt_key.as_str()) == active_prompt {
            "*"
        } else {
            " "
        };
        println!("  {marker} {prompt_key}");
    }
}

async fn handle_prompts_reload_command(state: &BridgeState) {
    let mut prompt_store = state.prompt_store.write().await;
    match prompt_store.reload() {
        Ok(summary) => {
            println!(
                "reloaded {} prompts from {}",
                summary.prompt_count,
                prompt_store.root_dir().display()
            );
            println!(
                "active prompt: {}",
                summary
                    .active_prompt_key
                    .as_deref()
                    .unwrap_or("<none>")
            );
        }
        Err(error) => {
            println!("failed to reload prompts: {error}");
        }
    }
}

async fn handle_prompt_use_command(state: &BridgeState, args: &str) {
    let prompt_key = args.trim();
    if prompt_key.is_empty() {
        println!("usage: .prompt.use <prompt-key>");
        return;
    }

    let mut prompt_store = state.prompt_store.write().await;
    match prompt_store.set_active_prompt(prompt_key) {
        Ok(()) => {
            println!("active prompt set to {prompt_key}");
        }
        Err(error) => {
            println!("failed to set active prompt: {error}");
        }
    }
}

async fn handle_prompt_show_command(state: &BridgeState) {
    let prompt_store = state.prompt_store.read().await;
    let Some(active_prompt_key) = prompt_store.active_prompt_key() else {
        println!("active prompt: <none>");
        return;
    };

    let preview = prompt_store
        .active_prompt_preview(220)
        .unwrap_or_else(|| "<empty>".to_string());

    println!("active prompt: {active_prompt_key}");
    println!("preview: {preview}");
}

async fn handle_cli_chat_command(state: &BridgeState, args: &str) {
    let text = args.trim();
    if text.is_empty() {
        println!("usage: .chat <free text>");
        return;
    }

    let request = NpcInteractionRequest {
        request_id: Some(build_cli_request_id()),
        npc_id: state.config.debug_npc_id.clone(),
        npc_name: None,
        npc_role: None,
        player_id: Some(state.config.debug_player_id.clone()),
        player_name: None,
        nearby_player_names: None,
        message: text.to_string(),
        context: None,
        temperature: None,
        max_tokens: None,
        prompt_key: None,
        system_prompt: None,
    };

    let ollama_started_at = Instant::now();

    match generate_npc_response(state, &request).await {
        Ok(response) => {
            let ollama_latency_ms = ollama_started_at.elapsed().as_millis() as u64;
            info!(
                request_id = request.request_id.as_deref().unwrap_or("none"),
                npc_id = %request.npc_id,
                model = %response.model,
                ollama_latency_ms = ollama_latency_ms,
                "debug chat response received"
            );
            println!(
                "[{} @ {} | {} ms] {}",
                request.npc_id,
                response.model,
                ollama_latency_ms,
                response.response.trim()
            );
        }
        Err(err) => {
            let ollama_latency_ms = ollama_started_at.elapsed().as_millis() as u64;
            warn!(
                request_id = request.request_id.as_deref().unwrap_or("none"),
                npc_id = %request.npc_id,
                ollama_latency_ms = ollama_latency_ms,
                %err,
                "debug chat request failed"
            );
            println!("chat failed: {err}");
        }
    }
}

async fn handle_service_command(
    command: ServiceCommandEvent,
    state: &BridgeState,
) -> Option<RuntimeAction> {
    match command {
        ServiceCommandEvent::Control(control) => match control {
            ServiceControlCommand::Help => {
                println!("{}", build_help_text(SERVICE_NAME, BRIDGE_CUSTOM_COMMANDS));
                None
            }
            ServiceControlCommand::Restart => {
                info!("received .rs command, restarting service runtime");
                Some(RuntimeAction::Restart)
            }
            ServiceControlCommand::Shutdown => {
                info!("received .shutdown command, shutting down service runtime");
                Some(RuntimeAction::Shutdown)
            }
        },
        ServiceCommandEvent::Custom { name, args } => {
            match name.as_str() {
                "chat" => {
                    handle_cli_chat_command(state, &args).await;
                }
                "prompts.list" => {
                    handle_prompts_list_command(state).await;
                }
                "prompts.reload" => {
                    handle_prompts_reload_command(state).await;
                }
                "prompt.use" => {
                    handle_prompt_use_command(state, &args).await;
                }
                "prompt.show" => {
                    handle_prompt_show_command(state).await;
                }
                unknown => {
                    println!("unknown command: .{unknown}. Try .help");
                }
            }

            None
        }
    }
}

async fn run_runtime_loop(
    state: Arc<BridgeState>,
    client: async_nats::Client,
    mut subscriber: async_nats::Subscriber,
    command_rx: &mut mpsc::UnboundedReceiver<ServiceCommandEvent>,
) -> RuntimeAction {
    let mut worker_tasks: JoinSet<()> = JoinSet::new();
    let mut command_channel_open = true;

    let action = loop {
        tokio::select! {
            signal_result = tokio::signal::ctrl_c() => {
                if let Err(err) = signal_result {
                    warn!(%err, "failed to install CTRL-C signal handler");
                }
                break RuntimeAction::Shutdown;
            }

            maybe_command = command_rx.recv(), if command_channel_open => {
                match maybe_command {
                    Some(command) => {
                        if let Some(action) = handle_service_command(command, &state).await {
                            break action;
                        }
                    }
                    None => {
                        command_channel_open = false;
                        warn!("service command input closed; continuing without interactive command input");
                    }
                }
            }

            maybe_message = subscriber.next() => {
                match maybe_message {
                    Some(message) => {
                        let nats_subject = message.subject.to_string();
                        let has_reply = message.reply.is_some();
                        let payload_size = message.payload.len();
                        let worker_state = state.clone();
                        let worker_client = client.clone();
                        worker_tasks.spawn(
                            async move {
                                handle_message(message, worker_state, worker_client).await;
                            }
                            .instrument(info_span!(
                                "nats.npc_interaction.request",
                                nats_subject = %nats_subject,
                                has_reply = has_reply,
                                payload_size = payload_size
                            ))
                        );
                    }
                    None => {
                        warn!("nats subscription closed, restarting runtime");
                        break RuntimeAction::Restart;
                    }
                }
            }

            join_result = worker_tasks.join_next(), if !worker_tasks.is_empty() => {
                if let Some(Err(err)) = join_result {
                    warn!(%err, "npc interaction worker task failed");
                }
            }
        }
    };

    info!(?action, "draining in-flight npc interaction tasks");
    while let Some(result) = worker_tasks.join_next().await {
        if let Err(err) = result {
            warn!(%err, "npc interaction worker task failed during drain");
        }
    }

    if let Err(err) = client.flush().await {
        warn!(%err, "failed to flush nats client during drain");
    }

    action
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    EnvConfig::init_from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));
    let _logger_guard = LoggerService::init_from_env("idklol-npc-interactions-bridge")?;

    let config = BridgeConfig::from_env()?;

    let prompt_store = PromptStore::load(
        config.prompts_dir.clone().into(),
        config.default_prompt_key.clone(),
    )?;
    let prompt_summary = prompt_store.summary();

    info!(
        nats_url = %config.nats_url,
        request_subject = %config.request_subject,
        response_subject = %config.response_subject,
        model = %config.ollama_model,
        debug_npc_id = %config.debug_npc_id,
        prompts_dir = %config.prompts_dir,
        prompt_count = prompt_summary.prompt_count,
        active_prompt = ?prompt_summary.active_prompt_key,
        "starting npc interactions bridge"
    );

    let state = Arc::new(BridgeState {
        config,
        http_client: reqwest::Client::new(),
        prompt_store: Arc::new(RwLock::new(prompt_store)),
    });

    let (command_tx, mut command_rx) = mpsc::unbounded_channel::<ServiceCommandEvent>();
    tokio::spawn(async move {
        if let Err(err) = run_stdin_command_loop(command_tx, SERVICE_NAME, BRIDGE_CUSTOM_COMMANDS).await {
            warn!(%err, "service command interface stopped");
        }
    });

    loop {
        info!(
            nats_url = %state.config.nats_url,
            request_subject = %state.config.request_subject,
            "connecting bridge runtime to nats"
        );

        let client = async_nats::connect(&state.config.nats_url).await?;
        let subscriber = client
            .subscribe(state.config.request_subject.clone())
            .await?;

        info!("bridge runtime is running");
        let action = run_runtime_loop(state.clone(), client, subscriber, &mut command_rx).await;

        match action {
            RuntimeAction::Restart => {
                info!("runtime restarted");
                continue;
            }
            RuntimeAction::Shutdown => {
                info!("runtime shutdown requested");
                break;
            }
        }
    }

    info!("npc interactions bridge stopped");

    Ok(())
}