use tonic::{Request, Status};
use crate::auth::jwt::jwt_validator_service::JwtValidatorService;
use crate::auth::token_validator_service::TokenValidatorService;
use std::sync::Arc;
use tracing::{debug, warn};

pub fn jwt_auth_interceptor(jwt_validator: Arc<JwtValidatorService>) -> impl Clone + FnMut(Request<()>) -> Result<Request<()>, Status> {
    move |req: Request<()>| {
        let metadata = req.metadata();
        let token = metadata
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .and_then(|auth_header| auth_header.strip_prefix("Bearer "))
            .map(String::from)
            .ok_or_else(|| {
                warn!("missing or invalid authorization header");
                Status::unauthenticated("missing or invalid authorization header")
            })?;

        debug!(token_preview = %&token[..50.min(token.len())], "validating token");
        
        // Spawn async validation on current runtime (don't block)
        let validator = jwt_validator.clone();
        let valid = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(validator.validate_token(&token))
        });
        
        if !valid {
            warn!("token validation failed in interceptor");
            return Err(Status::unauthenticated("invalid token"));
        }
        debug!("token validated successfully in interceptor");
        Ok(req)
    }
}
