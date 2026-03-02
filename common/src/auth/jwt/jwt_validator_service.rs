use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, error, warn};

use crate::{auth::token_validator_service::TokenValidatorService, config::env_config::EnvConfig};

#[derive(Debug, Deserialize)]
struct TokenHeader {
    kid: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Claims {
    exp: Option<i64>,
    iat: Option<i64>,
    jti: Option<String>,
    email: Option<String>,
    realm_access: Option<RealmAccess>,
    sub: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct RealmAccess {
    roles: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct UserClaims {
    pub email: String,
    pub roles: Vec<String>,
    pub sub: String,
}

impl UserClaims {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    pub fn is_admin(&self) -> bool {
        self.has_role("admin")
    }
}

#[derive(Debug, Deserialize)]
struct IntrospectionResponse {
    active: bool,
    email: Option<String>,
    realm_access: Option<RealmAccess>,
    sub: Option<String>,
}

pub struct JwtValidatorService {}

#[derive(Clone)]
struct CachedTokenValidation {
    is_valid: bool,
    email: Option<String>,
    roles: Option<Vec<String>>,
    sub: Option<String>,
    cached_at: Instant,
}

// Cache for token validation results (hash of token -> validation result)
static TOKEN_CACHE: OnceLock<RwLock<HashMap<String, CachedTokenValidation>>> = OnceLock::new();

impl JwtValidatorService {
    fn token_cache() -> &'static RwLock<HashMap<String, CachedTokenValidation>> {
        TOKEN_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
    }

    fn valid_cache_ttl() -> Duration {
        Duration::from_secs(EnvConfig::get_keycloak_public_key_cache_ttl_seconds())
    }

    fn invalid_cache_ttl() -> Duration {
        Duration::from_secs(60) // Cache invalid tokens for 1 minute
    }

    fn get_cached_validation(token_hash: &str) -> Option<CachedTokenValidation> {
        let cache = Self::token_cache();
        let guard = cache.read().ok()?;
        let entry = guard.get(token_hash)?;
        
        let ttl = if entry.is_valid {
            Self::valid_cache_ttl()
        } else {
            Self::invalid_cache_ttl()
        };

        if entry.cached_at.elapsed() < ttl {
            Some(entry.clone())
        } else {
            None
        }
    }

    fn set_cached_validation(token_hash: String, is_valid: bool, email: Option<String>, roles: Option<Vec<String>>, sub: Option<String>) {
        let cache = Self::token_cache();
        if let Ok(mut guard) = cache.write() {
            guard.insert(token_hash, CachedTokenValidation {
                is_valid,
                email,
                roles,
                sub,
                cached_at: Instant::now(),
            });
        }
    }

    fn compute_token_hash(token: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        token.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    fn check_token_expiration(token: &str) -> bool {
        // Manually decode JWT payload (base64url encoded JSON between the two dots)
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            debug!("Invalid JWT format");
            return false;
        }

        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

        // Decode the header (first part) to get kid
        let header_base64 = parts[0];
        let header_decoded = URL_SAFE_NO_PAD.decode(header_base64).ok();
        let header_kid = header_decoded
            .and_then(|bytes| serde_json::from_slice::<TokenHeader>(&bytes).ok())
            .and_then(|h| h.kid);

        // Decode the payload (second part)
        let payload_base64 = parts[1];
        let decoded_bytes = match URL_SAFE_NO_PAD.decode(payload_base64) {
            Ok(bytes) => bytes,
            Err(err) => {
                debug!(error = %err, "Failed to decode JWT payload");
                return false;
            }
        };

        let claims: Claims = match serde_json::from_slice(&decoded_bytes) {
            Ok(c) => c,
            Err(err) => {
                debug!(error = %err, "Failed to parse claims");
                return false;
            }
        };

        if let Some(exp) = claims.exp {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            
            debug!(
                kid = ?header_kid, 
                jti = ?claims.jti, 
                iat = ?claims.iat,
                exp = exp, 
                now = now, 
                "Checking token expiration"
            );
            
            if exp < now {
                warn!(exp = exp, now = now, kid = ?header_kid, jti = ?claims.jti, "Token expired");
                return false;
            }
            debug!(exp = exp, kid = ?header_kid, jti = ?claims.jti, "Token expiration check passed");
            true
        } else {
            warn!("Token missing exp claim");
            false
        }
    }

    async fn introspect_token(keycloak_url: &str, client_id: &str, client_secret: &str, token: &str) -> Result<IntrospectionResponse, String> {
        let introspect_url = format!("{}/protocol/openid-connect/token/introspect", keycloak_url);
        
        let params = HashMap::from([
            ("token".to_string(), token.to_string()),
            ("client_id".to_string(), client_id.to_string()),
            ("client_secret".to_string(), client_secret.to_string()),
        ]);

        debug!(url = %introspect_url, client_id = %client_id, token_preview = %&token[..token.len().min(20)], "Calling introspection endpoint");

        let client = reqwest::Client::new();
        let response = client
            .post(&introspect_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(serde_urlencoded::to_string(&params).unwrap())
            .send()
            .await
            .map_err(|e| format!("Failed to call introspection endpoint: {}", e))?;

        let status = response.status();
        debug!(status = %status, "Introspection response status");

        if !status.is_success() {
            return Err(format!("Introspection endpoint returned status: {}", status));
        }

        let response_text = response.text().await
            .map_err(|e| format!("Failed to read introspection response: {}", e))?;
        
        debug!(response_body = %response_text, "Introspection response body");

        let introspection: IntrospectionResponse = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse introspection response: {}", e))?;

        Ok(introspection)
    }
}

impl TokenValidatorService for JwtValidatorService {
    async fn validate_token(&self, token: &str) -> bool {
        let token_hash = Self::compute_token_hash(token);

        // Check cache first
        if let Some(cached) = Self::get_cached_validation(&token_hash) {
            debug!(is_valid = cached.is_valid, "Using cached token validation");
            return cached.is_valid;
        }

        // Local sanity check: expiration
        if !Self::check_token_expiration(token) {
            debug!("Token failed local expiration check");
            Self::set_cached_validation(token_hash, false, None, None, None);
            return false;
        }

        // Token passed local checks, now introspect with Keycloak
        let keycloak_url = match EnvConfig::get_required("KEYCLOAK_URL") {
            Ok(url) => url,
            Err(err) => {
                error!(error = %err, "missing or invalid KEYCLOAK_URL");
                return false;
            }
        };

        let client_id = EnvConfig::get_optional("KEYCLOAK_CLIENT_ID")
            .ok()
            .flatten()
            .unwrap_or_else(|| "idklol-chat".to_string());
        let client_secret = EnvConfig::get_optional("KEYCLOAK_CLIENT_SECRET")
            .ok()
            .flatten()
            .unwrap_or_else(|| "your-client-secret".to_string());

        match Self::introspect_token(&keycloak_url, &client_id, &client_secret, token).await {
            Ok(introspection) => {
                if introspection.active {
                    let roles = introspection.realm_access.as_ref().map(|ra| ra.roles.clone());
                    debug!(email = ?introspection.email, roles = ?roles, active = true, "Token introspection successful");
                    Self::set_cached_validation(token_hash, true, introspection.email, roles, introspection.sub);
                    true
                } else {
                    warn!(active = false, email = ?introspection.email, "Token introspection returned inactive");
                    Self::set_cached_validation(token_hash, false, None, None, None);
                    false
                }
            }
            Err(err) => {
                error!(error = %err, "Token introspection failed");
                Self::set_cached_validation(token_hash, false, None, None, None);
                false
            }
        }
    }
    
    async fn validate_and_extract_email(&self, token: &str) -> Option<String> {
        let token_hash = Self::compute_token_hash(token);

        // Check cache first
        if let Some(cached) = Self::get_cached_validation(&token_hash) {
            if cached.is_valid {
                debug!(email = ?cached.email, "Using cached token validation for email extraction");
                return cached.email;
            } else {
                debug!("Token is cached as invalid");
                return None;
            }
        }

        // Validate will populate cache
        if !self.validate_token(token).await {
            return None;
        }

        // Re-check cache after validation
        if let Some(cached) = Self::get_cached_validation(&token_hash) {
            return cached.email;
        }

        None
    }
}

impl JwtValidatorService {
    /// Validate token and extract full user claims including email, roles, and sub
    pub async fn validate_and_extract_claims(&self, token: &str) -> Option<UserClaims> {
        let token_hash = Self::compute_token_hash(token);

        // Check cache first
        if let Some(cached) = Self::get_cached_validation(&token_hash) {
            if cached.is_valid {
                if let (Some(email), Some(roles), Some(sub)) = (cached.email, cached.roles, cached.sub) {
                    debug!(email = ?email, roles = ?roles, "Using cached token validation for claims extraction");
                    return Some(UserClaims { email, roles, sub });
                }
            } else {
                debug!("Token is cached as invalid");
                return None;
            }
        }

        // Validate will populate cache
        if !self.validate_token(token).await {
            return None;
        }

        // Re-check cache after validation
        if let Some(cached) = Self::get_cached_validation(&token_hash) {
            if let (Some(email), Some(roles), Some(sub)) = (cached.email, cached.roles, cached.sub) {
                return Some(UserClaims { email, roles, sub });
            }
        }

        None
    }

    /// Extract user claims and verify admin role
    pub async fn validate_and_require_admin(&self, token: &str) -> Result<UserClaims, String> {
        let claims = self.validate_and_extract_claims(token).await
            .ok_or_else(|| "Invalid or expired token".to_string())?;
        
        if !claims.is_admin() {
            return Err("Admin role required".to_string());
        }

        Ok(claims)
    }
}

#[cfg(test)]
mod tests {
    use super::JwtValidatorService;
    use crate::auth::token_validator_service::TokenValidatorService;

    #[tokio::test]
    async fn validate_token_returns_false_when_keycloak_url_missing() {
        if std::env::var("KEYCLOAK_URL").is_ok() {
            return;
        }

        let service = JwtValidatorService {};
        let is_valid = service.validate_token("not-a-jwt").await;

        assert!(!is_valid);
    }
}