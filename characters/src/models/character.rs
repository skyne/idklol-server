use chrono::{DateTime, Utc};
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

/// Character model representing a created character
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Character {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub user_email: String,
    pub race_id: i32,
    pub gender_id: i32,
    pub skin_color_id: i32,
    pub class_id: i32,
    pub created_at: DateTime<Utc>,
}
