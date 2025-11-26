use tonic::{Request, Response, Status};
use crate::auth::auth_service_server::AuthService;
use crate::auth::{LoginRequest, LoginResponse, LoginStatus};

#[derive(Debug, Default)]
pub struct MyAuthService {}

#[tonic::async_trait]
impl AuthService for MyAuthService {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        println!("Got a request: {:?}", request);

        let status = LoginStatus::Success;

        let reply = LoginResponse {
            status: status as i32,
            message: format!("Hello {}!", request.into_inner().username),
        };

        Ok(Response::new(reply))
    }
}