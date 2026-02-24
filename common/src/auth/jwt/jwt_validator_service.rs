use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::Deserialize;
use std::sync::{Arc, OnceLock, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, error, warn};

use crate::{auth::token_validator_service::TokenValidatorService, config::env_config::EnvConfig};

#[derive(Debug, Deserialize)]
struct Claims {
    email: Option<String>,
}

pub struct JwtValidatorService {}

struct CachedDecodingKey {
    key: Arc<DecodingKey>,
    cached_at: Instant,
}

static DECODING_KEY_CACHE: OnceLock<RwLock<Option<CachedDecodingKey>>> = OnceLock::new();

impl JwtValidatorService {
    fn key_cache() -> &'static RwLock<Option<CachedDecodingKey>> {
        DECODING_KEY_CACHE.get_or_init(|| RwLock::new(None))
    }

    fn key_cache_ttl() -> Duration {
        Duration::from_secs(EnvConfig::get_keycloak_public_key_cache_ttl_seconds())
    }

    fn get_cached_decoding_key(ttl: Duration) -> Option<Arc<DecodingKey>> {
        let cache = Self::key_cache();
        let guard = cache.read().ok()?;
        match guard.as_ref() {
            Some(entry) if entry.cached_at.elapsed() < ttl => Some(Arc::clone(&entry.key)),
            _ => None,
        }
    }

    fn set_cached_decoding_key(key: Arc<DecodingKey>) {
        let cache = Self::key_cache();
        if let Ok(mut guard) = cache.write() {
            *guard = Some(CachedDecodingKey {
                key,
                cached_at: Instant::now(),
            });
        }
    }
}

impl TokenValidatorService for JwtValidatorService {
    async fn validate_token(&self, token: &str) -> bool {
        let cache_ttl = Self::key_cache_ttl();
        let decoding_key = if let Some(cached_key) = Self::get_cached_decoding_key(cache_ttl) {
            debug!("using cached Keycloak public key");
            cached_key
        } else {
            let keycloak_url = match EnvConfig::get_required("KEYCLOAK_URL") {
                Ok(url) => url,
                Err(err) => {
                    error!(error = %err, "missing or invalid KEYCLOAK_URL");
                    return false;
                }
            };

            let parsed_resp: serde_json::Value = match reqwest::get(&keycloak_url).await {
                Ok(response) => match response.error_for_status() {
                    Ok(valid_response) => match valid_response.json::<serde_json::Value>().await {
                        Ok(value) => value,
                        Err(err) => {
                            error!(error = %err, "failed to parse Keycloak response body");
                            return false;
                        }
                    },
                    Err(err) => {
                        error!(error = %err, "Keycloak returned non-success status");
                        return false;
                    }
                },
                Err(err) => {
                    error!(error = %err, "failed to call Keycloak endpoint");
                    return false;
                }
            };

            let public_key = match parsed_resp.get("public_key").and_then(|value| value.as_str()) {
                Some(key) if !key.is_empty() => key,
                _ => {
                    error!("public_key not found in Keycloak response");
                    return false;
                }
            };

            let rsa_pem = format!(
                "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----",
                public_key
            );

            let fetched_key = match DecodingKey::from_rsa_pem(rsa_pem.as_bytes()) {
                Ok(key) => Arc::new(key),
                Err(err) => {
                    error!(error = %err, "failed to create RSA decoding key from public key");
                    return false;
                }
            };

            Self::set_cached_decoding_key(Arc::clone(&fetched_key));
            debug!("refreshed Keycloak public key cache");
            fetched_key
        };

        let validation = Validation::new(Algorithm::RS256);
        match decode::<serde_json::Value>(token, decoding_key.as_ref(), &validation) {
            Ok(_) => true,
            Err(err) => {
                warn!(error = %err, "token validation failed");
                false
            }
        }
    }
    
    async fn validate_and_extract_email(&self, token: &str) -> Option<String> {
        let cache_ttl = Self::key_cache_ttl();
        let decoding_key = if let Some(cached_key) = Self::get_cached_decoding_key(cache_ttl) {
            debug!("using cached Keycloak public key for claims extraction");
            cached_key
        } else {
            let keycloak_url = match EnvConfig::get_required("KEYCLOAK_URL") {
                Ok(url) => url,
                Err(err) => {
                    error!(error = %err, "missing or invalid KEYCLOAK_URL");
                    return None;
                }
            };

            let parsed_resp: serde_json::Value = match reqwest::get(&keycloak_url).await {
                Ok(response) => match response.error_for_status() {
                    Ok(valid_response) => match valid_response.json::<serde_json::Value>().await {
                        Ok(value) => value,
                        Err(err) => {
                            error!(error = %err, "failed to parse Keycloak response body");
                            return None;
                        }
                    },
                    Err(err) => {
                        error!(error = %err, "Keycloak returned non-success status");
                        return None;
                    }
                },
                Err(err) => {
                    error!(error = %err, "failed to call Keycloak endpoint");
                    return None;
                }
            };

            let public_key = match parsed_resp.get("public_key").and_then(|value| value.as_str()) {
                Some(key) if !key.is_empty() => key,
                _ => {
                    error!("public_key not found in Keycloak response");
                    return None;
                }
            };

            let rsa_pem = format!(
                "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----",
                public_key
            );

            let fetched_key = match DecodingKey::from_rsa_pem(rsa_pem.as_bytes()) {
                Ok(key) => Arc::new(key),
                Err(err) => {
                    error!(error = %err, "failed to create RSA decoding key from public key");
                    return None;
                }
            };

            Self::set_cached_decoding_key(Arc::clone(&fetched_key));
            debug!("refreshed Keycloak public key cache");
            fetched_key
        };

        let validation = Validation::new(Algorithm::RS256);
        match decode::<Claims>(token, decoding_key.as_ref(), &validation) {
            Ok(token_data) => {
                match token_data.claims.email {
                    Some(email) if !email.is_empty() => {
                        debug!(%email, "successfully extracted email from token");
                        Some(email)
                    }
                    _ => {
                        warn!("email claim not found or empty in token");
                        None
                    }
                }
            }
            Err(err) => {
                warn!(error = %err, "token validation failed during claims extraction");
                None
            }
        }
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