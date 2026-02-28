use async_trait::async_trait;
use crate::models::character::Character;

#[async_trait]
pub trait CharacterAdminRepository: Send + Sync {
    /// List all characters (admin only - not scoped to user)
    async fn list_all_characters(&self) -> Result<Vec<Character>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Get a character by ID (admin only - not scoped to user)
    async fn get_character_by_id(&self, id: uuid::Uuid) -> Result<Option<Character>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Delete any character by ID (admin only)
    async fn delete_character(&self, id: uuid::Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    /// Update a character's attributes (admin only)
    async fn update_character(
        &self,
        id: uuid::Uuid,
        name: &str,
        race_id: i32,
        gender_id: i32,
        skin_color_id: i32,
        class_id: i32,
    ) -> Result<Character, Box<dyn std::error::Error + Send + Sync>>;
}
