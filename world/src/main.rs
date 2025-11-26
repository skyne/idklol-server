use tonic::transport::Server;
use world::world_service_server::WorldServiceServer;

pub mod world {
    tonic::include_proto!("world");
}

mod services;
use services::world_service::MyWorldService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let world_service = MyWorldService::default();

    println!("World server running on {}", addr);

    Server::builder()
        .add_service(WorldServiceServer::new(world_service))
        .serve(addr)
        .await?;

    Ok(())
}