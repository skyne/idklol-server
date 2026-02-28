use tonic::transport::Server;
use characters::character_service_server::CharacterServiceServer;
use idklol_common::config::env_config::EnvConfig;
use idklol_common::db;
use idklol_common::logging::logger_service::LoggerService;
use tracing::info;
use idklol_common::auth::jwt::jwt_validator_service::JwtValidatorService;
use idklol_common::auth::interceptor::jwt_auth_interceptor;
use std::sync::Arc;

pub mod characters {
    tonic::include_proto!("characters");
}

mod models;
mod repository;
mod services;
use services::character_service::MyCharacterService;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    EnvConfig::init_from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));
    let _logger_guard = LoggerService::init_from_env("idklol-characters")?;

    // Initialize database pool
    let database_url = EnvConfig::get_required("DATABASE_URL")?;
    info!("connecting to database");
    let pool = db::connect_pool(&database_url, 5).await?;
    
    // Run migrations
    info!("running database migrations");
    let schema_version = db::migrate_and_get_version(&pool, &MIGRATOR).await?;
    info!(?schema_version, "database migrations complete");

    let addr = "0.0.0.0:50052".parse()?;
    let character_service = MyCharacterService::with_pool(pool);
    let jwt_validator = Arc::new(JwtValidatorService {});
    let interceptor = jwt_auth_interceptor(jwt_validator);
    info!(%addr, "character server starting");

    Server::builder()
        .add_service(CharacterServiceServer::with_interceptor(character_service, interceptor))
        .serve(addr)
        .await?;

    Ok(())
}