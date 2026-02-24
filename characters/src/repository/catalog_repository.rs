use async_trait::async_trait;
use crate::models::catalog::{Race, Gender, SkinColor, Class, RaceGenderAllowed, RaceGenderSkinColorAllowed, RaceGenderClassAllowed};

#[async_trait]
pub trait CatalogRepository: Send + Sync {
    /// Get all available races
    async fn get_races(&self) -> Result<Vec<Race>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Get all genders
    async fn get_all_genders(&self) -> Result<Vec<Gender>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Get all skin colors
    async fn get_all_skin_colors(&self) -> Result<Vec<SkinColor>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Get all classes
    async fn get_all_classes(&self) -> Result<Vec<Class>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Get all allowed race-gender combinations
    async fn get_allowed_race_gender(&self) -> Result<Vec<RaceGenderAllowed>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Get all allowed race-gender-skin color combinations
    async fn get_allowed_race_gender_skin_color(&self) -> Result<Vec<RaceGenderSkinColorAllowed>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Get all allowed race-gender-class combinations
    async fn get_allowed_race_gender_class(&self) -> Result<Vec<RaceGenderClassAllowed>, Box<dyn std::error::Error + Send + Sync>>;
}
