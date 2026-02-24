use tonic::{Status, metadata::MetadataMap};
use idklol_common::auth::jwt::jwt_validator_service::JwtValidatorService;
use idklol_common::auth::token_validator_service::TokenValidatorService;

/// Extracts the bearer token from the authorization metadata
pub fn extract_bearer_token(metadata: &MetadataMap) -> Option<String> {
    metadata
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|auth_header| {
            auth_header.strip_prefix("Bearer ").map(String::from)
        })
}

/// Authenticates a user using the JWT token and returns the user's email
pub async fn authenticate_user(
    jwt_validator: &JwtValidatorService,
    metadata: &MetadataMap,
) -> Result<String, Status> {
    let token = extract_bearer_token(metadata)
        .ok_or_else(|| Status::unauthenticated("missing or invalid authorization header"))?;

    jwt_validator
        .validate_and_extract_email(&token)
        .await
        .ok_or_else(|| Status::unauthenticated("invalid token or missing email claim"))
}
