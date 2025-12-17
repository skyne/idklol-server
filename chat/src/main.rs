use tonic::transport::Server;
use chat::chat_service_server::ChatServiceServer;

pub mod chat {
    tonic::include_proto!("chat");
}

mod services;
use services::chat_service::MyChatService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50052".parse()?;
    let chat_service = MyChatService::default();

    println!("Auth server running on {}", addr);

    Server::builder()
        .add_service(ChatServiceServer::new(chat_service))
        .serve(addr)
        .await?;

    Ok(())
}