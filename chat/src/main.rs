use tonic::transport::Server;
use chat::chat_service_server::ChatServiceServer;
use idklol_common::config::env_config::EnvConfig;
use idklol_common::logging::logger_service::LoggerService;
use idklol_common::runtime;
use tracing::info;

pub mod chat {
    tonic::include_proto!("chat");
}

mod services;
use services::chat_service::MyChatService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    EnvConfig::init_from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));
    let _logger_guard = LoggerService::init_from_env("idklol-chat")?;

    let addr = "0.0.0.0:50052".parse()?;
    let chat_service = MyChatService::from_env().await;

    info!(%addr, "chat server starting");

    Server::builder()
        .add_service(ChatServiceServer::new(chat_service))
        .serve_with_shutdown(addr, runtime::wait_for_shutdown_signal())
        .await?;

    Ok(())
}