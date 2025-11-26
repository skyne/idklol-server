use tonic::transport::Server;
use auth::auth_service_server::AuthServiceServer;

pub mod auth {
    tonic::include_proto!("auth");
}

mod services;
use services::auth_service::MyAuthService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let auth_service = MyAuthService::default();

    println!("Auth server running on {}", addr);

    Server::builder()
        .add_service(AuthServiceServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}