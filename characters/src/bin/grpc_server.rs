use tonic::transport::Server;
use characters_core::characters::character_service_server::CharacterServiceServer;
use idklol_common::config::env_config::EnvConfig;
use idklol_common::db;
use idklol_common::logging::logger_service::LoggerService;
use idklol_common::runtime;
use tracing::info;
use idklol_common::auth::jwt::jwt_validator_service::JwtValidatorService;
use idklol_common::auth::interceptor::jwt_auth_interceptor;
use std::sync::Arc;

use characters_core::services::character_service::MyCharacterService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    EnvConfig::init_from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));
    let _logger_guard = LoggerService::init_from_env("idklol-characters-grpc")?;
    let retry_config = runtime::RetryConfig::from_env();

    // Initialize database pool
    let database_url = EnvConfig::get_required("DATABASE_URL")?;
    info!("connecting to database");
    let pool = runtime::retry_with_backoff("database connection", retry_config, || {
        db::connect_pool(&database_url, 5)
    })
    .await?;
    
    // Run migrations
    info!("running database migrations");
    let schema_version = runtime::retry_with_backoff("database migrations", retry_config, || {
        db::migrate_and_get_version(&pool, &characters_core::MIGRATOR)
    })
    .await?;
    info!(?schema_version, "database migrations complete");

    let addr = "0.0.0.0:50052".parse()?;
    let character_service = MyCharacterService::with_pool(pool);
    let jwt_validator = Arc::new(JwtValidatorService {});
    let interceptor = jwt_auth_interceptor(jwt_validator);
    info!(%addr, "character gRPC server starting");

    Server::builder()
        .add_service(CharacterServiceServer::with_interceptor(character_service, interceptor))
        .serve_with_shutdown(addr, runtime::wait_for_shutdown_signal())
        .await?;

    Ok(())
}
