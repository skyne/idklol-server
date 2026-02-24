use tonic::transport::Server;
use chat::chat_service_server::ChatServiceServer;
use idklol_common::config::env_config::EnvConfig;
use idklol_common::logging::logger_service::LoggerService;
use tracing::{debug, info, warn};

pub mod chat {
    tonic::include_proto!("chat");
}

mod services;
use services::chat_service::MyChatService;
use idklol_common::auth::{
    jwt::jwt_validator_service::JwtValidatorService,
    token_validator_service::TokenValidatorService,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    EnvConfig::init_from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));
    let _logger_guard = LoggerService::init_from_env("idklol-chat")?;

    let addr = "0.0.0.0:50052".parse()?;
    let chat_service = MyChatService::default();

    info!(%addr, "chat server starting");

    Server::builder()
        .add_service(ChatServiceServer::new(chat_service))
        .serve(addr)
        .await?;

    Ok(())
}