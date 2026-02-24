use async_trait::async_trait;
use crate::models::character::Character;

#[async_trait]
pub trait CharacterRepository: Send + Sync {
    /// Check if a character name already exists (global uniqueness check)
    async fn name_exists(&self, name: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Create a new character
    async fn create_character(
        &self,
        name: &str,
        user_email: &str,
        race_id: i32,
        gender_id: i32,
        skin_color_id: i32,
        class_id: i32,
    ) -> Result<Character, Box<dyn std::error::Error + Send + Sync>>;
    
    /// List all characters created by a specific user
    async fn list_characters_by_user(&self, user_email: &str) -> Result<Vec<Character>, Box<dyn std::error::Error + Send + Sync>>;
}
