use std::future::Future;
use std::time::Duration;

use tracing::{info, warn};

use crate::config::env_config::EnvConfig;

#[derive(Debug, Clone, Copy)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
}

impl RetryConfig {
    pub fn from_env() -> Self {
        Self {
            max_attempts: EnvConfig::get_startup_retry_max_attempts(),
            initial_delay: Duration::from_millis(EnvConfig::get_startup_retry_initial_delay_ms()),
            max_delay: Duration::from_millis(EnvConfig::get_startup_retry_max_delay_ms()),
        }
    }
}

pub async fn retry_with_backoff<T, E, F, Fut>(
    operation_name: &str,
    config: RetryConfig,
    mut operation: F,
) -> Result<T, E>
where
    E: std::fmt::Display,
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let max_attempts = config.max_attempts.max(1);
    let mut delay = config.initial_delay.max(Duration::from_millis(100));

    for attempt in 1..=max_attempts {
        match operation().await {
            Ok(value) => {
                if attempt > 1 {
                    info!(
                        operation = operation_name,
                        attempt,
                        "startup dependency became available"
                    );
                }
                return Ok(value);
            }
            Err(error) => {
                if attempt == max_attempts {
                    warn!(
                        operation = operation_name,
                        attempt,
                        max_attempts,
                        error = %error,
                        "startup dependency failed after final retry"
                    );
                    return Err(error);
                }

                warn!(
                    operation = operation_name,
                    attempt,
                    max_attempts,
                    retry_in_ms = delay.as_millis(),
                    error = %error,
                    "startup dependency not ready yet; retrying"
                );

                tokio::time::sleep(delay).await;
                delay = std::cmp::min(delay.saturating_mul(2), config.max_delay);
            }
        }
    }

    unreachable!("retry loop always returns before reaching this point");
}

pub async fn wait_for_shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};

        match (
            signal(SignalKind::interrupt()),
            signal(SignalKind::terminate()),
        ) {
            (Ok(mut sigint), Ok(mut sigterm)) => {
                tokio::select! {
                    _ = sigint.recv() => info!("received SIGINT; starting graceful shutdown"),
                    _ = sigterm.recv() => info!("received SIGTERM; starting graceful shutdown"),
                }
            }
            _ => {
                warn!("failed to register unix signal handlers; falling back to ctrl_c");
                if let Err(error) = tokio::signal::ctrl_c().await {
                    warn!(%error, "failed waiting for ctrl_c signal");
                }
            }
        }
    }

    #[cfg(not(unix))]
    {
        if let Err(error) = tokio::signal::ctrl_c().await {
            warn!(%error, "failed waiting for ctrl_c signal");
        }
    }
}