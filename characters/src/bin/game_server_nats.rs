/// characters-server binary
///
/// Internal NATS server handling game-server → character-service requests.
/// No JWT authentication — this endpoint is trusted (internal NATS network only).
///
/// Subjects handled:
///   characters.get        request: {"id":"<uuid>"}
///                         response: {"id":"…","name":"…","race":N,"gender":N,
///                                    "skin_color":N,"character_class":N,
///                                    "created_at":"…","user_email":"…"}
///                         or {"error":"…"} on failure
use idklol_common::config::env_config::EnvConfig;
use idklol_common::db;
use idklol_common::logging::logger_service::LoggerService;
use tracing::{error, info, info_span, warn, Instrument};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use futures::StreamExt;

use characters_core::services::character_admin_service::CharacterAdminService;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

// ─── wire format ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct GetCharacterRequest {
    id: String,
}

/// Flat response shape expected by UE `ServerCharacterLoaderSubsystem`.
/// Field names must match the `GetStringField` / `GetIntegerField` keys in C++.
#[derive(Debug, Serialize)]
struct CharacterResponse {
    id: String,
    name: String,
    /// DB id == UE enum value (e.g. race_id 1 == ECharacterRace::Human)
    race: i32,
    gender: i32,
    skin_color: i32,
    character_class: i32,
    /// RFC-3339 string
    created_at: String,
    /// Keycloak user email that owns this character.
    /// Used by ATPSCoreGameMode::PostLogin to verify character ownership against the JWT.
    user_email: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

// ─── handlers ────────────────────────────────────────────────────────────────

async fn handle_characters_get(
    message: async_nats::Message,
    character_service: Arc<CharacterAdminService>,
    client: async_nats::Client,
) {
    let subject = message.subject.to_string();
    let has_reply = message.reply.is_some();
    let payload_size = message.payload.len();

    async move {
        let reply_subject = match message.reply.clone() {
            Some(r) => r,
            None => {
                warn!("characters.get: no reply subject, ignoring");
                return;
            }
        };

        let response_bytes: Vec<u8> = match serde_json::from_slice::<GetCharacterRequest>(&message.payload) {
            Err(e) => {
                warn!("characters.get: invalid request payload: {}", e);
                serde_json::to_vec(&ErrorResponse { error: format!("invalid request: {}", e) }).unwrap()
            }
            Ok(req) => {
                let uuid = match uuid::Uuid::parse_str(&req.id) {
                    Ok(u) => u,
                    Err(e) => {
                        warn!(id = %req.id, "characters.get: invalid UUID: {}", e);
                        let bytes = serde_json::to_vec(&ErrorResponse { error: format!("invalid UUID: {}", e) }).unwrap();
                        if let Err(e) = client.publish(reply_subject, bytes.into()).await {
                            error!("characters.get: failed to send reply: {}", e);
                        }
                        return;
                    }
                };

                match character_service.get_character_by_id(uuid).await {
                    Ok(Some(ch)) => {
                        info!(id = %uuid, name = %ch.name, "characters.get: found");
                        let resp = CharacterResponse {
                            id: ch.id.to_string(),
                            name: ch.name.clone(),
                            race: ch.race_id,
                            gender: ch.gender_id,
                            skin_color: ch.skin_color_id,
                            character_class: ch.class_id,
                            created_at: ch.created_at.to_rfc3339(),
                            user_email: ch.user_email.clone(),
                        };
                        serde_json::to_vec(&resp).unwrap()
                    }
                    Ok(None) => {
                        warn!(id = %uuid, "characters.get: not found");
                        serde_json::to_vec(&ErrorResponse { error: "character not found".to_string() }).unwrap()
                    }
                    Err(e) => {
                        error!(id = %uuid, "characters.get: db error: {}", e);
                        serde_json::to_vec(&ErrorResponse { error: "internal error".to_string() }).unwrap()
                    }
                }
            }
        };

        if let Err(e) = client.publish(reply_subject, response_bytes.into()).await {
            error!("characters.get: failed to send reply: {}", e);
        }
    }
    .instrument(info_span!(
        "nats.characters.get",
        nats_subject = %subject,
        has_reply = has_reply,
        payload_size = payload_size
    ))
    .await;
}

// ─── entry point ─────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    EnvConfig::init_from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));
    let _logger_guard = LoggerService::init_from_env("idklol-characters-server")?;

    let database_url = EnvConfig::get_required("DATABASE_URL")?;
    info!("connecting to database");
    let pool = db::connect_pool(&database_url, 5).await?;

    info!("running database migrations");
    let schema_version = db::migrate_and_get_version(&pool, &MIGRATOR).await?;
    info!(?schema_version, "database migrations complete");

    let character_service = Arc::new(CharacterAdminService::with_pool(pool));

    let nats_url = EnvConfig::get_optional("NATS_URL")
        .ok()
        .flatten()
        .unwrap_or_else(|| "nats://nats:4222".to_string());

    info!(%nats_url, "connecting to NATS");
    let client = async_nats::connect(&nats_url).await?;
    info!("connected to NATS successfully");

    // Subscribe: characters.get
    let mut sub = client.subscribe("characters.get").await?;
    let svc = character_service.clone();
    let nc = client.clone();
    tokio::spawn(async move {
        info!("listening on characters.get");
        while let Some(msg) = sub.next().await {
            let svc = svc.clone();
            let nc = nc.clone();
            tokio::spawn(async move {
                handle_characters_get(msg, svc, nc).await;
            });
        }
    });

    info!("Characters game-server NATS service is running");
    tokio::signal::ctrl_c().await?;
    info!("shutting down");
    Ok(())
}
