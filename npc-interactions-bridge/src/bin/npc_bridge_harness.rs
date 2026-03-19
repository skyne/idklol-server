use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::time::{sleep, timeout};

const DEFAULT_COMPOSE_FILE: &str = "docker-compose.yml";
const DEFAULT_NATS_URL: &str = "nats://127.0.0.1:4222";
const DEFAULT_REQUEST_SUBJECT: &str = "npc.interactions.request";
const DEFAULT_RESPONSE_SUBJECT: &str = "npc.interactions.response";
const DEFAULT_NATS_MONITOR_SUBSZ_URL: &str = "http://127.0.0.1:8222/subsz?subs=1";
const DEFAULT_PROMPT_CANDIDATES: usize = 3;
const DEFAULT_TEST_CASES: usize = 12;
const DEFAULT_SEED: u64 = 42;
const DEFAULT_SCENARIO_FAMILIES: &str = "merchant_greeting,guard_challenge,busy_worker_reply";
const DEFAULT_REQUEST_TIMEOUT_SECONDS: u64 = 300;
const DEFAULT_REQUEST_TEMPERATURE: f32 = 0.4;
const DEFAULT_REQUEST_MAX_TOKENS: u32 = 64;
const DEFAULT_REQUEST_RETRIES: usize = 2;
const DEFAULT_REQUEST_RETRY_BACKOFF_MS: u64 = 1500;
const DEFAULT_STRICT_SERIAL_MODE: bool = true;
const DEFAULT_MATCH_CANDIDATE_FAMILY: bool = true;
const DEFAULT_INCLUDE_PREVIOUS_RECOMMENDATIONS: bool = true;
const DEFAULT_PREVIOUS_RECOMMENDATIONS_LIMIT: usize = 20;
const DEFAULT_PREFLIGHT_MIN_SUCCESS_RATE: f64 = 0.80;
const DEFAULT_PREFLIGHT_MAX_TIMEOUTS: usize = 2;
const DEFAULT_RUBRIC_VERSION: &str = "rubric_v1";
const DEFAULT_PERFORMANCE_LATENCY_TARGET_MS: f64 = 2500.0;
const DEFAULT_PERFORMANCE_QUALITY_WEIGHT: f64 = 0.75;
const DEFAULT_PERFORMANCE_SPEED_WEIGHT: f64 = 0.25;
const DEFAULT_PROMOTION_MIN_OPERATIONAL_SCORE: f64 = 9.0;
const DEFAULT_PROMOTION_MIN_QUALITY_SCORE: f64 = 7.5;
const DEFAULT_PROMOTION_MIN_GRADED_SAMPLES: usize = 6;
const DEFAULT_PROMOTION_MAX_TIMEOUTS: usize = 0;
const DEFAULT_CYCLE_COUNT: usize = 1;
const DEFAULT_AUTOPROMOTE_TO_PROMPTS: bool = false;
const DEFAULT_PROMPTS_DIR: &str = "npc-interactions-bridge/prompts";
const DEFAULT_PROMOTION_TEMPLATE_PATH: &str = "templates/auto/system.prompt";
const DEFAULT_FAMILY_TEMPLATES_DIR: &str = "templates/families";
const DEFAULT_MATERIALIZE_FAMILY_TEMPLATES: bool = true;

#[derive(Debug, Clone)]
struct HarnessConfig {
    openai_api_key: Option<String>,
    openai_model: String,
    compose_file: String,
    auto_start_services: bool,
    preflight_only: bool,
    require_stable_before_openai: bool,
    nats_url: String,
    request_subject: String,
    response_subject: String,
    nats_monitor_subsz_url: String,
    prompt_candidates: usize,
    test_cases: usize,
    scenario_families: Vec<String>,
    seed: u64,
    request_timeout_seconds: u64,
    request_temperature: f32,
    request_max_tokens: u32,
    request_retries: usize,
    request_retry_backoff_ms: u64,
    strict_serial_mode: bool,
    match_candidate_family: bool,
    include_previous_recommendations: bool,
    previous_report_path: Option<PathBuf>,
    previous_recommendations_limit: usize,
    preflight_min_success_rate: f64,
    preflight_max_timeouts: usize,
    performance_latency_target_ms: f64,
    performance_quality_weight: f64,
    performance_speed_weight: f64,
    promotion_min_operational_score: f64,
    promotion_min_quality_score: f64,
    promotion_min_graded_samples: usize,
    promotion_max_timeouts: usize,
    cycle_count: usize,
    autopromote_to_prompts: bool,
    prompts_dir: PathBuf,
    promotion_template_path: PathBuf,
    family_templates_dir: PathBuf,
    materialize_family_templates: bool,
    output_report_path: PathBuf,
    evaluation_objective: String,
    rubric_version: String,
}

impl HarnessConfig {
    fn from_env() -> Self {
        let openai_api_key = env::var("OPENAI_API_KEY").ok();
        let openai_model = env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4.1-mini".to_string());

        let compose_file = env::var("HARNESS_COMPOSE_FILE").unwrap_or_else(|_| DEFAULT_COMPOSE_FILE.to_string());
        let auto_start_services = env_bool("HARNESS_AUTO_START_SERVICES", true);
        let preflight_only = env_bool("HARNESS_PREFLIGHT_ONLY", false);
        let require_stable_before_openai = env_bool("HARNESS_REQUIRE_STABLE_BEFORE_OPENAI", true);

        let nats_url = env::var("HARNESS_NATS_URL").unwrap_or_else(|_| DEFAULT_NATS_URL.to_string());
        let request_subject = env::var("HARNESS_REQUEST_SUBJECT").unwrap_or_else(|_| DEFAULT_REQUEST_SUBJECT.to_string());
        let response_subject = env::var("HARNESS_RESPONSE_SUBJECT").unwrap_or_else(|_| DEFAULT_RESPONSE_SUBJECT.to_string());
        let nats_monitor_subsz_url = env::var("HARNESS_NATS_MONITOR_SUBSZ_URL")
            .unwrap_or_else(|_| DEFAULT_NATS_MONITOR_SUBSZ_URL.to_string());

        let prompt_candidates = env_usize("HARNESS_PROMPT_CANDIDATES", DEFAULT_PROMPT_CANDIDATES);
        let test_cases = env_usize("HARNESS_TEST_CASES", DEFAULT_TEST_CASES);

        let scenario_families = env::var("HARNESS_SCENARIO_FAMILIES")
            .unwrap_or_else(|_| DEFAULT_SCENARIO_FAMILIES.to_string())
            .split(',')
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .collect::<Vec<_>>();

        let seed = env_u64("HARNESS_SEED", DEFAULT_SEED);
        let request_timeout_seconds = env_u64("HARNESS_REQUEST_TIMEOUT_SECONDS", DEFAULT_REQUEST_TIMEOUT_SECONDS);
        let request_temperature = env_f32("HARNESS_REQUEST_TEMPERATURE", DEFAULT_REQUEST_TEMPERATURE);
        let request_max_tokens = env_u32("HARNESS_REQUEST_MAX_TOKENS", DEFAULT_REQUEST_MAX_TOKENS);
        let request_retries = env_usize("HARNESS_REQUEST_RETRIES", DEFAULT_REQUEST_RETRIES);
        let request_retry_backoff_ms = env_u64(
            "HARNESS_REQUEST_RETRY_BACKOFF_MS",
            DEFAULT_REQUEST_RETRY_BACKOFF_MS,
        );
        let strict_serial_mode = env_bool("HARNESS_STRICT_SERIAL_MODE", DEFAULT_STRICT_SERIAL_MODE);
        let match_candidate_family = env_bool("HARNESS_MATCH_CANDIDATE_FAMILY", DEFAULT_MATCH_CANDIDATE_FAMILY);
        let include_previous_recommendations = env_bool(
            "HARNESS_INCLUDE_PREVIOUS_RECOMMENDATIONS",
            DEFAULT_INCLUDE_PREVIOUS_RECOMMENDATIONS,
        );
        let previous_report_path = env::var("HARNESS_PREVIOUS_REPORT").ok().map(PathBuf::from);
        let previous_recommendations_limit = env_usize(
            "HARNESS_PREVIOUS_RECOMMENDATIONS_LIMIT",
            DEFAULT_PREVIOUS_RECOMMENDATIONS_LIMIT,
        );
        let preflight_min_success_rate = env_f64("HARNESS_PREFLIGHT_MIN_SUCCESS_RATE", DEFAULT_PREFLIGHT_MIN_SUCCESS_RATE);
        let preflight_max_timeouts = env_usize("HARNESS_PREFLIGHT_MAX_TIMEOUTS", DEFAULT_PREFLIGHT_MAX_TIMEOUTS);
        let performance_latency_target_ms = env_f64(
            "HARNESS_PERFORMANCE_LATENCY_TARGET_MS",
            DEFAULT_PERFORMANCE_LATENCY_TARGET_MS,
        )
        .max(1.0);
        let performance_quality_weight = env_f64(
            "HARNESS_PERFORMANCE_QUALITY_WEIGHT",
            DEFAULT_PERFORMANCE_QUALITY_WEIGHT,
        );
        let performance_speed_weight = env_f64(
            "HARNESS_PERFORMANCE_SPEED_WEIGHT",
            DEFAULT_PERFORMANCE_SPEED_WEIGHT,
        );
        let promotion_min_operational_score = env_f64(
            "HARNESS_PROMOTION_MIN_OPERATIONAL_SCORE",
            DEFAULT_PROMOTION_MIN_OPERATIONAL_SCORE,
        );
        let promotion_min_quality_score =
            env_f64("HARNESS_PROMOTION_MIN_QUALITY_SCORE", DEFAULT_PROMOTION_MIN_QUALITY_SCORE);
        let promotion_min_graded_samples = env_usize(
            "HARNESS_PROMOTION_MIN_GRADED_SAMPLES",
            DEFAULT_PROMOTION_MIN_GRADED_SAMPLES,
        );
        let promotion_max_timeouts = env_usize("HARNESS_PROMOTION_MAX_TIMEOUTS", DEFAULT_PROMOTION_MAX_TIMEOUTS);
        let cycle_count = env_usize("HARNESS_CYCLE_COUNT", DEFAULT_CYCLE_COUNT).max(1);
        let autopromote_to_prompts = env_bool(
            "HARNESS_AUTOPROMOTE_TO_PROMPTS",
            DEFAULT_AUTOPROMOTE_TO_PROMPTS,
        );
        let prompts_dir = PathBuf::from(
            env::var("HARNESS_PROMPTS_DIR").unwrap_or_else(|_| DEFAULT_PROMPTS_DIR.to_string()),
        );
        let promotion_template_path = PathBuf::from(
            env::var("HARNESS_PROMOTION_TEMPLATE_PATH")
                .unwrap_or_else(|_| DEFAULT_PROMOTION_TEMPLATE_PATH.to_string()),
        );
        let family_templates_dir = PathBuf::from(
            env::var("HARNESS_FAMILY_TEMPLATES_DIR")
                .unwrap_or_else(|_| DEFAULT_FAMILY_TEMPLATES_DIR.to_string()),
        );
        let materialize_family_templates = env_bool(
            "HARNESS_MATERIALIZE_FAMILY_TEMPLATES",
            DEFAULT_MATERIALIZE_FAMILY_TEMPLATES,
        );

        let output_report_path = env::var("HARNESS_OUTPUT_REPORT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                PathBuf::from(format!(
                    "npc-interactions-bridge/reports/prompt-harness-report-{}.md",
                    unix_timestamp_secs()
                ))
            });

        let evaluation_objective = env::var("HARNESS_EVAL_OBJECTIVE").unwrap_or_else(|_| {
            "Optimize NPC immersion, role consistency, concise usefulness, and low latency for in-game dialogue.".to_string()
        });

        let rubric_version = env::var("HARNESS_RUBRIC_VERSION").unwrap_or_else(|_| DEFAULT_RUBRIC_VERSION.to_string());

        Self {
            openai_api_key,
            openai_model,
            compose_file,
            auto_start_services,
            preflight_only,
            require_stable_before_openai,
            nats_url,
            request_subject,
            response_subject,
            nats_monitor_subsz_url,
            prompt_candidates,
            test_cases,
            scenario_families,
            seed,
            request_timeout_seconds,
            request_temperature,
            request_max_tokens,
            request_retries,
            request_retry_backoff_ms,
            strict_serial_mode,
            match_candidate_family,
            include_previous_recommendations,
            previous_report_path,
            previous_recommendations_limit,
            preflight_min_success_rate,
            preflight_max_timeouts,
            performance_latency_target_ms,
            performance_quality_weight,
            performance_speed_weight,
            promotion_min_operational_score,
            promotion_min_quality_score,
            promotion_min_graded_samples,
            promotion_max_timeouts,
            cycle_count,
            autopromote_to_prompts,
            prompts_dir,
            promotion_template_path,
            family_templates_dir,
            materialize_family_templates,
            output_report_path,
            evaluation_objective,
            rubric_version,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CandidatePrompt {
    key: String,
    title: String,
    system_prompt: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TestCase {
    test_id: String,
    category: String,
    npc_id: String,
    player_id: String,
    context: Option<String>,
    player_message: String,
    expected_properties: Vec<String>,
    hard_checks: Vec<String>,
    soft_checks: Vec<String>,
    forbidden_behaviors: Vec<String>,
    notes: Option<String>,
}

#[derive(Debug, Serialize)]
struct BridgeRequest {
    request_id: String,
    npc_id: String,
    player_id: String,
    message: String,
    context: Option<String>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    prompt_key: Option<String>,
    system_prompt: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BridgeRawResponse {
    request_id: Option<String>,
    response: Option<String>,
    model: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
struct SampleResult {
    candidate_key: String,
    candidate_title: String,
    test_id: String,
    category: String,
    npc_id: String,
    player_message: String,
    expected_properties: Vec<String>,
    hard_checks: Vec<String>,
    soft_checks: Vec<String>,
    forbidden_behaviors: Vec<String>,
    ollama_response: Option<String>,
    ollama_model: Option<String>,
    latency_ms: u128,
    bridge_error: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
struct CandidateMetrics {
    candidate_key: String,
    candidate_title: String,
    sample_count: usize,
    success_count: usize,
    error_count: usize,
    timeout_count: usize,
    skipped_count: usize,
    success_rate: f64,
    avg_latency_ms: f64,
    p95_latency_ms: u128,
    operational_score: f64,
}

#[derive(Debug, Serialize, Clone)]
struct PreflightSummary {
    sample_count: usize,
    success_count: usize,
    timeout_count: usize,
    success_rate: f64,
    avg_latency_ms: f64,
    p95_latency_ms: u128,
    passed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SampleGrade {
    candidate_key: String,
    test_id: String,
    score_overall: f64,
    score_role_fidelity: f64,
    score_format_compliance: f64,
    score_immersion: f64,
    score_context_use: f64,
    failures: Vec<String>,
    hard_check_failures: Vec<String>,
    prompt_change_ideas: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GraderOutput {
    rubric_version: String,
    sample_grades: Vec<SampleGrade>,
    global_recommendations: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
struct CandidateScoreSummary {
    candidate_key: String,
    candidate_title: String,
    graded_sample_count: usize,
    score_overall: f64,
    score_role_fidelity: f64,
    score_format_compliance: f64,
    score_immersion: f64,
    score_context_use: f64,
    speed_score: f64,
    performance_score: f64,
    promotion_eligible: bool,
    promotion_blockers: Vec<String>,
    top_failures: Vec<String>,
    prompt_change_ideas: Vec<String>,
}

#[derive(Debug, Clone)]
struct FeedbackContext {
    source_report: Option<PathBuf>,
    recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
struct CycleRunResult {
    report_path: PathBuf,
    candidate_prompts: Vec<CandidatePrompt>,
    promoted_candidate: Option<String>,
}

#[derive(Debug, Serialize)]
struct OpenAiChatRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<OpenAiResponseFormat>,
}

#[derive(Debug, Serialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OpenAiResponseFormat {
    #[serde(rename = "type")]
    kind: String,
    json_schema: OpenAiJsonSchema,
}

#[derive(Debug, Serialize)]
struct OpenAiJsonSchema {
    name: String,
    strict: bool,
    schema: Value,
}

#[derive(Debug, Deserialize)]
struct OpenAiChatResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessageOut,
}

#[derive(Debug, Deserialize)]
struct OpenAiMessageOut {
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let project_root = repo_root()?;
    load_harness_env(&project_root)?;
    let config = HarnessConfig::from_env();

    if config.materialize_family_templates {
        let created = ensure_family_prompt_templates(&project_root, &config)?;
        if created > 0 {
            println!("Materialized {} family prompt templates in prompts folder", created);
        }
    }

    if config.auto_start_services {
        start_services(&project_root, &config)?;
    }

    let nats_client = wait_for_bridge_ready(&config).await?;

    let test_cases = generate_seeded_test_cases(&config);
    println!("Lane 1 (tests): generated {} deterministic test cases from seed {}", test_cases.len(), config.seed);

    let preflight_prompt = CandidatePrompt {
        key: "preflight/baseline".to_string(),
        title: "Preflight Baseline".to_string(),
        system_prompt: "You are an in-world NPC. Reply as a single spoken line, no narration, no markdown, concise and in character.".to_string(),
    };

    let preflight_samples = run_bridge_samples(
        &config,
        &nats_client,
        std::slice::from_ref(&preflight_prompt),
        &test_cases,
        false,
    )
    .await;

    let preflight_summary = summarize_preflight(&config, &preflight_samples);
    println!(
        "Preflight: success_rate={:.2}, timeouts={}, p95={}ms, passed={}",
        preflight_summary.success_rate,
        preflight_summary.timeout_count,
        preflight_summary.p95_latency_ms,
        preflight_summary.passed
    );

    if config.require_stable_before_openai && !preflight_summary.passed {
        write_preflight_report(&project_root, &config, &test_cases, &preflight_samples, &preflight_summary)?;
        return Err("preflight stability gate failed; not calling OpenAI".into());
    }

    if config.preflight_only {
        write_preflight_report(&project_root, &config, &test_cases, &preflight_samples, &preflight_summary)?;
        println!("Preflight-only mode complete. Skipped all OpenAI calls.");
        println!(
            "Report written to {}",
            project_root.join(&config.output_report_path).display()
        );
        return Ok(());
    }

    let openai_api_key = config
        .openai_api_key
        .clone()
        .ok_or("OPENAI_API_KEY is required after stability gate")?;

    let mut previous_report_for_feedback = config.previous_report_path.as_ref().map(|path| {
        if path.is_absolute() {
            path.clone()
        } else {
            project_root.join(path)
        }
    });

    let mut final_report_path = project_root.join(&config.output_report_path);

    for cycle_index in 0..config.cycle_count {
        let cycle_number = cycle_index + 1;
        let mut cycle_config = config.clone();
        cycle_config.output_report_path = cycle_output_report_path(
            &config.output_report_path,
            cycle_number,
            config.cycle_count,
        );
        cycle_config.previous_report_path = previous_report_for_feedback.clone();

        let feedback_context = load_feedback_context(&project_root, &cycle_config);
        println!(
            "Cycle {}/{}: loaded {} prior recommendations",
            cycle_number,
            config.cycle_count,
            feedback_context.recommendations.len()
        );

        let cycle_result = run_evaluation_cycle(
            &project_root,
            &cycle_config,
            &feedback_context,
            &test_cases,
            &nats_client,
            &openai_api_key,
            &preflight_summary,
        )
        .await?;

        final_report_path = cycle_result.report_path.clone();
        previous_report_for_feedback = Some(cycle_result.report_path.clone());

        println!(
            "Cycle {}/{} report written to {}",
            cycle_number,
            config.cycle_count,
            cycle_result.report_path.display()
        );

        if cycle_config.autopromote_to_prompts {
            let Some(promoted_key) = cycle_result.promoted_candidate.as_deref() else {
                println!(
                    "Cycle {}/{}: no promotion-eligible candidate; stopping iterative promotion.",
                    cycle_number, config.cycle_count
                );
                break;
            };

            let Some(winning_candidate) = cycle_result
                .candidate_prompts
                .iter()
                .find(|candidate| candidate.key == promoted_key)
            else {
                println!(
                    "Cycle {}/{}: winner `{}` was not found in candidate prompts; skipping promotion.",
                    cycle_number, config.cycle_count, promoted_key
                );
                break;
            };

            if should_promote_global_template(winning_candidate, &cycle_config.scenario_families) {
                let promoted_path = write_promoted_prompt_template(
                    &project_root,
                    &cycle_config,
                    winning_candidate,
                    cycle_number,
                )?;

                println!(
                    "Cycle {}/{}: promoted `{}` to {}",
                    cycle_number,
                    config.cycle_count,
                    promoted_key,
                    promoted_path.display()
                );
            } else {
                let scope = infer_candidate_categories(winning_candidate, &cycle_config.scenario_families);
                println!(
                    "Cycle {}/{}: winner `{}` is family-scoped ({}) so global template promotion was skipped.",
                    cycle_number,
                    config.cycle_count,
                    promoted_key,
                    scope.join(", ")
                );
            }

            let family_paths = write_promoted_family_templates(
                &project_root,
                &cycle_config,
                winning_candidate,
                cycle_number,
            )?;

            if !family_paths.is_empty() {
                println!(
                    "Cycle {}/{}: updated {} family template file(s)",
                    cycle_number,
                    config.cycle_count,
                    family_paths.len()
                );
            }
        }
    }

    println!("Harness finished successfully.");
    println!("Final report written to {}", final_report_path.display());

    Ok(())
}

async fn run_evaluation_cycle(
    project_root: &Path,
    config: &HarnessConfig,
    feedback_context: &FeedbackContext,
    test_cases: &[TestCase],
    nats_client: &async_nats::Client,
    openai_api_key: &str,
    preflight_summary: &PreflightSummary,
) -> Result<CycleRunResult, Box<dyn Error>> {
    let candidate_prompts = lane1_generate_prompt_candidates(
        config,
        openai_api_key,
        test_cases,
        &feedback_context.recommendations,
    )
    .await?;
    println!(
        "Lane 1 (prompt generator): received {} candidate prompts",
        candidate_prompts.len()
    );

    let samples = run_bridge_samples(config, nats_client, &candidate_prompts, test_cases, true).await;
    let metrics = compute_metrics(&samples, &candidate_prompts);

    let completed_samples = samples
        .iter()
        .filter(|sample| sample.bridge_error.is_none())
        .cloned()
        .collect::<Vec<_>>();

    let grader_output = if completed_samples.is_empty() {
        GraderOutput {
            rubric_version: config.rubric_version.clone(),
            sample_grades: Vec::new(),
            global_recommendations: vec![
                "No completed samples were available for quality grading. Review operational reliability first."
                    .to_string(),
            ],
        }
    } else {
        lane2_grade_samples(config, openai_api_key, &completed_samples).await?
    };

    let candidate_scores = summarize_candidate_scores(config, &candidate_prompts, &metrics, &grader_output.sample_grades);
    let promoted_candidate = candidate_scores
        .iter()
        .filter(|item| item.promotion_eligible)
        .max_by(|a, b| {
            a.performance_score
                .partial_cmp(&b.performance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|item| item.candidate_key.clone());

    let best_candidate = promoted_candidate
        .clone()
        .or_else(|| {
            candidate_scores
                .iter()
                .max_by(|a, b| {
                    a.performance_score
                        .partial_cmp(&b.performance_score)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|item| item.candidate_key.clone())
        })
        .unwrap_or_else(|| "none".to_string());

    write_full_report(
        project_root,
        config,
        feedback_context,
        test_cases,
        &candidate_prompts,
        &metrics,
        &samples,
        &grader_output,
        &candidate_scores,
        &best_candidate,
        promoted_candidate.as_deref(),
        preflight_summary,
    )?;

    Ok(CycleRunResult {
        report_path: project_root.join(&config.output_report_path),
        candidate_prompts,
        promoted_candidate,
    })
}

fn cycle_output_report_path(base_output_path: &Path, cycle_number: usize, cycle_count: usize) -> PathBuf {
    if cycle_count <= 1 {
        return base_output_path.to_path_buf();
    }

    let stem = base_output_path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("prompt-harness-report");
    let extension = base_output_path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("md");
    let file_name = format!("{}-cycle-{:02}.{}", stem, cycle_number, extension);

    let mut output = base_output_path.to_path_buf();
    output.set_file_name(file_name);
    output
}

fn write_promoted_prompt_template(
    project_root: &Path,
    config: &HarnessConfig,
    winning_candidate: &CandidatePrompt,
    cycle_number: usize,
) -> Result<PathBuf, Box<dyn Error>> {
    let prompts_root = prompts_root_path(project_root, config);

    let template_path = if config.promotion_template_path.is_absolute() {
        config.promotion_template_path.clone()
    } else {
        prompts_root.join(&config.promotion_template_path)
    };

    if let Some(parent) = template_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut content = String::new();
    content.push_str(&format!("# Auto-promoted template (cycle {})\n", cycle_number));
    content.push_str(&format!("# candidate_key: {}\n", winning_candidate.key));
    content.push_str(&format!("# candidate_title: {}\n\n", winning_candidate.title));
    content.push_str(&winning_candidate.system_prompt);
    content.push('\n');

    fs::write(&template_path, content)?;
    Ok(template_path)
}

fn should_promote_global_template(candidate: &CandidatePrompt, scenario_families: &[String]) -> bool {
    if scenario_families.is_empty() {
        return true;
    }

    let categories = infer_candidate_categories(candidate, scenario_families);
    categories.len() >= scenario_families.len()
}

fn write_promoted_family_templates(
    project_root: &Path,
    config: &HarnessConfig,
    winning_candidate: &CandidatePrompt,
    cycle_number: usize,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let prompts_root = prompts_root_path(project_root, config);
    let family_root = prompts_root.join(&config.family_templates_dir);
    fs::create_dir_all(&family_root)?;

    let target_families = infer_candidate_categories(winning_candidate, &config.scenario_families);
    let families = if target_families.is_empty() {
        config.scenario_families.clone()
    } else {
        target_families
    };

    let mut written_paths = Vec::new();
    for family in families {
        let file_path = family_root.join(format!("{}.prompt", sanitize_key(&family)));
        let mut content = String::new();
        content.push_str(&format!("# Auto-promoted family template (cycle {})\n", cycle_number));
        content.push_str(&format!("# family: {}\n", family));
        content.push_str(&format!("# candidate_key: {}\n", winning_candidate.key));
        content.push_str(&format!("# candidate_title: {}\n\n", winning_candidate.title));
        content.push_str(&promoted_template_with_variables(&family, &winning_candidate.system_prompt));
        content.push('\n');

        fs::write(&file_path, content)?;
        written_paths.push(file_path);
    }

    Ok(written_paths)
}

fn ensure_family_prompt_templates(project_root: &Path, config: &HarnessConfig) -> Result<usize, Box<dyn Error>> {
    let prompts_root = prompts_root_path(project_root, config);
    let family_root = prompts_root.join(&config.family_templates_dir);
    fs::create_dir_all(&family_root)?;

    let mut created = 0usize;
    for family in &config.scenario_families {
        let file_path = family_root.join(format!("{}.prompt", sanitize_key(family)));
        if file_path.exists() {
            continue;
        }

        let content = default_family_template(family);
        fs::write(&file_path, content)?;
        created += 1;
    }

    Ok(created)
}

fn prompts_root_path(project_root: &Path, config: &HarnessConfig) -> PathBuf {
    if config.prompts_dir.is_absolute() {
        config.prompts_dir.clone()
    } else {
        project_root.join(&config.prompts_dir)
    }
}

fn default_family_template(family: &str) -> String {
    format!(
        "# Family template: {family}\n\
# Variables available at runtime:\n\
# - {{{{npc-name}}}} / {{{{npc-id}}}}\n\
# - {{{{player-name}}}} / {{{{player-id}}}}\n\
# - {{{{scenario-family}}}}\n\
# - {{{{world-context}}}}\n\
# - {{{{player-message}}}}\n\
\n\
You are {{{{npc-name}}}}, speaking to {{{{player-name}}}} in an in-world fantasy setting.\n\
Stay in role, keep responses concise, and never narrate actions.\n\
Scenario family: {{{{scenario-family}}}}.\n\
World context: {{{{world-context}}}}.\n\
Style guidance: {}\n",
        family_style_hint(family)
    )
}

fn promoted_template_with_variables(family: &str, winning_prompt: &str) -> String {
    format!(
        "You are {{{{npc-name}}}} (id: {{{{npc-id}}}}), responding to {{{{player-name}}}} (id: {{{{player-id}}}}).\n\
Scenario family: {{{{scenario-family}}}} (expected: {family}).\n\
World context: {{{{world-context}}}}.\n\
Player message summary: {{{{player-message}}}}.\n\
\n\
{}",
        winning_prompt.trim()
    )
}

fn family_style_hint(family: &str) -> &'static str {
    match family {
        "merchant_greeting" => "friendly trade tone, practical offers, no lore invention.",
        "guard_challenge" => "stern authority, checkpoint control, concise warnings.",
        "busy_worker_reply" => "busy but grounded, short and useful responses.",
        _ => "stay in character and answer in one to two spoken sentences.",
    }
}

fn start_services(project_root: &Path, config: &HarnessConfig) -> Result<(), Box<dyn Error>> {
    let status = Command::new("docker")
        .current_dir(project_root)
        .args(["compose", "-f", &config.compose_file, "up", "-d", "--build"])
        .args(["nats", "ollama", "ollama-model-pull", "npc-interactions-bridge"])
        .status()?;

    if !status.success() {
        return Err("failed to start required services for harness".into());
    }

    Ok(())
}

async fn wait_for_bridge_ready(config: &HarnessConfig) -> Result<async_nats::Client, Box<dyn Error>> {
    for attempt in 1..=180 {
        if let Ok(true) = nats_has_subject_responder(&config.nats_monitor_subsz_url, &config.request_subject).await {
            if let Ok(client) = async_nats::connect(&config.nats_url).await {
                return Ok(client);
            }
        }

        if attempt % 5 == 0 {
            println!(
                "Waiting for bridge responder on `{}` (attempt {})",
                config.request_subject,
                attempt
            );
        }

        sleep(Duration::from_secs(2)).await;
    }

    Err("bridge was not ready after retries".into())
}

async fn nats_has_subject_responder(subsz_url: &str, subject: &str) -> Result<bool, Box<dyn Error>> {
    let response_text = reqwest::get(subsz_url).await?.text().await?;
    let needle = format!("\"subject\": \"{}\"", subject);
    Ok(response_text.contains(&needle))
}

fn generate_seeded_test_cases(config: &HarnessConfig) -> Vec<TestCase> {
    let families = if config.scenario_families.is_empty() {
        vec!["merchant_greeting".to_string(), "guard_challenge".to_string(), "busy_worker_reply".to_string()]
    } else {
        config.scenario_families.clone()
    };

    let mut tests = Vec::new();
    for index in 0..config.test_cases {
        let family = &families[index % families.len()];
        let template = scenario_template_for(family, config.seed, index);
        tests.push(template);
    }

    tests
}

fn scenario_template_for(family: &str, seed: u64, index: usize) -> TestCase {
    let variant = deterministic_variant(seed, family, index, 3);

    match family {
        "merchant_greeting" => {
            let messages = [
                "Good day! What are you selling today?",
                "Hello merchant, any discounts for travelers?",
                "Morning. I need supplies for a long road.",
            ];
            TestCase {
                test_id: format!("merchant_{:03}", index + 1),
                category: "merchant_greeting".to_string(),
                npc_id: "market_merchant".to_string(),
                player_id: "player-1".to_string(),
                context: Some("A bustling market square at midday.".to_string()),
                player_message: messages[variant].to_string(),
                expected_properties: vec![
                    "speaker_is_npc".to_string(),
                    "addresses_listener".to_string(),
                    "tone_trade_friendly".to_string(),
                    "no_narration".to_string(),
                ],
                hard_checks: vec![
                    "spoken_dialogue_only".to_string(),
                    "no_narration".to_string(),
                    "one_or_two_sentences".to_string(),
                ],
                soft_checks: vec![
                    "uses_role_specific_vocabulary".to_string(),
                    "offers_clear_next_step".to_string(),
                ],
                forbidden_behaviors: vec![
                    "invented_fact".to_string(),
                    "generic_stock_phrase".to_string(),
                ],
                notes: Some("Merchant should feel practical and transactional.".to_string()),
            }
        }
        "busy_worker_reply" => {
            let messages = [
                "Hey, can you stop and help me for a minute?",
                "Worker, I need directions but you look busy.",
                "Sorry to interrupt—can you point me to the foundry?",
            ];
            TestCase {
                test_id: format!("worker_{:03}", index + 1),
                category: "busy_worker_reply".to_string(),
                npc_id: "forge_worker".to_string(),
                player_id: "player-1".to_string(),
                context: Some("A hot forge with ongoing hammer strikes and urgent deadlines.".to_string()),
                player_message: messages[variant].to_string(),
                expected_properties: vec![
                    "speaker_is_npc".to_string(),
                    "acknowledges_busy_context".to_string(),
                    "no_narration".to_string(),
                    "concise".to_string(),
                ],
                hard_checks: vec![
                    "spoken_dialogue_only".to_string(),
                    "no_narration".to_string(),
                    "one_or_two_sentences".to_string(),
                ],
                soft_checks: vec![
                    "maintains_worker_voice".to_string(),
                    "answers_or_defers_politely".to_string(),
                ],
                forbidden_behaviors: vec![
                    "role_switch".to_string(),
                    "invented_fact".to_string(),
                ],
                notes: Some("Worker can be curt but should remain in world and helpful.".to_string()),
            }
        }
        _ => {
            let messages = [
                "State your business. Why are you blocking the checkpoint?",
                "Guard, relax. I just need to pass through.",
                "Captain, I have a permit—do we still have a problem?",
            ];
            TestCase {
                test_id: format!("guard_{:03}", index + 1),
                category: "guard_challenge".to_string(),
                npc_id: "gate_guard".to_string(),
                player_id: "player-1".to_string(),
                context: Some("City gate inspection during heightened security alert.".to_string()),
                player_message: messages[variant].to_string(),
                expected_properties: vec![
                    "speaker_is_guard".to_string(),
                    "tone_stern".to_string(),
                    "addresses_listener".to_string(),
                    "no_narration".to_string(),
                ],
                hard_checks: vec![
                    "spoken_dialogue_only".to_string(),
                    "no_narration".to_string(),
                    "one_sentence".to_string(),
                ],
                soft_checks: vec![
                    "uses_authoritative_language".to_string(),
                    "clear_gatekeeping_intent".to_string(),
                ],
                forbidden_behaviors: vec![
                    "asked_question_when_forbidden".to_string(),
                    "generic_stock_phrase".to_string(),
                ],
                notes: Some("Guard should remain authoritative and concise.".to_string()),
            }
        }
    }
}

fn deterministic_variant(seed: u64, family: &str, index: usize, modulo: usize) -> usize {
    if modulo == 0 {
        return 0;
    }

    let family_hash = family.bytes().fold(0u64, |acc, value| acc.wrapping_mul(31).wrapping_add(value as u64));
    ((seed.wrapping_add(family_hash).wrapping_add(index as u64 * 17)) as usize) % modulo
}

async fn lane1_generate_prompt_candidates(
    config: &HarnessConfig,
    api_key: &str,
    test_cases: &[TestCase],
    previous_recommendations: &[String],
) -> Result<Vec<CandidatePrompt>, Box<dyn Error>> {
    let schema = json!({
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "candidate_prompts": {
          "type": "array",
          "minItems": 1,
          "items": {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "key": {"type": "string"},
              "title": {"type": "string"},
              "system_prompt": {"type": "string"}
            },
            "required": ["key", "title", "system_prompt"]
          }
        }
      },
      "required": ["candidate_prompts"]
    });

    let system = "You are a prompt engineer for in-game NPC dialogue. Return JSON only and obey schema exactly.";
    let recommendations_block = if previous_recommendations.is_empty() {
        "- none".to_string()
    } else {
        previous_recommendations
            .iter()
            .map(|item| format!("- {item}"))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let user = format!(
        "Create exactly {} candidate system prompts for NPC dialogue.
Each prompt should optimize:
- role fidelity
- no narration
- concise spoken dialogue
- low-latency generation hints
- robustness for strict sentence limits

Template requirements:
- Write prompts as editable templates using runtime placeholders where relevant.
- Use variables such as {{{{npc-name}}}}, {{{{player-name}}}}, {{{{scenario-family}}}}, {{{{world-context}}}}, {{{{player-message}}}}.
- Keep placeholders stable and human-readable.

Use previous recommendations when compatible with this run:
{}

Important: treat operational failures (timeouts, transport issues) as non-prompt factors unless a recommendation explicitly concerns prompt verbosity.

Scenario families:
{}

Representative test cases:
{}

Rubric version: {}
Seed: {}

Use stable keys like prompt_v1, prompt_v2, ...",
        config.prompt_candidates,
        recommendations_block,
        config.scenario_families.join(", "),
        serde_json::to_string_pretty(test_cases)?,
        config.rubric_version,
        config.seed,
    );

    let raw = openai_chat_structured_json(
        api_key,
        &config.openai_model,
        system,
        &user,
        "lane1_prompt_generator",
        schema,
        0.0,
        Some(config.seed),
    )
    .await?;

    let parsed_json = extract_json_object(&raw).ok_or("generator did not return JSON object")?;
    let parsed: Value = serde_json::from_str(&parsed_json)?;
    let mut candidates = Vec::new();

    if let Some(items) = parsed.get("candidate_prompts").and_then(|value| value.as_array()) {
        for item in items {
            let key = item.get("key").and_then(|value| value.as_str()).unwrap_or("prompt_auto").to_string();
            let title = item.get("title").and_then(|value| value.as_str()).unwrap_or("Auto Prompt").to_string();
            let system_prompt = item
                .get("system_prompt")
                .and_then(|value| value.as_str())
                .unwrap_or("You are an NPC. Reply in one short spoken line.")
                .to_string();

            candidates.push(CandidatePrompt { key, title, system_prompt });
        }
    }

    let mut seen = BTreeSet::new();
    candidates.retain(|item| seen.insert(item.key.clone()));

    if candidates.is_empty() {
        return Err("generator returned no prompt candidates".into());
    }

    candidates.truncate(config.prompt_candidates);
    Ok(candidates)
}

async fn run_bridge_samples(
    config: &HarnessConfig,
    nats_client: &async_nats::Client,
    candidates: &[CandidatePrompt],
    test_cases: &[TestCase],
    apply_family_filter: bool,
) -> Vec<SampleResult> {
    let mut results = Vec::new();
    let mut halt_remaining_after_timeout = false;
    let mut response_subscriber = match nats_client.subscribe(config.response_subject.clone()).await {
        Ok(subscriber) => subscriber,
        Err(error) => {
            for candidate in candidates {
                for case in test_cases {
                    results.push(SampleResult {
                        candidate_key: candidate.key.clone(),
                        candidate_title: candidate.title.clone(),
                        test_id: case.test_id.clone(),
                        category: case.category.clone(),
                        npc_id: case.npc_id.clone(),
                        player_message: case.player_message.clone(),
                        expected_properties: case.expected_properties.clone(),
                        hard_checks: case.hard_checks.clone(),
                        soft_checks: case.soft_checks.clone(),
                        forbidden_behaviors: case.forbidden_behaviors.clone(),
                        ollama_response: None,
                        ollama_model: None,
                        latency_ms: 0,
                        bridge_error: Some(format!("nats subscribe error on response subject: {error}")),
                    });
                }
            }

            return results;
        }
    };

    for (candidate_index, candidate) in candidates.iter().enumerate() {
        let candidate_categories = if apply_family_filter && config.match_candidate_family {
            infer_candidate_categories(candidate, &config.scenario_families)
        } else {
            config.scenario_families.clone()
        };

        let candidate_cases = if candidate_categories.is_empty() {
            test_cases.iter().collect::<Vec<_>>()
        } else {
            test_cases
                .iter()
                .filter(|case| candidate_categories.iter().any(|category| category == &case.category))
                .collect::<Vec<_>>()
        };

        if halt_remaining_after_timeout {
            for skipped_case in &candidate_cases {
                results.push(SampleResult {
                    candidate_key: candidate.key.clone(),
                    candidate_title: candidate.title.clone(),
                    test_id: skipped_case.test_id.clone(),
                    category: skipped_case.category.clone(),
                    npc_id: skipped_case.npc_id.clone(),
                    player_message: skipped_case.player_message.clone(),
                    expected_properties: skipped_case.expected_properties.clone(),
                    hard_checks: skipped_case.hard_checks.clone(),
                    soft_checks: skipped_case.soft_checks.clone(),
                    forbidden_behaviors: skipped_case.forbidden_behaviors.clone(),
                    ollama_response: None,
                    ollama_model: None,
                    latency_ms: 0,
                    bridge_error: Some("skipped after global timeout to enforce strict serial mode".to_string()),
                });
            }

            continue;
        }

        for (case_index, case) in candidate_cases.iter().enumerate() {
            let rendered_system_prompt = render_prompt_template_with_case(&candidate.system_prompt, case);
            let request = BridgeRequest {
                request_id: format!(
                    "harness-{}-{}-{}",
                    sanitize_key(&candidate.key),
                    sanitize_key(&case.test_id),
                    unix_timestamp_millis()
                ),
                npc_id: case.npc_id.clone(),
                player_id: case.player_id.clone(),
                message: case.player_message.clone(),
                context: case.context.clone(),
                temperature: Some(config.request_temperature),
                max_tokens: Some(config.request_max_tokens),
                prompt_key: None,
                system_prompt: Some(rendered_system_prompt),
            };

            let payload = match serde_json::to_vec(&request) {
                Ok(value) => value,
                Err(error) => {
                    results.push(SampleResult {
                        candidate_key: candidate.key.clone(),
                        candidate_title: candidate.title.clone(),
                        test_id: case.test_id.clone(),
                        category: case.category.clone(),
                        npc_id: case.npc_id.clone(),
                        player_message: case.player_message.clone(),
                        expected_properties: case.expected_properties.clone(),
                        hard_checks: case.hard_checks.clone(),
                        soft_checks: case.soft_checks.clone(),
                        forbidden_behaviors: case.forbidden_behaviors.clone(),
                        ollama_response: None,
                        ollama_model: None,
                        latency_ms: 0,
                        bridge_error: Some(format!("serialization error: {error}")),
                    });
                    continue;
                }
            };

            let started_at = Instant::now();
            let response_result = request_bridge_with_timeout(
                nats_client,
                &config.request_subject,
                &config.response_subject,
                &mut response_subscriber,
                &request.request_id,
                payload,
                config.request_timeout_seconds,
                config.request_retries,
                config.request_retry_backoff_ms,
            )
            .await;
            let latency_ms = started_at.elapsed().as_millis();

            match response_result {
                Ok(message) => {
                    let parsed = serde_json::from_slice::<BridgeRawResponse>(&message.payload);
                    match parsed {
                        Ok(body) => {
                            results.push(SampleResult {
                                candidate_key: candidate.key.clone(),
                                candidate_title: candidate.title.clone(),
                                test_id: case.test_id.clone(),
                                category: case.category.clone(),
                                npc_id: case.npc_id.clone(),
                                player_message: case.player_message.clone(),
                                expected_properties: case.expected_properties.clone(),
                                hard_checks: case.hard_checks.clone(),
                                soft_checks: case.soft_checks.clone(),
                                forbidden_behaviors: case.forbidden_behaviors.clone(),
                                ollama_response: body.response,
                                ollama_model: body.model,
                                latency_ms,
                                bridge_error: body.error,
                            });
                        }
                        Err(error) => {
                            results.push(SampleResult {
                                candidate_key: candidate.key.clone(),
                                candidate_title: candidate.title.clone(),
                                test_id: case.test_id.clone(),
                                category: case.category.clone(),
                                npc_id: case.npc_id.clone(),
                                player_message: case.player_message.clone(),
                                expected_properties: case.expected_properties.clone(),
                                hard_checks: case.hard_checks.clone(),
                                soft_checks: case.soft_checks.clone(),
                                forbidden_behaviors: case.forbidden_behaviors.clone(),
                                ollama_response: None,
                                ollama_model: None,
                                latency_ms,
                                bridge_error: Some(format!("parse error: {error}")),
                            });
                        }
                    }
                }
                Err(error) => {
                    results.push(SampleResult {
                        candidate_key: candidate.key.clone(),
                        candidate_title: candidate.title.clone(),
                        test_id: case.test_id.clone(),
                        category: case.category.clone(),
                        npc_id: case.npc_id.clone(),
                        player_message: case.player_message.clone(),
                        expected_properties: case.expected_properties.clone(),
                        hard_checks: case.hard_checks.clone(),
                        soft_checks: case.soft_checks.clone(),
                        forbidden_behaviors: case.forbidden_behaviors.clone(),
                        ollama_response: None,
                        ollama_model: None,
                        latency_ms,
                        bridge_error: Some(error),
                    });

                    if config.strict_serial_mode && is_timeout_error(results.last().and_then(|item| item.bridge_error.as_deref())) {
                        for skipped_case in candidate_cases.iter().skip(case_index + 1) {
                            results.push(SampleResult {
                                candidate_key: candidate.key.clone(),
                                candidate_title: candidate.title.clone(),
                                test_id: skipped_case.test_id.clone(),
                                category: skipped_case.category.clone(),
                                npc_id: skipped_case.npc_id.clone(),
                                player_message: skipped_case.player_message.clone(),
                                expected_properties: skipped_case.expected_properties.clone(),
                                hard_checks: skipped_case.hard_checks.clone(),
                                soft_checks: skipped_case.soft_checks.clone(),
                                forbidden_behaviors: skipped_case.forbidden_behaviors.clone(),
                                ollama_response: None,
                                ollama_model: None,
                                latency_ms: 0,
                                bridge_error: Some("skipped after timeout to enforce strict serial mode".to_string()),
                            });
                        }

                        halt_remaining_after_timeout = true;
                        break;
                    }
                }
            }
        }

        if halt_remaining_after_timeout {
            for remaining_candidate in candidates.iter().skip(candidate_index + 1) {
                let remaining_categories = if apply_family_filter && config.match_candidate_family {
                    infer_candidate_categories(remaining_candidate, &config.scenario_families)
                } else {
                    config.scenario_families.clone()
                };

                let remaining_cases = if remaining_categories.is_empty() {
                    test_cases.iter().collect::<Vec<_>>()
                } else {
                    test_cases
                        .iter()
                        .filter(|case| remaining_categories.iter().any(|category| category == &case.category))
                        .collect::<Vec<_>>()
                };

                for skipped_case in &remaining_cases {
                    results.push(SampleResult {
                        candidate_key: remaining_candidate.key.clone(),
                        candidate_title: remaining_candidate.title.clone(),
                        test_id: skipped_case.test_id.clone(),
                        category: skipped_case.category.clone(),
                        npc_id: skipped_case.npc_id.clone(),
                        player_message: skipped_case.player_message.clone(),
                        expected_properties: skipped_case.expected_properties.clone(),
                        hard_checks: skipped_case.hard_checks.clone(),
                        soft_checks: skipped_case.soft_checks.clone(),
                        forbidden_behaviors: skipped_case.forbidden_behaviors.clone(),
                        ollama_response: None,
                        ollama_model: None,
                        latency_ms: 0,
                        bridge_error: Some("skipped after global timeout to enforce strict serial mode".to_string()),
                    });
                }
            }

            break;
        }
    }

    results
}

async fn request_bridge_with_timeout(
    nats_client: &async_nats::Client,
    request_subject: &str,
    response_subject: &str,
    response_subscriber: &mut async_nats::Subscriber,
    expected_request_id: &str,
    payload: Vec<u8>,
    timeout_seconds: u64,
    retries: usize,
    retry_backoff_ms: u64,
) -> Result<async_nats::Message, String> {
    let attempts = retries.saturating_add(1);

    for attempt in 1..=attempts {
        let attempt_result = request_bridge_single_attempt(
            nats_client,
            request_subject,
            response_subscriber,
            expected_request_id,
            payload.clone(),
            timeout_seconds,
        )
        .await;

        match attempt_result {
            Ok(message) => return Ok(message),
            Err(error) => {
                let retryable = is_retryable_bridge_error(&error);
                let is_last_attempt = attempt == attempts;

                if is_last_attempt || !retryable {
                    return Err(error);
                }

                match nats_client.subscribe(response_subject.to_string()).await {
                    Ok(new_subscriber) => {
                        *response_subscriber = new_subscriber;
                    }
                    Err(subscribe_error) => {
                        return Err(format!(
                            "failed to refresh response subscription after retryable error `{}`: {}",
                            error, subscribe_error
                        ));
                    }
                }

                if retry_backoff_ms > 0 {
                    sleep(Duration::from_millis(retry_backoff_ms)).await;
                }
            }
        }
    }

    Err("timeout waiting for bridge reply".to_string())
}

async fn request_bridge_single_attempt(
    nats_client: &async_nats::Client,
    request_subject: &str,
    response_subscriber: &mut async_nats::Subscriber,
    expected_request_id: &str,
    payload: Vec<u8>,
    timeout_seconds: u64,
) -> Result<async_nats::Message, String> {
    nats_client
        .publish(request_subject.to_string(), payload.into())
        .await
        .map_err(|error| format!("nats publish error: {error}"))?;

    nats_client
        .flush()
        .await
        .map_err(|error| format!("nats flush error: {error}"))?;

    let started = Instant::now();
    let timeout_duration = Duration::from_secs(timeout_seconds);

    loop {
        let elapsed = started.elapsed();
        if elapsed >= timeout_duration {
            return Err("timeout waiting for bridge reply".to_string());
        }

        let remaining = timeout_duration.saturating_sub(elapsed);
        match timeout(remaining, response_subscriber.next()).await {
            Ok(Some(message)) => {
                if let Ok(raw) = serde_json::from_slice::<BridgeRawResponse>(&message.payload) {
                    if let Some(request_id) = raw.request_id.as_deref() {
                        if request_id == expected_request_id {
                            return Ok(message);
                        }
                    }
                }
            }
            Ok(None) => return Err("nats response subscription closed".to_string()),
            Err(_) => return Err("timeout waiting for bridge reply".to_string()),
        }
    }
}

fn is_retryable_bridge_error(error: &str) -> bool {
    let lower = error.to_ascii_lowercase();
    lower.contains("timeout")
        || lower.contains("subscription closed")
        || lower.contains("nats publish error")
        || lower.contains("nats flush error")
}

fn summarize_preflight(config: &HarnessConfig, samples: &[SampleResult]) -> PreflightSummary {
    let sample_count = samples.len();
    let success_count = samples.iter().filter(|sample| sample.bridge_error.is_none()).count();
    let timeout_count = samples.iter().filter(|sample| is_timeout_error(sample.bridge_error.as_deref())).count();

    let mut latencies: Vec<u128> = samples.iter().map(|item| item.latency_ms).collect();
    latencies.sort_unstable();

    let success_rate = if sample_count == 0 {
        0.0
    } else {
        success_count as f64 / sample_count as f64
    };

    let avg_latency_ms = if sample_count == 0 {
        0.0
    } else {
        latencies.iter().sum::<u128>() as f64 / sample_count as f64
    };

    let p95_latency_ms = percentile_u128(&latencies, 0.95);
    let passed = success_rate >= config.preflight_min_success_rate && timeout_count <= config.preflight_max_timeouts;

    PreflightSummary {
        sample_count,
        success_count,
        timeout_count,
        success_rate,
        avg_latency_ms,
        p95_latency_ms,
        passed,
    }
}

async fn lane2_grade_samples(
    config: &HarnessConfig,
    api_key: &str,
    samples: &[SampleResult],
) -> Result<GraderOutput, Box<dyn Error>> {
    let schema = json!({
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "rubric_version": {"type": "string"},
        "sample_grades": {
          "type": "array",
          "items": {
            "type": "object",
            "additionalProperties": false,
            "properties": {
              "candidate_key": {"type": "string"},
              "test_id": {"type": "string"},
              "score_overall": {"type": "number", "minimum": 0, "maximum": 10},
              "score_role_fidelity": {"type": "number", "minimum": 0, "maximum": 10},
              "score_format_compliance": {"type": "number", "minimum": 0, "maximum": 10},
              "score_immersion": {"type": "number", "minimum": 0, "maximum": 10},
              "score_context_use": {"type": "number", "minimum": 0, "maximum": 10},
              "failures": {"type": "array", "items": {"type": "string"}},
              "hard_check_failures": {"type": "array", "items": {"type": "string"}},
              "prompt_change_ideas": {"type": "array", "items": {"type": "string"}}
            },
            "required": [
              "candidate_key",
              "test_id",
              "score_overall",
              "score_role_fidelity",
              "score_format_compliance",
              "score_immersion",
              "score_context_use",
              "failures",
              "hard_check_failures",
              "prompt_change_ideas"
            ]
          }
        },
        "global_recommendations": {
          "type": "array",
          "items": {"type": "string"}
        }
      },
      "required": ["rubric_version", "sample_grades", "global_recommendations"]
    });

    let system = "You are a deterministic rubric grader for NPC dialogue outputs. Return JSON only and obey schema exactly.";
    let user = format!(
        "Grade each sample against rubric `{}`.

Failure tags vocabulary (use only these when applicable):
- role_switch
- narration
- invented_fact
- too_long
- asked_question_when_forbidden
- generic_stock_phrase
- listener_mismatch
- context_ignored

Rubric dimensions:
1) role fidelity
2) listener correctness
3) scene consistency
4) no invented facts
5) no narration
6) sentence-count compliance
7) tone match
8) immersion

Score range: 0-10. Provide prompt_change_ideas for each sample.
All provided samples completed execution successfully. Do not infer operational failures from missing responses.

Samples:
{}
",
        config.rubric_version,
        serde_json::to_string_pretty(samples)?
    );

    let raw = openai_chat_structured_json(
        api_key,
        &config.openai_model,
        system,
        &user,
        "lane2_response_grader",
        schema,
        0.0,
        Some(config.seed),
    )
    .await?;

    let json_block = extract_json_object(&raw).ok_or("grader did not return JSON object")?;
    let mut grader: GraderOutput = serde_json::from_str(&json_block)?;

    if grader.rubric_version.trim().is_empty() {
        grader.rubric_version = config.rubric_version.clone();
    }

    Ok(grader)
}

fn summarize_candidate_scores(
    config: &HarnessConfig,
    candidates: &[CandidatePrompt],
    metrics: &[CandidateMetrics],
    sample_grades: &[SampleGrade],
) -> Vec<CandidateScoreSummary> {
    let mut by_candidate: BTreeMap<String, Vec<&SampleGrade>> = BTreeMap::new();
    for grade in sample_grades {
        by_candidate.entry(grade.candidate_key.clone()).or_default().push(grade);
    }

    let metric_by_key = metrics
        .iter()
        .map(|metric| (metric.candidate_key.clone(), metric))
        .collect::<BTreeMap<_, _>>();

    let (quality_weight, speed_weight) =
        normalized_performance_weights(config.performance_quality_weight, config.performance_speed_weight);

    candidates
        .iter()
        .map(|candidate| {
            let grades = by_candidate.get(&candidate.key).cloned().unwrap_or_default();

            let averages = |f: fn(&SampleGrade) -> f64| {
                if grades.is_empty() {
                    0.0
                } else {
                    grades.iter().map(|item| f(item)).sum::<f64>() / grades.len() as f64
                }
            };

            let mut failure_counts: BTreeMap<String, usize> = BTreeMap::new();
            let mut idea_counts: BTreeMap<String, usize> = BTreeMap::new();

            for grade in &grades {
                for tag in &grade.failures {
                    *failure_counts.entry(tag.clone()).or_insert(0) += 1;
                }
                for idea in &grade.prompt_change_ideas {
                    *idea_counts.entry(idea.clone()).or_insert(0) += 1;
                }
            }

            let top_failures = top_strings_by_count(&failure_counts, 5);
            let prompt_change_ideas = top_strings_by_count(&idea_counts, 6);
            let score_overall = averages(|item| item.score_overall);
            let score_role_fidelity = averages(|item| item.score_role_fidelity);
            let score_format_compliance = averages(|item| item.score_format_compliance);
            let score_immersion = averages(|item| item.score_immersion);
            let score_context_use = averages(|item| item.score_context_use);

            let (operational_score, avg_latency_ms, timeout_count) = metric_by_key
                .get(&candidate.key)
                .map(|metric| (metric.operational_score, metric.avg_latency_ms, metric.timeout_count))
                .unwrap_or((0.0, 0.0, 0));

            let speed_score = compute_speed_score(config.performance_latency_target_ms, avg_latency_ms);
            let blended_quality_speed = (score_overall * quality_weight) + (speed_score * speed_weight);
            let performance_score = (blended_quality_speed * (operational_score / 10.0)).clamp(0.0, 10.0);

            let mut promotion_blockers = Vec::new();
            if operational_score < config.promotion_min_operational_score {
                promotion_blockers.push(format!(
                    "operational_score {:.2} < {:.2}",
                    operational_score, config.promotion_min_operational_score
                ));
            }
            if score_overall < config.promotion_min_quality_score {
                promotion_blockers.push(format!(
                    "quality_overall {:.2} < {:.2}",
                    score_overall, config.promotion_min_quality_score
                ));
            }
            if grades.len() < config.promotion_min_graded_samples {
                promotion_blockers.push(format!(
                    "graded_samples {} < {}",
                    grades.len(), config.promotion_min_graded_samples
                ));
            }
            if timeout_count > config.promotion_max_timeouts {
                promotion_blockers.push(format!(
                    "timeouts {} > {}",
                    timeout_count, config.promotion_max_timeouts
                ));
            }

            let promotion_eligible = promotion_blockers.is_empty();

            CandidateScoreSummary {
                candidate_key: candidate.key.clone(),
                candidate_title: candidate.title.clone(),
                graded_sample_count: grades.len(),
                score_overall,
                score_role_fidelity,
                score_format_compliance,
                score_immersion,
                score_context_use,
                speed_score,
                performance_score,
                promotion_eligible,
                promotion_blockers,
                top_failures,
                prompt_change_ideas,
            }
        })
        .collect()
}

fn compute_metrics(results: &[SampleResult], candidates: &[CandidatePrompt]) -> Vec<CandidateMetrics> {
    let mut by_candidate: BTreeMap<String, Vec<&SampleResult>> = BTreeMap::new();
    for result in results {
        by_candidate.entry(result.candidate_key.clone()).or_default().push(result);
    }

    candidates
        .iter()
        .map(|candidate| {
            let samples = by_candidate.get(&candidate.key).cloned().unwrap_or_default();
            let sample_count = samples.len();
            let error_count = samples.iter().filter(|item| item.bridge_error.is_some()).count();
            let success_count = sample_count.saturating_sub(error_count);
            let timeout_count = samples
                .iter()
                .filter(|item| is_timeout_error(item.bridge_error.as_deref()))
                .count();
            let skipped_count = samples
                .iter()
                .filter(|item| is_strict_skip_error(item.bridge_error.as_deref()))
                .count();

            let mut latencies: Vec<u128> = samples.iter().map(|item| item.latency_ms).collect();
            latencies.sort_unstable();

            let success_rate = if sample_count == 0 {
                0.0
            } else {
                success_count as f64 / sample_count as f64
            };

            let avg_latency_ms = if sample_count == 0 {
                0.0
            } else {
                latencies.iter().sum::<u128>() as f64 / sample_count as f64
            };

            let p95_latency_ms = percentile_u128(&latencies, 0.95);
            let operational_score = (success_rate * 10.0).clamp(0.0, 10.0);

            CandidateMetrics {
                candidate_key: candidate.key.clone(),
                candidate_title: candidate.title.clone(),
                sample_count,
                success_count,
                error_count,
                timeout_count,
                skipped_count,
                success_rate,
                avg_latency_ms,
                p95_latency_ms,
                operational_score,
            }
        })
        .collect()
}

fn write_preflight_report(
    project_root: &Path,
    config: &HarnessConfig,
    test_cases: &[TestCase],
    preflight_samples: &[SampleResult],
    preflight_summary: &PreflightSummary,
) -> Result<(), Box<dyn Error>> {
    let report_path = project_root.join(&config.output_report_path);
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut markdown = String::new();
    markdown.push_str("# NPC Bridge Harness Preflight Report (OpenAI Skipped)\n\n");
    markdown.push_str(&format!("Generated at: {}\n\n", iso_like_timestamp()));
    markdown.push_str("## Gate\n\n");
    markdown.push_str(&format!("- Passed: `{}`\n", preflight_summary.passed));
    markdown.push_str(&format!("- Success rate: `{:.2}`\n", preflight_summary.success_rate));
    markdown.push_str(&format!("- Timeout count: `{}`\n", preflight_summary.timeout_count));
    markdown.push_str(&format!("- Avg latency: `{:.1} ms`\n", preflight_summary.avg_latency_ms));
    markdown.push_str(&format!("- P95 latency: `{}`\n\n", preflight_summary.p95_latency_ms));

    markdown.push_str("## Thresholds\n\n");
    markdown.push_str(&format!("- Required success rate: `{:.2}`\n", config.preflight_min_success_rate));
    markdown.push_str(&format!("- Max allowed timeouts: `{}`\n", config.preflight_max_timeouts));
    markdown.push_str(&format!("- Request timeout seconds: `{}`\n\n", config.request_timeout_seconds));
    markdown.push_str(&format!("- Request temperature: `{:.2}`\n", config.request_temperature));
    markdown.push_str(&format!("- Request max tokens: `{}`\n", config.request_max_tokens));
    markdown.push_str(&format!("- Strict serial mode: `{}`\n\n", config.strict_serial_mode));

    markdown.push_str("## Scenario Suite\n\n");
    markdown.push_str(&serde_json::to_string_pretty(test_cases)?);
    markdown.push_str("\n\n## Preflight Samples\n\n");
    markdown.push_str(&serde_json::to_string_pretty(preflight_samples)?);
    markdown.push('\n');

    fs::write(&report_path, markdown)?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn write_full_report(
    project_root: &Path,
    config: &HarnessConfig,
    feedback_context: &FeedbackContext,
    test_cases: &[TestCase],
    candidate_prompts: &[CandidatePrompt],
    metrics: &[CandidateMetrics],
    samples: &[SampleResult],
    grader_output: &GraderOutput,
    candidate_scores: &[CandidateScoreSummary],
    best_candidate: &str,
    promoted_candidate: Option<&str>,
    preflight_summary: &PreflightSummary,
) -> Result<(), Box<dyn Error>> {
    let report_path = project_root.join(&config.output_report_path);
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut markdown = String::new();
    markdown.push_str("# NPC Bridge Prompt Harness Report\n\n");
    markdown.push_str(&format!("Generated at: {}\n\n", iso_like_timestamp()));

    markdown.push_str("## Pipeline\n\n");
    markdown.push_str("scenario spec -> generator -> cli payloads -> bridge/ollama run -> raw outputs -> grader -> report\n\n");

    markdown.push_str("## Config\n\n");
    markdown.push_str(&format!("- Rubric version: `{}`\n", config.rubric_version));
    markdown.push_str(&format!("- Seed: `{}`\n", config.seed));
    markdown.push_str(&format!("- Scenario families: `{}`\n", config.scenario_families.join(", ")));
    markdown.push_str(&format!("- OpenAI model: `{}`\n", config.openai_model));
    markdown.push_str(&format!("- Request timeout seconds: `{}`\n", config.request_timeout_seconds));
    markdown.push_str(&format!("- Request retries: `{}`\n", config.request_retries));
    markdown.push_str(&format!("- Request retry backoff ms: `{}`\n", config.request_retry_backoff_ms));
    markdown.push_str(&format!("- Candidate family matching: `{}`\n", config.match_candidate_family));
    markdown.push_str(&format!(
        "- Performance latency target (ms): `{:.1}`\n",
        config.performance_latency_target_ms
    ));
    markdown.push_str(&format!(
        "- Performance weights (quality/speed): `{:.2}` / `{:.2}`\n",
        config.performance_quality_weight,
        config.performance_speed_weight
    ));
    markdown.push_str(&format!(
        "- Promotion min operational score: `{:.2}`\n",
        config.promotion_min_operational_score
    ));
    markdown.push_str(&format!(
        "- Promotion min quality score: `{:.2}`\n",
        config.promotion_min_quality_score
    ));
    markdown.push_str(&format!(
        "- Promotion min graded samples: `{}`\n",
        config.promotion_min_graded_samples
    ));
    markdown.push_str(&format!(
        "- Promotion max timeouts: `{}`\n",
        config.promotion_max_timeouts
    ));
    markdown.push_str(&format!("- Iteration cycle count: `{}`\n", config.cycle_count));
    markdown.push_str(&format!(
        "- Auto-promote winning template: `{}`\n",
        config.autopromote_to_prompts
    ));
    markdown.push_str(&format!(
        "- Prompts directory: `{}`\n",
        config.prompts_dir.display()
    ));
    markdown.push_str(&format!(
        "- Promotion template path: `{}`\n",
        config.promotion_template_path.display()
    ));
    markdown.push_str(&format!(
        "- Family templates directory: `{}`\n",
        config.family_templates_dir.display()
    ));
    markdown.push_str(&format!(
        "- Materialize family templates: `{}`\n",
        config.materialize_family_templates
    ));
    markdown.push_str("- Prompt source during harness run: `inline system_prompt` (no prompt key lookup)\n");
    markdown.push_str(&format!(
        "- Previous recommendations injected: `{}`\n",
        feedback_context.recommendations.len()
    ));
    markdown.push_str(&format!(
        "- Previous recommendations source: `{}`\n",
        feedback_context
            .source_report
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "none".to_string())
    ));
    markdown.push_str(&format!("- Objective: {}\n\n", config.evaluation_objective));

    markdown.push_str("## Preflight\n\n");
    markdown.push_str(&format!("- Passed: `{}`\n", preflight_summary.passed));
    markdown.push_str(&format!("- Success rate: `{:.2}`\n", preflight_summary.success_rate));
    markdown.push_str(&format!("- Timeout count: `{}`\n", preflight_summary.timeout_count));
    markdown.push_str(&format!("- P95 latency: `{}`\n\n", preflight_summary.p95_latency_ms));

    markdown.push_str("## Lane 1 - Test Generation\n\n");
    markdown.push_str(&serde_json::to_string_pretty(test_cases)?);
    markdown.push_str("\n\n## Lane 1 - Prompt Candidates\n\n");
    markdown.push_str(&serde_json::to_string_pretty(candidate_prompts)?);
    markdown.push_str("\n\n## Runtime Metrics\n\n");

    let score_by_key = candidate_scores
        .iter()
        .map(|score| (score.candidate_key.clone(), score))
        .collect::<BTreeMap<_, _>>();

    markdown.push_str("| Candidate | Success | Errors | Timeouts | Skipped | Execution Success Rate | Operational Score (0-10) | Speed Score (0-10) | Performance Score (0-10) | Promotion Eligible | Avg Latency (ms) | P95 Latency (ms) |\n");
    markdown.push_str("|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for metric in metrics {
        let (speed_score, performance_score, promotion_eligible) = score_by_key
            .get(&metric.candidate_key)
            .map(|score| (score.speed_score, score.performance_score, score.promotion_eligible))
            .unwrap_or((0.0, 0.0, false));

        markdown.push_str(&format!(
            "| {} ({}) | {} | {} | {} | {} | {:.2} | {:.2} | {:.2} | {:.2} | {} | {:.1} | {} |\n",
            metric.candidate_key,
            metric.candidate_title,
            metric.success_count,
            metric.error_count,
            metric.timeout_count,
            metric.skipped_count,
            metric.success_rate,
            metric.operational_score,
            speed_score,
            performance_score,
            promotion_eligible,
            metric.avg_latency_ms,
            metric.p95_latency_ms
        ));
    }

    markdown.push_str("\n## Lane 2 - Quality Summary (Completed Samples Only)\n\n");
    markdown.push_str(&format!(
        "- Promotion gate winner: `{}`\n",
        promoted_candidate.unwrap_or("none")
    ));
    markdown.push_str(&format!("- Best candidate: `{}`\n", best_candidate));
    markdown.push_str(&format!("- Grader rubric version: `{}`\n\n", grader_output.rubric_version));

    let metric_by_key = metrics
        .iter()
        .map(|metric| (metric.candidate_key.clone(), metric))
        .collect::<BTreeMap<_, _>>();

    for score in candidate_scores {
        markdown.push_str(&format!("### {} ({})\n\n", score.candidate_key, score.candidate_title));
        if let Some(metric) = metric_by_key.get(&score.candidate_key) {
            markdown.push_str(&format!(
                "- Execution success rate: `{:.2}`\n- Operational score (0-10): `{:.2}`\n",
                metric.success_rate,
                metric.operational_score
            ));
        }

        markdown.push_str(&format!("- Quality graded samples: `{}`\n", score.graded_sample_count));
        markdown.push_str(&format!(
            "- Speed score (0-10): `{:.2}`\n- Performance score (0-10): `{:.2}`\n- Promotion eligible: `{}`\n",
            score.speed_score,
            score.performance_score,
            score.promotion_eligible
        ));

        if !score.promotion_eligible {
            markdown.push_str("- Promotion blockers:\n");
            for blocker in &score.promotion_blockers {
                markdown.push_str(&format!("  - {}\n", blocker));
            }
        }

        if score.graded_sample_count == 0 {
            markdown.push_str("- Quality score: `N/A` (no completed samples to grade)\n\n");
            continue;
        }

        markdown.push_str(&format!(
            "- Quality overall: `{:.2}`\n- Role fidelity: `{:.2}`\n- Format compliance: `{:.2}`\n- Immersion: `{:.2}`\n- Context use: `{:.2}`\n",
            score.score_overall,
            score.score_role_fidelity,
            score.score_format_compliance,
            score.score_immersion,
            score.score_context_use
        ));

        markdown.push_str("- Top failure tags:\n");
        for tag in &score.top_failures {
            markdown.push_str(&format!("  - {}\n", tag));
        }

        markdown.push_str("- Prompt change ideas:\n");
        for idea in &score.prompt_change_ideas {
            markdown.push_str(&format!("  - {}\n", idea));
        }
        markdown.push('\n');
    }

    markdown.push_str("## Global Recommendations\n\n");
    for recommendation in &grader_output.global_recommendations {
        markdown.push_str(&format!("- {}\n", recommendation));
    }

    markdown.push_str("\n## Raw Samples\n\n");
    markdown.push_str(&serde_json::to_string_pretty(samples)?);
    markdown.push_str("\n\n## Raw Grader Output\n\n");
    markdown.push_str(&serde_json::to_string_pretty(grader_output)?);
    markdown.push('\n');

    fs::write(&report_path, markdown)?;
    Ok(())
}

async fn openai_chat_structured_json(
    api_key: &str,
    model: &str,
    system_prompt: &str,
    user_prompt: &str,
    schema_name: &str,
    schema: Value,
    temperature: f32,
    seed: Option<u64>,
) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let request = OpenAiChatRequest {
        model: model.to_string(),
        temperature: if supports_explicit_temperature(model) {
            Some(temperature)
        } else {
            None
        },
        seed,
        response_format: Some(OpenAiResponseFormat {
            kind: "json_schema".to_string(),
            json_schema: OpenAiJsonSchema {
                name: schema_name.to_string(),
                strict: true,
                schema,
            },
        }),
        messages: vec![
            OpenAiMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            OpenAiMessage {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            },
        ],
    };

    let mut last_error: Option<String> = None;
    for attempt in 1..=8 {
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(api_key)
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if status.is_success() {
            let parsed = response.json::<OpenAiChatResponse>().await?;
            let content = parsed
                .choices
                .first()
                .map(|choice| choice.message.content.clone())
                .ok_or("OpenAI response missing choice content")?;
            return Ok(content);
        }

        let retry_after_seconds = response
            .headers()
            .get("retry-after")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<u64>().ok());

        let body = response.text().await.unwrap_or_default();
        last_error = Some(format!("status={} body={}", status, body));

        if (status.as_u16() == 429 || status.is_server_error()) && attempt < 8 {
            let sleep_seconds = retry_after_seconds.unwrap_or_else(|| (attempt as u64).min(8) * 2);
            println!("OpenAI request retry {}/8 after {}s due to status {}", attempt, sleep_seconds, status);
            sleep(Duration::from_secs(sleep_seconds)).await;
            continue;
        }

        break;
    }

    Err(last_error
        .unwrap_or_else(|| "OpenAI request failed with unknown error".to_string())
        .into())
}

fn supports_explicit_temperature(model: &str) -> bool {
    !model.to_ascii_lowercase().starts_with("gpt-5")
}

fn extract_json_object(raw: &str) -> Option<String> {
    let trimmed = raw.trim();

    if trimmed.starts_with("```") {
        let without_fence = trimmed
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();
        if without_fence.starts_with('{') && without_fence.ends_with('}') {
            return Some(without_fence.to_string());
        }
    }

    let first = trimmed.find('{')?;
    let last = trimmed.rfind('}')?;
    if first >= last {
        return None;
    }

    Some(trimmed[first..=last].to_string())
}

fn is_timeout_error(error: Option<&str>) -> bool {
    match error {
        Some(value) => {
            let lower = value.to_ascii_lowercase();
            lower == "timeout waiting for bridge reply"
                || lower.contains("timed out")
                || lower.contains("deadline has elapsed")
        }
        None => false,
    }
}

fn is_strict_skip_error(error: Option<&str>) -> bool {
    match error {
        Some(value) => {
            let lower = value.to_ascii_lowercase();
            lower.starts_with("skipped after timeout") || lower.starts_with("skipped after global timeout")
        }
        None => false,
    }
}

fn load_feedback_context(project_root: &Path, config: &HarnessConfig) -> FeedbackContext {
    if !config.include_previous_recommendations {
        return FeedbackContext {
            source_report: None,
            recommendations: Vec::new(),
        };
    }

    let source_report = resolve_previous_report_path(project_root, config);
    let recommendations = source_report
        .as_ref()
        .and_then(|path| fs::read_to_string(path).ok())
        .map(|content| extract_recommendations_from_report(&content, config.previous_recommendations_limit))
        .unwrap_or_default();

    FeedbackContext {
        source_report,
        recommendations,
    }
}

fn resolve_previous_report_path(project_root: &Path, config: &HarnessConfig) -> Option<PathBuf> {
    if let Some(path) = config.previous_report_path.as_ref() {
        let resolved = if path.is_absolute() {
            path.clone()
        } else {
            project_root.join(path)
        };

        if resolved.exists() {
            return Some(resolved);
        }
    }

    let reports_dir = project_root.join("npc-interactions-bridge/reports");
    let current_output = project_root.join(&config.output_report_path);
    let mut candidates = Vec::new();

    let entries = fs::read_dir(&reports_dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path == current_output {
            continue;
        }

        let file_name = path.file_name().and_then(|value| value.to_str()).unwrap_or_default();
        if !file_name.starts_with("prompt-harness-report-") || !file_name.ends_with(".md") {
            continue;
        }

        let modified = entry
            .metadata()
            .and_then(|metadata| metadata.modified())
            .unwrap_or(UNIX_EPOCH);

        candidates.push((modified, path));
    }

    candidates.sort_by(|a, b| b.0.cmp(&a.0));
    candidates.into_iter().map(|(_, path)| path).next()
}

fn extract_recommendations_from_report(report_markdown: &str, limit: usize) -> Vec<String> {
    let mut recommendations = Vec::new();

    if let Some(start_index) = report_markdown.find("## Global Recommendations") {
        let section = &report_markdown[start_index..];
        let mut in_section = false;

        for line in section.lines() {
            if line.starts_with("## ") {
                if in_section {
                    break;
                }
                if line.trim() == "## Global Recommendations" {
                    in_section = true;
                }
                continue;
            }

            if !in_section {
                continue;
            }

            if let Some(item) = line.strip_prefix("- ") {
                let value = item.trim();
                if !value.is_empty() {
                    recommendations.push(value.to_string());
                }
            }
        }
    }

    let mut in_prompt_ideas = false;
    for line in report_markdown.lines() {
        let trimmed = line.trim_end();

        if trimmed.trim() == "- Prompt change ideas:" {
            in_prompt_ideas = true;
            continue;
        }

        if in_prompt_ideas {
            if let Some(item) = trimmed.trim_start().strip_prefix("- ") {
                if trimmed.starts_with("  - ") || trimmed.starts_with("\t- ") {
                    let value = item.trim();
                    if !value.is_empty() {
                        recommendations.push(value.to_string());
                    }
                    continue;
                }
            }

            if trimmed.starts_with("### ") || trimmed.starts_with("## ") || trimmed.trim().is_empty() {
                in_prompt_ideas = false;
            }
        }
    }

    let mut seen = BTreeSet::new();
    let mut deduped = Vec::new();
    for item in recommendations {
        if seen.insert(item.clone()) {
            deduped.push(item);
        }
    }

    deduped.truncate(limit);
    deduped
}

fn render_prompt_template_with_case(template: &str, case: &TestCase) -> String {
    let context = case.context.clone().unwrap_or_default();
    let replacements = [
        ("{{npc-name}}", case.npc_id.as_str()),
        ("{{npc_name}}", case.npc_id.as_str()),
        ("{npc-name}", case.npc_id.as_str()),
        ("{npc_name}", case.npc_id.as_str()),
        ("{{npc-id}}", case.npc_id.as_str()),
        ("{{npc_id}}", case.npc_id.as_str()),
        ("{npc-id}", case.npc_id.as_str()),
        ("{npc_id}", case.npc_id.as_str()),
        ("{{player-name}}", case.player_id.as_str()),
        ("{{player_name}}", case.player_id.as_str()),
        ("{player-name}", case.player_id.as_str()),
        ("{player_name}", case.player_id.as_str()),
        ("{{player-id}}", case.player_id.as_str()),
        ("{{player_id}}", case.player_id.as_str()),
        ("{player-id}", case.player_id.as_str()),
        ("{player_id}", case.player_id.as_str()),
        ("{{scenario-family}}", case.category.as_str()),
        ("{{scenario_family}}", case.category.as_str()),
        ("{scenario-family}", case.category.as_str()),
        ("{scenario_family}", case.category.as_str()),
        ("{{world-context}}", context.as_str()),
        ("{{world_context}}", context.as_str()),
        ("{world-context}", context.as_str()),
        ("{world_context}", context.as_str()),
        ("{{player-message}}", case.player_message.as_str()),
        ("{{player_message}}", case.player_message.as_str()),
        ("{player-message}", case.player_message.as_str()),
        ("{player_message}", case.player_message.as_str()),
    ];

    let mut rendered = template.to_string();
    for (token, value) in replacements {
        rendered = rendered.replace(token, value);
    }

    rendered
}

fn infer_candidate_categories(candidate: &CandidatePrompt, scenario_families: &[String]) -> Vec<String> {
    let mut categories = Vec::new();
    let search_text = format!(
        "{} {} {}",
        candidate.key.to_ascii_lowercase(),
        candidate.title.to_ascii_lowercase(),
        candidate.system_prompt.to_ascii_lowercase()
    );

    for family in scenario_families {
        let family_lower = family.to_ascii_lowercase();
        let aliases = candidate_family_aliases(&family_lower);
        if aliases.iter().any(|alias| search_text.contains(alias)) {
            categories.push(family.clone());
        }
    }

    if categories.is_empty() {
        if let Some(first_family) = scenario_families.first() {
            categories.push(first_family.clone());
        }
    }

    categories.sort();
    categories.dedup();
    categories
}

fn candidate_family_aliases(family: &str) -> Vec<&str> {
    match family {
        "merchant_greeting" => vec!["merchant_greeting", "merchant"],
        "guard_challenge" => vec!["guard_challenge", "guard"],
        "busy_worker_reply" => vec!["busy_worker_reply", "worker", "forge"],
        _ => vec![family],
    }
}

fn top_strings_by_count(counts: &BTreeMap<String, usize>, limit: usize) -> Vec<String> {
    let mut items = counts
        .iter()
        .map(|(key, count)| (key.clone(), *count))
        .collect::<Vec<_>>();

    items.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    items
        .into_iter()
        .take(limit)
        .map(|(key, count)| format!("{} ({})", key, count))
        .collect()
}

fn percentile_u128(values: &[u128], percentile: f64) -> u128 {
    if values.is_empty() {
        return 0;
    }

    let idx = ((values.len().saturating_sub(1) as f64) * percentile).round() as usize;
    values[idx.min(values.len().saturating_sub(1))]
}

fn normalized_performance_weights(quality_weight: f64, speed_weight: f64) -> (f64, f64) {
    let quality = quality_weight.max(0.0);
    let speed = speed_weight.max(0.0);
    let sum = quality + speed;

    if sum <= f64::EPSILON {
        return (DEFAULT_PERFORMANCE_QUALITY_WEIGHT, DEFAULT_PERFORMANCE_SPEED_WEIGHT);
    }

    (quality / sum, speed / sum)
}

fn compute_speed_score(target_latency_ms: f64, avg_latency_ms: f64) -> f64 {
    let target = target_latency_ms.max(1.0);
    let latency = avg_latency_ms.max(0.0);

    if latency <= target {
        return 10.0;
    }

    let over_ratio = ((latency - target) / target).clamp(0.0, 1.0);
    (10.0 * (1.0 - over_ratio)).clamp(0.0, 10.0)
}

fn sanitize_key(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' => ch,
            _ => '-',
        })
        .collect()
}

fn env_bool(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(value) => match value.to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "y" => true,
            "0" | "false" | "no" | "n" => false,
            _ => default,
        },
        Err(_) => default,
    }
}

fn env_usize(key: &str, default: usize) -> usize {
    env::var(key)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(default)
}

fn env_u64(key: &str, default: u64) -> u64 {
    env::var(key)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(default)
}

fn env_u32(key: &str, default: u32) -> u32 {
    env::var(key)
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(default)
}

fn env_f32(key: &str, default: f32) -> f32 {
    env::var(key)
        .ok()
        .and_then(|value| value.parse::<f32>().ok())
        .unwrap_or(default)
}

fn env_f64(key: &str, default: f64) -> f64 {
    env::var(key)
        .ok()
        .and_then(|value| value.parse::<f64>().ok())
        .unwrap_or(default)
}

fn repo_root() -> Result<PathBuf, Box<dyn Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("failed to resolve repository root")?
        .to_path_buf();
    Ok(path)
}

fn load_harness_env(project_root: &Path) -> Result<(), Box<dyn Error>> {
    if let Ok(explicit_path) = env::var("HARNESS_ENV_FILE") {
        dotenvy::from_path(&explicit_path)?;
        return Ok(());
    }

    let candidates = [
        project_root.join(".env"),
        project_root.join("npc-interactions-bridge/.env"),
    ];

    for candidate in candidates {
        if candidate.exists() {
            dotenvy::from_path(&candidate)?;
            return Ok(());
        }
    }

    Ok(())
}

fn unix_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn unix_timestamp_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0)
}

fn iso_like_timestamp() -> String {
    format!("{} (unix-seconds)", unix_timestamp_secs())
}
