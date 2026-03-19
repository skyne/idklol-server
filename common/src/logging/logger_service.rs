use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

use opentelemetry::trace::TracerProvider;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::{Sampler, TracerProvider as SdkTracerProvider};
use opentelemetry_sdk::Resource;
use tracing::metadata::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

use crate::config::env_config::{EnvConfig, EnvConfigError};

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl LogLevel {
    fn from_str(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "error" => Some(Self::Error),
            "warn" | "warning" => Some(Self::Warn),
            "info" => Some(Self::Info),
            "debug" => Some(Self::Debug),
            _ => None,
        }
    }

    fn as_level_filter(self) -> LevelFilter {
        match self {
            Self::Error => LevelFilter::ERROR,
            Self::Warn => LevelFilter::WARN,
            Self::Info => LevelFilter::INFO,
            Self::Debug => LevelFilter::DEBUG,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LogOutput {
    Console,
    File,
    Both,
}

impl LogOutput {
    fn from_str(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "console" => Some(Self::Console),
            "file" => Some(Self::File),
            "both" => Some(Self::Both),
            _ => None,
        }
    }

    fn includes_console(self) -> bool {
        matches!(self, Self::Console | Self::Both)
    }

    fn includes_file(self) -> bool {
        matches!(self, Self::File | Self::Both)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LogFormat {
    Plain,
    Json,
}

impl LogFormat {
    fn from_str(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "plain" => Some(Self::Plain),
            "json" | "bunyan" => Some(Self::Json),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoggerConfig {
    pub level: LogLevel,
    pub output: LogOutput,
    pub format: LogFormat,
    pub file_path: String,
    pub tracing: TracingConfig,
}

#[derive(Debug, Clone)]
pub struct TracingConfig {
    pub otlp_enabled: bool,
    pub otlp_endpoint: String,
    pub sample_ratio: f64,
}

impl LoggerConfig {
    pub fn from_env() -> Result<Self, LoggerServiceError> {
        let level = match EnvConfig::get_optional("LOG_LEVEL")? {
            Some(value) => LogLevel::from_str(&value).ok_or_else(|| {
                LoggerServiceError::InvalidConfig(format!(
                    "Invalid LOG_LEVEL `{value}`. Use one of: error, warn, info, debug"
                ))
            })?,
            None => LogLevel::Info,
        };

        let output = match EnvConfig::get_optional("LOG_OUTPUT")? {
            Some(value) => LogOutput::from_str(&value).ok_or_else(|| {
                LoggerServiceError::InvalidConfig(format!(
                    "Invalid LOG_OUTPUT `{value}`. Use one of: console, file, both"
                ))
            })?,
            None => LogOutput::Console,
        };

        let format = match EnvConfig::get_optional("LOG_FORMAT")? {
            Some(value) => LogFormat::from_str(&value).ok_or_else(|| {
                LoggerServiceError::InvalidConfig(format!(
                    "Invalid LOG_FORMAT `{value}`. Use one of: plain, json"
                ))
            })?,
            None => LogFormat::Plain,
        };

        let file_path = EnvConfig::get_optional("LOG_FILE_PATH")?
            .unwrap_or_else(|| String::from("logs/service.log"));

        let otlp_enabled = match EnvConfig::get_optional("TRACING_OTLP_ENABLED")? {
            Some(value) => parse_bool(&value).ok_or_else(|| {
                LoggerServiceError::InvalidConfig(format!(
                    "Invalid TRACING_OTLP_ENABLED `{value}`. Use one of: true, false, 1, 0, yes, no"
                ))
            })?,
            None => false,
        };

        let otlp_endpoint = EnvConfig::get_optional("OTEL_EXPORTER_OTLP_ENDPOINT")?
            .unwrap_or_else(|| String::from("http://otel-collector:4317"));

        let sample_ratio = match EnvConfig::get_optional_parsed::<f64>("TRACING_SAMPLE_RATIO")? {
            Some(value) if (0.0..=1.0).contains(&value) => value,
            Some(value) => {
                return Err(LoggerServiceError::InvalidConfig(format!(
                    "Invalid TRACING_SAMPLE_RATIO `{value}`. Value must be between 0.0 and 1.0"
                )));
            }
            None => 1.0,
        };

        Ok(Self {
            level,
            output,
            format,
            file_path,
            tracing: TracingConfig {
                otlp_enabled,
                otlp_endpoint,
                sample_ratio,
            },
        })
    }
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "y" | "on" => Some(true),
        "false" | "0" | "no" | "n" | "off" => Some(false),
        _ => None,
    }
}

pub struct LoggerGuard {
    _guards: Vec<WorkerGuard>,
    _otlp_enabled: bool,
}

impl Drop for LoggerGuard {
    fn drop(&mut self) {
        if self._otlp_enabled {
            opentelemetry::global::shutdown_tracer_provider();
        }
    }
}

pub struct LoggerService;

impl LoggerService {
    pub fn init_from_env(service_name: &str) -> Result<LoggerGuard, LoggerServiceError> {
        let config = LoggerConfig::from_env()?;
        Self::init(service_name, config)
    }

    pub fn init(service_name: &str, config: LoggerConfig) -> Result<LoggerGuard, LoggerServiceError> {
        let mut guards = Vec::new();
        let mut file_writer = None;
        let otlp_enabled = config.tracing.otlp_enabled;

        if config.output.includes_file() {
            let file_path = Path::new(&config.file_path);
            let file_name = file_path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| {
                    LoggerServiceError::InvalidConfig(format!(
                        "Invalid LOG_FILE_PATH `{}`",
                        config.file_path
                    ))
                })?;

            let directory = file_path
                .parent()
                .filter(|parent| !parent.as_os_str().is_empty())
                .unwrap_or(Path::new("."));

            fs::create_dir_all(directory)?;

            let appender = tracing_appender::rolling::never(directory, file_name);
            let (non_blocking, guard) = tracing_appender::non_blocking(appender);
            file_writer = Some(non_blocking);
            guards.push(guard);
        }

        let level_filter = config.level.as_level_filter();

        match config.format {
            LogFormat::Plain => {
                let otel_tracer = Self::build_otlp_tracer(service_name, &config.tracing)?;

                let console_layer = config.output.includes_console().then(|| {
                    tracing_subscriber::fmt::layer()
                        .with_filter(level_filter)
                });

                let file_layer = file_writer.map(|writer| {
                    tracing_subscriber::fmt::layer()
                        .with_ansi(false)
                        .with_writer(writer)
                        .with_filter(level_filter)
                });

                let subscriber = tracing_subscriber::registry()
                    .with(console_layer)
                    .with(file_layer);

                if let Some(tracer) = otel_tracer {
                    subscriber
                        .with(tracing_opentelemetry::layer().with_tracer(tracer))
                        .try_init()?;
                } else {
                    subscriber.try_init()?;
                }
            }
            LogFormat::Json => {
                let otel_tracer = Self::build_otlp_tracer(service_name, &config.tracing)?;

                let console_layer = config.output.includes_console().then(|| {
                    BunyanFormattingLayer::new(service_name.to_string(), std::io::stdout)
                        .with_filter(level_filter)
                });

                let file_layer = file_writer.map(|writer| {
                    BunyanFormattingLayer::new(service_name.to_string(), writer)
                        .with_filter(level_filter)
                });

                let subscriber = tracing_subscriber::registry()
                    .with(JsonStorageLayer)
                    .with(console_layer)
                    .with(file_layer);

                if let Some(tracer) = otel_tracer {
                    subscriber
                        .with(tracing_opentelemetry::layer().with_tracer(tracer))
                        .try_init()?;
                } else {
                    subscriber.try_init()?;
                }
            }
        }

        Ok(LoggerGuard {
            _guards: guards,
            _otlp_enabled: otlp_enabled,
        })
    }

    fn build_otlp_tracer(
        default_service_name: &str,
        tracing_config: &TracingConfig,
    ) -> Result<Option<opentelemetry_sdk::trace::Tracer>, LoggerServiceError> {
        if !tracing_config.otlp_enabled {
            return Ok(None);
        }

        if tracing_config.otlp_endpoint.trim().is_empty() {
            return Err(LoggerServiceError::InvalidConfig(
                "OTEL_EXPORTER_OTLP_ENDPOINT cannot be empty when TRACING_OTLP_ENABLED is true"
                    .to_string(),
            ));
        }

        let service_name = EnvConfig::get_optional("OTEL_SERVICE_NAME")?
            .unwrap_or_else(|| default_service_name.to_string());

        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_endpoint(tracing_config.otlp_endpoint.clone())
            .build()
            .map_err(|err| LoggerServiceError::Otel(err.to_string()))?;

        let provider = SdkTracerProvider::builder()
            .with_sampler(Sampler::TraceIdRatioBased(tracing_config.sample_ratio))
            .with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                service_name.clone(),
            )]))
            .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
            .build();

        let tracer = provider.tracer(service_name);
        opentelemetry::global::set_tracer_provider(provider);

        Ok(Some(tracer))
    }
}

#[derive(Debug)]
pub enum LoggerServiceError {
    Env(EnvConfigError),
    InvalidConfig(String),
    Io(std::io::Error),
    Init(tracing_subscriber::util::TryInitError),
    Otel(String),
}

impl fmt::Display for LoggerServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Env(err) => write!(f, "{err}"),
            Self::InvalidConfig(message) => write!(f, "{message}"),
            Self::Io(err) => write!(f, "{err}"),
            Self::Init(err) => write!(f, "{err}"),
            Self::Otel(err) => write!(f, "{err}"),
        }
    }
}

impl Error for LoggerServiceError {}

impl From<EnvConfigError> for LoggerServiceError {
    fn from(value: EnvConfigError) -> Self {
        Self::Env(value)
    }
}

impl From<std::io::Error> for LoggerServiceError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<tracing_subscriber::util::TryInitError> for LoggerServiceError {
    fn from(value: tracing_subscriber::util::TryInitError) -> Self {
        Self::Init(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_bool, LogFormat, LogLevel, LogOutput};

    #[test]
    fn parses_log_level_values() {
        assert!(matches!(LogLevel::from_str("info"), Some(LogLevel::Info)));
        assert!(matches!(LogLevel::from_str("DEBUG"), Some(LogLevel::Debug)));
        assert!(LogLevel::from_str("trace").is_none());
    }

    #[test]
    fn parses_log_output_values() {
        assert!(matches!(LogOutput::from_str("console"), Some(LogOutput::Console)));
        assert!(matches!(LogOutput::from_str("file"), Some(LogOutput::File)));
        assert!(matches!(LogOutput::from_str("both"), Some(LogOutput::Both)));
        assert!(LogOutput::from_str("stdout").is_none());
    }

    #[test]
    fn parses_log_format_values() {
        assert!(matches!(LogFormat::from_str("plain"), Some(LogFormat::Plain)));
        assert!(matches!(LogFormat::from_str("json"), Some(LogFormat::Json)));
        assert!(matches!(LogFormat::from_str("bunyan"), Some(LogFormat::Json)));
        assert!(LogFormat::from_str("pretty").is_none());
    }

    #[test]
    fn parses_boolean_values() {
        assert_eq!(parse_bool("true"), Some(true));
        assert_eq!(parse_bool("1"), Some(true));
        assert_eq!(parse_bool("yes"), Some(true));
        assert_eq!(parse_bool("false"), Some(false));
        assert_eq!(parse_bool("0"), Some(false));
        assert_eq!(parse_bool("no"), Some(false));
        assert_eq!(parse_bool("maybe"), None);
    }
}
