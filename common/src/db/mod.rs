use sqlx::migrate::Migrator;
use sqlx::postgres::{PgPool, PgPoolOptions};

type DbError = Box<dyn std::error::Error>;

pub async fn connect_pool(database_url: &str, max_connections: u32) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await
}

pub async fn run_migrations(pool: &PgPool, migrator: &Migrator) -> Result<(), DbError> {
    migrator.run(pool).await?;
    Ok(())
}

pub async fn current_migration_version(pool: &PgPool) -> Result<Option<i64>, DbError> {
    let version = sqlx::query_scalar::<_, Option<i64>>("SELECT MAX(version) FROM _sqlx_migrations")
        .fetch_one(pool)
        .await?;
    Ok(version)
}

pub async fn migrate_and_get_version(
    pool: &PgPool,
    migrator: &Migrator,
) -> Result<Option<i64>, DbError> {
    run_migrations(pool, migrator).await?;
    current_migration_version(pool).await
}
