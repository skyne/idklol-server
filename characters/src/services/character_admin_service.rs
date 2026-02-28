use std::sync::Arc;
use crate::repository::{CharacterAdminRepository, PostgresCharacterAdminRepository};
use crate::models::character::Character;
use tracing::info;

pub struct CharacterAdminService {
    character_admin_repo: Arc<dyn CharacterAdminRepository>,
}

impl CharacterAdminService {
    pub fn new(character_admin_repo: Arc<dyn CharacterAdminRepository>) -> Self {
        Self { character_admin_repo }
    }

    pub fn with_pool(pool: sqlx::PgPool) -> Self {
        let character_admin_repo = Arc::new(PostgresCharacterAdminRepository::new(pool));
        Self::new(character_admin_repo)
    }

    pub async fn list_all_characters(&self) -> Result<Vec<Character>, String> {
        self.character_admin_repo.list_all_characters()
            .await
            .map_err(|e| format!("Failed to list all characters: {}", e))
    }

    pub async fn get_character_by_id(&self, id: uuid::Uuid) -> Result<Option<Character>, String> {
        self.character_admin_repo.get_character_by_id(id)
            .await
            .map_err(|e| format!("Failed to get character by ID: {}", e))
    }

    pub async fn delete_character(&self, id: uuid::Uuid) -> Result<(), String> {
        info!("Admin deleting character: {}", id);
        self.character_admin_repo.delete_character(id)
            .await
            .map_err(|e| format!("Failed to delete character: {}", e))
    }

    pub async fn update_character(
        &self,
        id: uuid::Uuid,
        name: &str,
        race_id: i32,
        gender_id: i32,
        skin_color_id: i32,
        class_id: i32,
    ) -> Result<Character, String> {
        info!("Admin updating character: {}", id);
        self.character_admin_repo.update_character(id, name, race_id, gender_id, skin_color_id, class_id)
            .await
            .map_err(|e| format!("Failed to update character: {}", e))
    }
}
