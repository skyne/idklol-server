mod handlers;
mod models;
mod repository;

use std::sync::Arc;

use futures::StreamExt;
use idklol_common::config::env_config::EnvConfig;
use idklol_common::db;
use idklol_common::logging::logger_service::LoggerService;
use idklol_common::runtime;
use tracing::info;

use repository::npc_repository::NpcRepository;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    EnvConfig::init_from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));
    let _logger_guard = LoggerService::init_from_env("idklol-npc-metadata-service")?;
    let retry_config = runtime::RetryConfig::from_env();

    let database_url = EnvConfig::get_required("DATABASE_URL")?;
    info!("connecting to database");
    let pool = runtime::retry_with_backoff("database connection", retry_config, || {
        db::connect_pool(&database_url, 5)
    })
    .await?;

    info!("running database migrations");
    let schema_version = runtime::retry_with_backoff("database migrations", retry_config, || {
        db::migrate_and_get_version(&pool, &MIGRATOR)
    })
    .await?;
    info!(?schema_version, "database migrations complete");

    let repo = Arc::new(NpcRepository::new(pool));

    let nats_url = EnvConfig::get_optional("NATS_URL")
        .ok()
        .flatten()
        .unwrap_or_else(|| "nats://nats:4222".to_string());

    info!(%nats_url, "connecting to NATS");
    let client = runtime::retry_with_backoff("nats connection", retry_config, || {
        async_nats::connect(&nats_url)
    })
    .await?;
    info!("connected to NATS");

    // npc.meta.get
    {
        let mut sub = client.subscribe("npc.meta.get").await?;
        let repo = repo.clone();
        let nc = client.clone();
        tokio::spawn(async move {
            info!("listening on npc.meta.get");
            while let Some(msg) = sub.next().await {
                let repo = repo.clone();
                let nc = nc.clone();
                tokio::spawn(async move {
                    handlers::meta_get::handle(msg, repo, nc).await;
                });
            }
        });
    }

    // npc.meta.batch_get
    {
        let mut sub = client.subscribe("npc.meta.batch_get").await?;
        let repo = repo.clone();
        let nc = client.clone();
        tokio::spawn(async move {
            info!("listening on npc.meta.batch_get");
            while let Some(msg) = sub.next().await {
                let repo = repo.clone();
                let nc = nc.clone();
                tokio::spawn(async move {
                    handlers::meta_batch_get::handle(msg, repo, nc).await;
                });
            }
        });
    }

    // npc.meta.by_zone
    {
        let mut sub = client.subscribe("npc.meta.by_zone").await?;
        let repo = repo.clone();
        let nc = client.clone();
        tokio::spawn(async move {
            info!("listening on npc.meta.by_zone");
            while let Some(msg) = sub.next().await {
                let repo = repo.clone();
                let nc = nc.clone();
                tokio::spawn(async move {
                    handlers::meta_by_zone::handle(msg, repo, nc).await;
                });
            }
        });
    }

    // npc.meta.list
    {
        let mut sub = client.subscribe("npc.meta.list").await?;
        let repo = repo.clone();
        let nc = client.clone();
        tokio::spawn(async move {
            info!("listening on npc.meta.list");
            while let Some(msg) = sub.next().await {
                let repo = repo.clone();
                let nc = nc.clone();
                tokio::spawn(async move {
                    handlers::meta_list::handle(msg, repo, nc).await;
                });
            }
        });
    }

    // npc.meta.upsert
    {
        let mut sub = client.subscribe("npc.meta.upsert").await?;
        let repo = repo.clone();
        let nc = client.clone();
        tokio::spawn(async move {
            info!("listening on npc.meta.upsert");
            while let Some(msg) = sub.next().await {
                let repo = repo.clone();
                let nc = nc.clone();
                tokio::spawn(async move {
                    handlers::meta_upsert::handle(msg, repo, nc).await;
                });
            }
        });
    }

    // npc.meta.delete
    {
        let mut sub = client.subscribe("npc.meta.delete").await?;
        let repo = repo.clone();
        let nc = client.clone();
        tokio::spawn(async move {
            info!("listening on npc.meta.delete");
            while let Some(msg) = sub.next().await {
                let repo = repo.clone();
                let nc = nc.clone();
                tokio::spawn(async move {
                    handlers::meta_delete::handle(msg, repo, nc).await;
                });
            }
        });
    }

    // npc.meta.resolve_context
    {
        let mut sub = client.subscribe("npc.meta.resolve_context").await?;
        let repo = repo.clone();
        let nc = client.clone();
        tokio::spawn(async move {
            info!("listening on npc.meta.resolve_context");
            while let Some(msg) = sub.next().await {
                let repo = repo.clone();
                let nc = nc.clone();
                tokio::spawn(async move {
                    handlers::meta_resolve_context::handle(msg, repo, nc).await;
                });
            }
        });
    }

    info!("npc-metadata-service is running");
    runtime::wait_for_shutdown_signal().await;
    info!("shutting down");
    Ok(())
}
