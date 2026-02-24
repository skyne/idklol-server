use sqlx::FromRow;
use serde::{Deserialize, Serialize};

/// Race model representing a playable race
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Race {
    pub id: i32,
    pub name: String,
}

/// Gender model representing character gender
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Gender {
    pub id: i32,
    pub name: String,
}

/// SkinColor model representing available skin tones
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SkinColor {
    pub id: i32,
    pub name: String,
}

/// Class model representing character classes
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Class {
    pub id: i32,
    pub name: String,
}

/// Junction table model for allowed race-gender combinations
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct RaceGenderAllowed {
    pub race_id: i32,
    pub gender_id: i32,
}

/// Junction table model for allowed race-gender-skin combinations
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct RaceGenderSkinColorAllowed {
    pub race_id: i32,
    pub gender_id: i32,
    pub skin_color_id: i32,
}

/// Junction table model for allowed race-gender-class combinations
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct RaceGenderClassAllowed {
    pub race_id: i32,
    pub gender_id: i32,
    pub class_id: i32,
}
