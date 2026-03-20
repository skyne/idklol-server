use std::env;
use std::error::Error;
use std::fmt;
use std::str::FromStr;
use std::sync::Once;

static DOTENV_INIT: Once = Once::new();
pub const DEFAULT_KEYCLOAK_PUBLIC_KEY_CACHE_TTL_SECONDS: u64 = 300;
pub const DEFAULT_STARTUP_RETRY_MAX_ATTEMPTS: u32 = 30;
pub const DEFAULT_STARTUP_RETRY_INITIAL_DELAY_MS: u64 = 1000;
pub const DEFAULT_STARTUP_RETRY_MAX_DELAY_MS: u64 = 30000;

#[derive(Debug)]
pub enum EnvConfigError {
	MissingVar { key: String },
	InvalidVar {
		key: String,
		value: String,
		message: String,
	},
}

impl fmt::Display for EnvConfigError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			EnvConfigError::MissingVar { key } => {
				write!(f, "Required environment variable `{key}` is missing")
			}
			EnvConfigError::InvalidVar {
				key,
				value,
				message,
			} => write!(
				f,
				"Invalid value for environment variable `{key}`: `{value}` ({message})"
			),
		}
	}
}

impl Error for EnvConfigError {}

pub struct EnvConfig;

impl EnvConfig {
	pub fn init_from_path(path: &str) {
		DOTENV_INIT.call_once(|| {
			let _ = dotenvy::from_path(path);
		});
	}

	fn ensure_dotenv_loaded() {
		DOTENV_INIT.call_once(|| {
			if dotenvy::dotenv().is_ok() {
				return;
			}

			if let Ok(path) = env::var("ENV_FILE") {
				let _ = dotenvy::from_path(path);
			}
		});
	}

	pub fn get_required(key: &str) -> Result<String, EnvConfigError> {
		Self::ensure_dotenv_loaded();
		match env::var(key) {
			Ok(value) => Ok(value),
			Err(env::VarError::NotPresent) => Err(EnvConfigError::MissingVar {
				key: key.to_string(),
			}),
			Err(env::VarError::NotUnicode(_)) => Err(EnvConfigError::InvalidVar {
				key: key.to_string(),
				value: String::from("<non-unicode>"),
				message: String::from("value is not valid unicode"),
			}),
		}
	}

	pub fn get_optional(key: &str) -> Result<Option<String>, EnvConfigError> {
		Self::ensure_dotenv_loaded();
		match env::var(key) {
			Ok(value) => Ok(Some(value)),
			Err(env::VarError::NotPresent) => Ok(None),
			Err(env::VarError::NotUnicode(_)) => Err(EnvConfigError::InvalidVar {
				key: key.to_string(),
				value: String::from("<non-unicode>"),
				message: String::from("value is not valid unicode"),
			}),
		}
	}

	pub fn get_required_parsed<T>(key: &str) -> Result<T, EnvConfigError>
	where
		T: FromStr,
		T::Err: fmt::Display,
	{
		let value = Self::get_required(key)?;
		value
			.parse::<T>()
			.map_err(|err| EnvConfigError::InvalidVar {
				key: key.to_string(),
				value,
				message: err.to_string(),
			})
	}

	pub fn get_optional_parsed<T>(key: &str) -> Result<Option<T>, EnvConfigError>
	where
		T: FromStr,
		T::Err: fmt::Display,
	{
		match Self::get_optional(key)? {
			Some(value) => value
				.parse::<T>()
				.map(Some)
				.map_err(|err| EnvConfigError::InvalidVar {
					key: key.to_string(),
					value,
					message: err.to_string(),
				}),
			None => Ok(None),
		}
	}

	pub fn get_keycloak_public_key_cache_ttl_seconds() -> u64 {
		match Self::get_optional_parsed::<u64>("KEYCLOAK_PUBLIC_KEY_CACHE_TTL_SECONDS") {
			Ok(Some(seconds)) => seconds,
			Ok(None) => DEFAULT_KEYCLOAK_PUBLIC_KEY_CACHE_TTL_SECONDS,
			Err(_) => DEFAULT_KEYCLOAK_PUBLIC_KEY_CACHE_TTL_SECONDS,
		}
	}

	pub fn get_startup_retry_max_attempts() -> u32 {
		match Self::get_optional_parsed::<u32>("STARTUP_RETRY_MAX_ATTEMPTS") {
			Ok(Some(attempts)) if attempts > 0 => attempts,
			_ => DEFAULT_STARTUP_RETRY_MAX_ATTEMPTS,
		}
	}

	pub fn get_startup_retry_initial_delay_ms() -> u64 {
		match Self::get_optional_parsed::<u64>("STARTUP_RETRY_INITIAL_DELAY_MS") {
			Ok(Some(delay_ms)) if delay_ms > 0 => delay_ms,
			_ => DEFAULT_STARTUP_RETRY_INITIAL_DELAY_MS,
		}
	}

	pub fn get_startup_retry_max_delay_ms() -> u64 {
		match Self::get_optional_parsed::<u64>("STARTUP_RETRY_MAX_DELAY_MS") {
			Ok(Some(delay_ms)) if delay_ms > 0 => delay_ms,
			_ => DEFAULT_STARTUP_RETRY_MAX_DELAY_MS,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{
		EnvConfig,
		EnvConfigError,
		DEFAULT_KEYCLOAK_PUBLIC_KEY_CACHE_TTL_SECONDS,
	};
	use std::time::{SystemTime, UNIX_EPOCH};

	fn unique_missing_key(prefix: &str) -> String {
		let nanos = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.expect("system time before unix epoch")
			.as_nanos();
		format!("{prefix}_{nanos}")
	}

	#[test]
	fn get_optional_returns_none_for_missing_key() {
		let key = unique_missing_key("OPTIONAL_ENV_MISSING_TEST");
		let value = EnvConfig::get_optional(&key).expect("get_optional should not error");
		assert!(value.is_none());
	}

	#[test]
	fn get_required_returns_missing_error_for_missing_key() {
		let key = unique_missing_key("REQUIRED_ENV_MISSING_TEST");
		let result = EnvConfig::get_required(&key);
		match result {
			Err(EnvConfigError::MissingVar { key: missing_key }) => {
				assert_eq!(missing_key, key);
			}
			other => panic!("expected MissingVar error, got: {:?}", other),
		}
	}

	#[test]
	fn get_optional_parsed_returns_none_for_missing_key() {
		let key = unique_missing_key("OPTIONAL_PARSED_ENV_MISSING_TEST");
		let value = EnvConfig::get_optional_parsed::<u16>(&key)
			.expect("get_optional_parsed should not error for missing var");
		assert!(value.is_none());
	}

	#[test]
	fn keycloak_public_key_cache_ttl_uses_default_when_env_missing() {
		let ttl = EnvConfig::get_keycloak_public_key_cache_ttl_seconds();
		assert_eq!(ttl, DEFAULT_KEYCLOAK_PUBLIC_KEY_CACHE_TTL_SECONDS);
	}
}
