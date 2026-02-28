use idklol_common::config::env_config::EnvConfig;
use idklol_common::db;
use idklol_common::logging::logger_service::LoggerService;
use idklol_common::auth::jwt::jwt_validator_service::JwtValidatorService;
use tracing::{info, error, warn};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use futures::StreamExt;

use characters_core::services::catalog_admin_service::CatalogAdminService;
use characters_core::services::character_admin_service::CharacterAdminService;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

#[derive(Debug, Deserialize)]
struct CreateItemRequest {
    name: String,
}

#[derive(Debug, Deserialize)]
struct UpdateItemRequest {
    id: i32,
    name: String,
}

#[derive(Debug, Deserialize)]
struct DeleteItemRequest {
    id: i32,
}

#[derive(Debug, Deserialize)]
struct SetCombinationRequest {
    race_id: i32,
    gender_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    skin_color_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    class_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct GetCharacterRequest {
    id: String, // UUID as string
}

#[derive(Debug, Deserialize)]
struct DeleteCharacterRequest {
    id: String, // UUID as string
}

#[derive(Debug, Deserialize)]
struct UpdateCharacterRequest {
    id: String, // UUID as string
    name: String,
    race_id: i32,
    gender_id: i32,
    skin_color_id: i32,
    class_id: i32,
}

#[derive(Debug, Serialize)]
struct SuccessResponse<T> {
    success: bool,
    data: T,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    success: bool,
    error: String,
}

// Helper to extract JWT from NATS message headers
fn extract_jwt_from_headers(headers: &async_nats::HeaderMap) -> Option<String> {
    headers.get("Authorization")
        .map(|v| v.as_str())
        .and_then(|auth_header| {
            if auth_header.starts_with("Bearer ") {
                Some(auth_header[7..].to_string())
            } else {
                None
            }
        })
}

async fn handle_catalog_admin_request(
    subject: String,
    message: async_nats::Message,
    catalog_admin_service: Arc<CatalogAdminService>,
    jwt_validator: Arc<JwtValidatorService>,
    client: async_nats::Client,
) {
    // Extract and validate JWT
    let jwt = match message.headers.as_ref().and_then(|h| extract_jwt_from_headers(h)) {
        Some(token) => token,
        None => {
            let response = ErrorResponse { success: false, error: "Missing Authorization header".to_string() };
            if let Some(reply) = message.reply {
                let _ = client.publish(reply, serde_json::to_vec(&response).unwrap().into()).await;
            }
            return;
        }
    };

    // Validate admin role
    let claims = match jwt_validator.validate_and_require_admin(&jwt).await {
        Ok(c) => c,
        Err(e) => {
            warn!("Unauthorized admin access attempt: {}", e);
            let response = ErrorResponse { success: false, error: e };
            if let Some(reply) = message.reply {
                let _ = client.publish(reply, serde_json::to_vec(&response).unwrap().into()).await;
            }
            return;
        }
    };

    info!(email = %claims.email, subject = %subject, "Processing admin catalog request");

    // Route to appropriate handler based on subject
    let response_bytes = match subject.as_str() {
        // Races
        "admin.catalog.races.list" => {
            match catalog_admin_service.list_races().await {
                Ok(races) => serde_json::to_vec(&SuccessResponse { success: true, data: races }).unwrap(),
                Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
            }
        }
        "admin.catalog.races.create" => {
            match serde_json::from_slice::<CreateItemRequest>(&message.payload) {
                Ok(req) => {
                    match catalog_admin_service.create_race(&req.name).await {
                        Ok(race) => serde_json::to_vec(&SuccessResponse { success: true, data: race }).unwrap(),
                        Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
                    }
                }
                Err(e) => {
                    let response = ErrorResponse { success: false, error: format!("Invalid request: {}", e) };
                    serde_json::to_vec(&response).unwrap()
                }
            }
        }
        "admin.catalog.races.update" => {
            match serde_json::from_slice::<UpdateItemRequest>(&message.payload) {
                Ok(req) => {
                    match catalog_admin_service.update_race(req.id, &req.name).await {
                        Ok(race) => serde_json::to_vec(&SuccessResponse { success: true, data: race }).unwrap(),
                        Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
                    }
                }
                Err(e) => {
                    let response = ErrorResponse { success: false, error: format!("Invalid request: {}", e) };
                    serde_json::to_vec(&response).unwrap()
                }
            }
        }
        "admin.catalog.races.delete" => {
            match serde_json::from_slice::<DeleteItemRequest>(&message.payload) {
                Ok(req) => {
                    match catalog_admin_service.delete_race(req.id).await {
                        Ok(_) => serde_json::to_vec(&SuccessResponse { success: true, data: () }).unwrap(),
                        Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
                    }
                }
                Err(e) => {
                    let response = ErrorResponse { success: false, error: format!("Invalid request: {}", e) };
                    serde_json::to_vec(&response).unwrap()
                }
            }
        }

        // Genders
        "admin.catalog.genders.list" => {
            match catalog_admin_service.list_genders().await {
                Ok(genders) => serde_json::to_vec(&SuccessResponse { success: true, data: genders }).unwrap(),
                Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
            }
        }
        "admin.catalog.genders.create" => {
            match serde_json::from_slice::<CreateItemRequest>(&message.payload) {
                Ok(req) => {
                    match catalog_admin_service.create_gender(&req.name).await {
                        Ok(gender) => serde_json::to_vec(&SuccessResponse { success: true, data: gender }).unwrap(),
                        Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
                    }
                }
                Err(e) => {
                    let response = ErrorResponse { success: false, error: format!("Invalid request: {}", e) };
                    serde_json::to_vec(&response).unwrap()
                }
            }
        }
        "admin.catalog.genders.update" => {
            match serde_json::from_slice::<UpdateItemRequest>(&message.payload) {
                Ok(req) => {
                    match catalog_admin_service.update_gender(req.id, &req.name).await {
                        Ok(gender) => serde_json::to_vec(&SuccessResponse { success: true, data: gender }).unwrap(),
                        Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
                    }
                }
                Err(e) => {
                    let response = ErrorResponse { success: false, error: format!("Invalid request: {}", e) };
                    serde_json::to_vec(&response).unwrap()
                }
            }
        }
        "admin.catalog.genders.delete" => {
            match serde_json::from_slice::<DeleteItemRequest>(&message.payload) {
                Ok(req) => {
                    match catalog_admin_service.delete_gender(req.id).await {
                        Ok(_) => serde_json::to_vec(&SuccessResponse { success: true, data: () }).unwrap(),
                        Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
                    }
                }
                Err(e) => {
                    let response = ErrorResponse { success: false, error: format!("Invalid request: {}", e) };
                    serde_json::to_vec(&response).unwrap()
                }
            }
        }

        // Skin Colors
        "admin.catalog.skincolors.list" => {
            match catalog_admin_service.list_skin_colors().await {
                Ok(skin_colors) => serde_json::to_vec(&SuccessResponse { success: true, data: skin_colors }).unwrap(),
                Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
            }
        }
        "admin.catalog.skincolors.create" => {
            match serde_json::from_slice::<CreateItemRequest>(&message.payload) {
                Ok(req) => {
                    match catalog_admin_service.create_skin_color(&req.name).await {
                        Ok(skin_color) => serde_json::to_vec(&SuccessResponse { success: true, data: skin_color }).unwrap(),
                        Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
                    }
                }
                Err(e) => {
                    let response = ErrorResponse { success: false, error: format!("Invalid request: {}", e) };
                    serde_json::to_vec(&response).unwrap()
                }
            }
        }
        "admin.catalog.skincolors.update" => {
            match serde_json::from_slice::<UpdateItemRequest>(&message.payload) {
                Ok(req) => {
                    match catalog_admin_service.update_skin_color(req.id, &req.name).await {
                        Ok(skin_color) => serde_json::to_vec(&SuccessResponse { success: true, data: skin_color }).unwrap(),
                        Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
                    }
                }
                Err(e) => {
                    let response = ErrorResponse { success: false, error: format!("Invalid request: {}", e) };
                    serde_json::to_vec(&response).unwrap()
                }
            }
        }
        "admin.catalog.skincolors.delete" => {
            match serde_json::from_slice::<DeleteItemRequest>(&message.payload) {
                Ok(req) => {
                    match catalog_admin_service.delete_skin_color(req.id).await {
                        Ok(_) => serde_json::to_vec(&SuccessResponse { success: true, data: () }).unwrap(),
                        Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
                    }
                }
                Err(e) => {
                    let response = ErrorResponse { success: false, error: format!("Invalid request: {}", e) };
                    serde_json::to_vec(&response).unwrap()
                }
            }
        }

        // Classes
        "admin.catalog.classes.list" => {
            match catalog_admin_service.list_classes().await {
                Ok(classes) => serde_json::to_vec(&SuccessResponse { success: true, data: classes }).unwrap(),
                Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
            }
        }
        "admin.catalog.classes.create" => {
            match serde_json::from_slice::<CreateItemRequest>(&message.payload) {
                Ok(req) => {
                    match catalog_admin_service.create_class(&req.name).await {
                        Ok(class) => serde_json::to_vec(&SuccessResponse { success: true, data: class }).unwrap(),
                        Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
                    }
                }
                Err(e) => {
                    let response = ErrorResponse { success: false, error: format!("Invalid request: {}", e) };
                    serde_json::to_vec(&response).unwrap()
                }
            }
        }
        "admin.catalog.classes.update" => {
            match serde_json::from_slice::<UpdateItemRequest>(&message.payload) {
                Ok(req) => {
                    match catalog_admin_service.update_class(req.id, &req.name).await {
                        Ok(class) => serde_json::to_vec(&SuccessResponse { success: true, data: class }).unwrap(),
                        Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
                    }
                }
                Err(e) => {
                    let response = ErrorResponse { success: false, error: format!("Invalid request: {}", e) };
                    serde_json::to_vec(&response).unwrap()
                }
            }
        }
        "admin.catalog.classes.delete" => {
            match serde_json::from_slice::<DeleteItemRequest>(&message.payload) {
                Ok(req) => {
                    match catalog_admin_service.delete_class(req.id).await {
                        Ok(_) => serde_json::to_vec(&SuccessResponse { success: true, data: () }).unwrap(),
                        Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
                    }
                }
                Err(e) => {
                    let response = ErrorResponse { success: false, error: format!("Invalid request: {}", e) };
                    serde_json::to_vec(&response).unwrap()
                }
            }
        }

        // Combinations
        "admin.catalog.combinations.race_gender.set" => {
            match serde_json::from_slice::<SetCombinationRequest>(&message.payload) {
                Ok(req) => {
                    match catalog_admin_service.set_race_gender_allowed(req.race_id, req.gender_id).await {
                        Ok(_) => serde_json::to_vec(&SuccessResponse { success: true, data: () }).unwrap(),
                        Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
                    }
                }
                Err(e) => {
                    let response = ErrorResponse { success: false, error: format!("Invalid request: {}", e) };
                    serde_json::to_vec(&response).unwrap()
                }
            }
        }
        "admin.catalog.combinations.race_gender.remove" => {
            match serde_json::from_slice::<SetCombinationRequest>(&message.payload) {
                Ok(req) => {
                    match catalog_admin_service.remove_race_gender_allowed(req.race_id, req.gender_id).await {
                        Ok(_) => serde_json::to_vec(&SuccessResponse { success: true, data: () }).unwrap(),
                        Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
                    }
                }
                Err(e) => {
                    let response = ErrorResponse { success: false, error: format!("Invalid request: {}", e) };
                    serde_json::to_vec(&response).unwrap()
                }
            }
        }

        _ => {
            let response = ErrorResponse { success: false, error: format!("Unknown subject: {}", subject) };
            serde_json::to_vec(&response).unwrap()
        }
    };

    // Send response if reply subject provided
    if let Some(reply) = message.reply {
        if let Err(e) = client.publish(reply, response_bytes.into()).await {
            error!("Failed to send NATS reply: {}", e);
        }
    }
}

async fn handle_character_admin_request(
    subject: String,
    message: async_nats::Message,
    character_admin_service: Arc<CharacterAdminService>,
    jwt_validator: Arc<JwtValidatorService>,
    client: async_nats::Client,
) {
    // Extract and validate JWT
    let jwt = match message.headers.as_ref().and_then(|h| extract_jwt_from_headers(h)) {
        Some(token) => token,
        None => {
            let response = ErrorResponse { success: false, error: "Missing Authorization header".to_string() };
            if let Some(reply) = message.reply {
                let _ = client.publish(reply, serde_json::to_vec(&response).unwrap().into()).await;
            }
            return;
        }
    };

    // Validate admin role
    let claims = match jwt_validator.validate_and_require_admin(&jwt).await {
        Ok(c) => c,
        Err(e) => {
            warn!("Unauthorized admin access attempt: {}", e);
            let response = ErrorResponse { success: false, error: e };
            if let Some(reply) = message.reply {
                let _ = client.publish(reply, serde_json::to_vec(&response).unwrap().into()).await;
            }
            return;
        }
    };

    info!(email = %claims.email, subject = %subject, "Processing admin character request");

    // Route to appropriate handler
    let response_bytes = match subject.as_str() {
        "admin.characters.list" => {
            match character_admin_service.list_all_characters().await {
                Ok(characters) => serde_json::to_vec(&SuccessResponse { success: true, data: characters }).unwrap(),
                Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
            }
        }
        "admin.characters.get" => {
            match serde_json::from_slice::<GetCharacterRequest>(&message.payload) {
                Ok(req) => {
                    let id = match uuid::Uuid::parse_str(&req.id) {
                        Ok(id) => id,
                        Err(e) => {
                            let response = ErrorResponse { success: false, error: format!("Invalid UUID: {}", e) };
                            if let Some(reply) = message.reply {
                                let _ = client.publish(reply, serde_json::to_vec(&response).unwrap().into()).await;
                            }
                            return;
                        }
                    };
                    match character_admin_service.get_character_by_id(id).await {
                        Ok(character) => serde_json::to_vec(&SuccessResponse { success: true, data: character }).unwrap(),
                        Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
                    }
                }
                Err(e) => {
                    let response = ErrorResponse { success: false, error: format!("Invalid request: {}", e) };
                    serde_json::to_vec(&response).unwrap()
                }
            }
        }
        "admin.characters.delete" => {
            match serde_json::from_slice::<DeleteCharacterRequest>(&message.payload) {
                Ok(req) => {
                    let id = match uuid::Uuid::parse_str(&req.id) {
                        Ok(id) => id,
                        Err(e) => {
                            let response = ErrorResponse { success: false, error: format!("Invalid UUID: {}", e) };
                            if let Some(reply) = message.reply {
                                let _ = client.publish(reply, serde_json::to_vec(&response).unwrap().into()).await;
                            }
                            return;
                        }
                    };
                    match character_admin_service.delete_character(id).await {
                        Ok(_) => serde_json::to_vec(&SuccessResponse { success: true, data: () }).unwrap(),
                        Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
                    }
                }
                Err(e) => {
                    let response = ErrorResponse { success: false, error: format!("Invalid request: {}", e) };
                    serde_json::to_vec(&response).unwrap()
                }
            }
        }
        "admin.characters.update" => {
            match serde_json::from_slice::<UpdateCharacterRequest>(&message.payload) {
                Ok(req) => {
                    let id = match uuid::Uuid::parse_str(&req.id) {
                        Ok(id) => id,
                        Err(e) => {
                            let response = ErrorResponse { success: false, error: format!("Invalid UUID: {}", e) };
                            if let Some(reply) = message.reply {
                                let _ = client.publish(reply, serde_json::to_vec(&response).unwrap().into()).await;
                            }
                            return;
                        }
                    };
                    match character_admin_service.update_character(id, &req.name, req.race_id, req.gender_id, req.skin_color_id, req.class_id).await {
                        Ok(character) => serde_json::to_vec(&SuccessResponse { success: true, data: character }).unwrap(),
                        Err(e) => serde_json::to_vec(&ErrorResponse { success: false, error: e }).unwrap(),
                    }
                }
                Err(e) => {
                    let response = ErrorResponse { success: false, error: format!("Invalid request: {}", e) };
                    serde_json::to_vec(&response).unwrap()
                }
            }
        }

        _ => {
            let response = ErrorResponse { success: false, error: format!("Unknown subject: {}", subject) };
            serde_json::to_vec(&response).unwrap()
        }
    };

    // Send response if reply subject provided
    if let Some(reply) = message.reply {
        if let Err(e) = client.publish(reply, response_bytes.into()).await {
            error!("Failed to send NATS reply: {}", e);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    EnvConfig::init_from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));
    let _logger_guard = LoggerService::init_from_env("idklol-characters-admin")?;

    // Initialize database pool
    let database_url = EnvConfig::get_required("DATABASE_URL")?;
    info!("connecting to database");
    let pool = db::connect_pool(&database_url, 5).await?;
    
    // Run migrations
    info!("running database migrations");
    let schema_version = db::migrate_and_get_version(&pool, &MIGRATOR).await?;
    info!(?schema_version, "database migrations complete");

    // Initialize services
    let catalog_admin_service = Arc::new(CatalogAdminService::with_pool(pool.clone()));
    let character_admin_service = Arc::new(CharacterAdminService::with_pool(pool.clone()));
    let jwt_validator = Arc::new(JwtValidatorService {});

    // Connect to NATS
    let nats_url = EnvConfig::get_optional("NATS_URL")
        .ok()
        .flatten()
        .unwrap_or_else(|| "nats://nats:4222".to_string());
    
    info!(%nats_url, "connecting to NATS");
    let client = async_nats::connect(&nats_url).await?;
    info!("connected to NATS successfully");

    // Subscribe to catalog admin subjects
    let catalog_subjects = vec![
        "admin.catalog.races.>",
        "admin.catalog.genders.>",
        "admin.catalog.skincolors.>",
        "admin.catalog.classes.>",
        "admin.catalog.combinations.>",
    ];

    for subject_pattern in catalog_subjects {
        let mut subscriber = client.subscribe(subject_pattern.to_string()).await?;
        let catalog_service = catalog_admin_service.clone();
        let validator = jwt_validator.clone();
        let nats_client = client.clone();
        
        tokio::spawn(async move {
            info!(subject = %subject_pattern, "Listening for catalog admin requests");
            while let Some(message) = subscriber.next().await {
                let subject = message.subject.to_string();
                handle_catalog_admin_request(subject, message, catalog_service.clone(), validator.clone(), nats_client.clone()).await;
            }
        });
    }

    // Subscribe to character admin subjects
    let mut char_subscriber = client.subscribe("admin.characters.>".to_string()).await?;
    let char_service = character_admin_service.clone();
    let char_validator = jwt_validator.clone();
    let char_client = client.clone();
    
    tokio::spawn(async move {
        info!("Listening for character admin requests");
        while let Some(message) = char_subscriber.next().await {
            let subject = message.subject.to_string();
            handle_character_admin_request(subject, message, char_service.clone(), char_validator.clone(), char_client.clone()).await;
        }
    });

    info!("Characters admin NATS server is running");
    
    // Keep the server running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down characters admin NATS server");

    Ok(())
}
