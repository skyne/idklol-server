use chrono::Local;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use tracing::{debug, info, info_span, Instrument};

use crate::chat::chat_service_server::ChatService;
use crate::chat::ChatMessage;

const DATE_FORMAT_STRING: &str = "[%H:%M:%S]";
const DEFAULT_SPAWN_SUBJECT: &str = "npc.spawn.request";
const DEFAULT_PLAYER_CONTEXT_SUBJECT: &str = "server.player_context.get";

#[derive(Debug, Serialize)]
struct PlayerContextRequest {
    character_id: String,
}

#[derive(Debug, Deserialize)]
struct PlayerContextResponse {
    #[serde(default)]
    error: Option<String>,
    zone_id: Option<String>,
    position_x: Option<f64>,
    position_y: Option<f64>,
    position_z: Option<f64>,
    yaw: Option<f64>,
}

#[derive(Debug, Serialize)]
struct SpawnNpcRequest {
    npc_id: String,
    character_id: String,
    zone_id: String,
    position_x: f64,
    position_y: f64,
    position_z: f64,
    yaw: f64,
}

#[derive(Debug)]
struct SpawnCommand {
    npc_id: String,
}

fn parse_spawn_command(message: &str) -> Option<SpawnCommand> {
    let trimmed = message.trim();
    let body = trimmed.strip_prefix('!').or_else(|| trimmed.strip_prefix('.'))?;
    let mut parts = body.split_whitespace();
    let command_name = parts.next()?.to_ascii_lowercase();

    if command_name != "spawnnpc" && command_name != "spawn" {
        return None;
    }

    let npc_id = parts.next().map(str::trim).unwrap_or_default();

    if npc_id.is_empty() {
        return None;
    }

    Some(SpawnCommand {
        npc_id: npc_id.to_string(),
    })
}

#[derive(Debug)]
pub struct MyChatService {
    subscribers: Arc<Mutex<Vec<mpsc::Sender<Result<ChatMessage, Status>>>>>,
    nats_client: Option<async_nats::Client>,
    spawn_subject: String,
    player_context_subject: String,
}

impl MyChatService {
    pub async fn from_env() -> Self {
        let nats_url = std::env::var("NATS_URL").unwrap_or_else(|_| "nats://nats:4222".to_string());
        let spawn_subject = std::env::var("CHAT_NPC_SPAWN_SUBJECT")
            .unwrap_or_else(|_| DEFAULT_SPAWN_SUBJECT.to_string());
        let player_context_subject = std::env::var("CHAT_PLAYER_CONTEXT_SUBJECT")
            .unwrap_or_else(|_| DEFAULT_PLAYER_CONTEXT_SUBJECT.to_string());
        let nats_client = async_nats::connect(&nats_url).await.ok();

        Self {
            subscribers: Arc::new(Mutex::new(Vec::new())),
            nats_client,
            spawn_subject,
            player_context_subject,
        }
    }

    async fn broadcast_message(&self, message: ChatMessage) {
        let subscribers = self.subscribers.lock().await;
        for tx in subscribers.iter() {
            let _ = tx.send(Ok(message.clone())).await;
        }
    }

    async fn execute_spawn_command(
        &self,
        sender: &str,
        sender_character_id: Option<&str>,
        command: SpawnCommand,
    ) -> ChatMessage {
        let status_text = match self
            .resolve_and_publish_spawn(sender_character_id, &command.npc_id)
            .await
        {
            Ok(()) => format!("Spawn requested for NPC '{}' near {}.", command.npc_id, sender),
            Err(error) => format!("Spawn request failed for NPC '{}': {}", command.npc_id, error),
        };

        ChatMessage {
            timestamp: Local::now().format(DATE_FORMAT_STRING).to_string(),
            sender: "System".to_string(),
            message: status_text,
        }
    }

    async fn resolve_and_publish_spawn(
        &self,
        sender_character_id: Option<&str>,
        npc_id: &str,
    ) -> Result<(), String> {
        let character_id = sender_character_id
            .filter(|value| !value.is_empty())
            .ok_or_else(|| "missing x-character-id metadata".to_string())?;

        let nats_client = self
            .nats_client
            .as_ref()
            .ok_or_else(|| "NATS client unavailable".to_string())?;

        let context_request = serde_json::to_vec(&PlayerContextRequest {
            character_id: character_id.to_string(),
        })
        .map_err(|error| error.to_string())?;

        let context_message = nats_client
            .request(self.player_context_subject.clone(), context_request.into())
            .await
            .map_err(|error| error.to_string())?;

        let context: PlayerContextResponse = serde_json::from_slice(&context_message.payload)
            .map_err(|error| error.to_string())?;

        if let Some(error) = context.error {
            return Err(error);
        }

        let spawn_request = SpawnNpcRequest {
            npc_id: npc_id.to_string(),
            character_id: character_id.to_string(),
            zone_id: context
                .zone_id
                .ok_or_else(|| "missing zone_id in player context response".to_string())?,
            position_x: context
                .position_x
                .ok_or_else(|| "missing position_x in player context response".to_string())?,
            position_y: context
                .position_y
                .ok_or_else(|| "missing position_y in player context response".to_string())?,
            position_z: context
                .position_z
                .ok_or_else(|| "missing position_z in player context response".to_string())?,
            yaw: context
                .yaw
                .ok_or_else(|| "missing yaw in player context response".to_string())?,
        };

        let payload = serde_json::to_vec(&spawn_request).map_err(|error| error.to_string())?;
        nats_client
            .publish(self.spawn_subject.clone(), payload.into())
            .await
            .map_err(|error| error.to_string())?;

        Ok(())
    }
}

impl Default for MyChatService {
    fn default() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(Vec::new())),
            nats_client: None,
            spawn_subject: DEFAULT_SPAWN_SUBJECT.to_string(),
            player_context_subject: DEFAULT_PLAYER_CONTEXT_SUBJECT.to_string(),
        }
    }
}

#[tonic::async_trait]
impl ChatService for MyChatService {
    type StreamStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn message(
        &self,
        request: Request<ChatMessage>,
    ) -> Result<Response<()>, Status> {
        let has_authorization = request.metadata().get("authorization").is_some();
        let sender_character_id = request
            .metadata()
            .get("x-character-id")
            .and_then(|value| value.to_str().ok())
            .map(str::to_string);

        async move {
            info!("chat message request received");
            debug!(has_authorization, "authorization header inspected");

            let mut msg = request.into_inner();
            msg.timestamp = Local::now().format(DATE_FORMAT_STRING).to_string();

            if let Some(command) = parse_spawn_command(&msg.message) {
                let system_message = self
                    .execute_spawn_command(&msg.sender, sender_character_id.as_deref(), command)
                    .await;
                self.broadcast_message(system_message).await;
                return Ok(Response::new(()));
            }

            self.broadcast_message(msg).await;

            Ok(Response::new(()))
        }
        .instrument(info_span!(
            "grpc.chat.message",
            has_authorization = has_authorization
        ))
        .await
    }

    async fn stream(
        &self,
        _request: Request<()>,
    ) -> Result<Response<Self::StreamStream>, Status> {
        async move {
            info!("chat stream request received");

            let (tx, rx) = mpsc::channel(16);

            {
                let mut subscribers = self.subscribers.lock().await;
                subscribers.push(tx);
            }

            Ok(Response::new(ReceiverStream::new(rx)))
        }
        .instrument(info_span!("grpc.chat.stream"))
        .await
    }
}