use tonic::{Request, Response, Status};
use crate::auth::auth_service_server::AuthService;
use crate::auth::{LoginRequest, LoginResponse, LoginStatus};
use chrono::Local;

#[derive(Debug, Default)]
pub struct MyAuthService {}

const  DATE_FORMAT_STRING: &'static str = "[%Y-%m-%d %H:%M:%S:%.3f]";

#[tonic::async_trait]
impl AuthService for MyAuthService {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        println!("{}: Got a request: {:?}", Local::now().format(DATE_FORMAT_STRING), request);

        let status = LoginStatus::Success;

        let reply = LoginResponse {
            status: status as i32,
            message: format!("Hello {}!", request.into_inner().username),
        };

        Ok(Response::new(reply))
    }
}