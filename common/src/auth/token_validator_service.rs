#[allow(async_fn_in_trait)]
pub trait TokenValidatorService {
    async fn validate_token(&self, token: &str) -> bool;
    
    /// Validate the token and extract the email claim if valid
    /// Returns None if token is invalid or email claim is missing
    async fn validate_and_extract_email(&self, token: &str) -> Option<String>;
}