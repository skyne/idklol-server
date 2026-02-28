// Characters core library
// Contains all business logic, models, repositories, and services
// Used by both the gRPC server and admin NATS server

pub mod characters {
    tonic::include_proto!("characters");
}

pub mod models;
pub mod repository;
pub mod services;

// Re-export commonly used types
pub use models::catalog::*;
pub use models::character::*;
pub use repository::{CatalogRepository, CharacterRepository, PostgresCatalogRepository, PostgresCharacterRepository};

// Migrator for database operations
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
