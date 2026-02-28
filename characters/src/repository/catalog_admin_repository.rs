use async_trait::async_trait;
use crate::models::catalog::{Race, Gender, SkinColor, Class};

#[async_trait]
pub trait CatalogAdminRepository: Send + Sync {
    // Race operations
    async fn create_race(&self, name: &str) -> Result<Race, Box<dyn std::error::Error + Send + Sync>>;
    async fn update_race(&self, id: i32, name: &str) -> Result<Race, Box<dyn std::error::Error + Send + Sync>>;
    async fn delete_race(&self, id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    // Gender operations
    async fn create_gender(&self, name: &str) -> Result<Gender, Box<dyn std::error::Error + Send + Sync>>;
    async fn update_gender(&self, id: i32, name: &str) -> Result<Gender, Box<dyn std::error::Error + Send + Sync>>;
    async fn delete_gender(&self, id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    // Skin color operations
    async fn create_skin_color(&self, name: &str) -> Result<SkinColor, Box<dyn std::error::Error + Send + Sync>>;
    async fn update_skin_color(&self, id: i32, name: &str) -> Result<SkinColor, Box<dyn std::error::Error + Send + Sync>>;
    async fn delete_skin_color(&self, id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    // Class operations
    async fn create_class(&self, name: &str) -> Result<Class, Box<dyn std::error::Error + Send + Sync>>;
    async fn update_class(&self, id: i32, name: &str) -> Result<Class, Box<dyn std::error::Error + Send + Sync>>;
    async fn delete_class(&self, id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    // Combination operations
    async fn set_race_gender_allowed(&self, race_id: i32, gender_id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn remove_race_gender_allowed(&self, race_id: i32, gender_id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    async fn set_race_gender_skin_color_allowed(&self, race_id: i32, gender_id: i32, skin_color_id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn remove_race_gender_skin_color_allowed(&self, race_id: i32, gender_id: i32, skin_color_id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    async fn set_race_gender_class_allowed(&self, race_id: i32, gender_id: i32, class_id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn remove_race_gender_class_allowed(&self, race_id: i32, gender_id: i32, class_id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
