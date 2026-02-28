use async_trait::async_trait;
use sqlx::PgPool;
use crate::repository::character_admin_repository::CharacterAdminRepository;
use crate::models::character::Character;

#[derive(Debug, Clone)]
pub struct PostgresCharacterAdminRepository {
    pool: PgPool,
}

impl PostgresCharacterAdminRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CharacterAdminRepository for PostgresCharacterAdminRepository {
    async fn list_all_characters(&self) -> Result<Vec<Character>, Box<dyn std::error::Error + Send + Sync>> {
        let characters = sqlx::query_as::<_, Character>(
            "SELECT id, name, user_email, race_id, gender_id, skin_color_id, class_id, created_at 
             FROM characters 
             ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(characters)
    }
    
    async fn get_character_by_id(&self, id: uuid::Uuid) -> Result<Option<Character>, Box<dyn std::error::Error + Send + Sync>> {
        let character = sqlx::query_as::<_, Character>(
            "SELECT id, name, user_email, race_id, gender_id, skin_color_id, class_id, created_at 
             FROM characters 
             WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(character)
    }
    
    async fn delete_character(&self, id: uuid::Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query("DELETE FROM characters WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    async fn update_character(
        &self,
        id: uuid::Uuid,
        name: &str,
        race_id: i32,
        gender_id: i32,
        skin_color_id: i32,
        class_id: i32,
    ) -> Result<Character, Box<dyn std::error::Error + Send + Sync>> {
        let character = sqlx::query_as::<_, Character>(
            "UPDATE characters 
             SET name = $1, race_id = $2, gender_id = $3, skin_color_id = $4, class_id = $5 
             WHERE id = $6 
             RETURNING id, name, user_email, race_id, gender_id, skin_color_id, class_id, created_at"
        )
        .bind(name)
        .bind(race_id)
        .bind(gender_id)
        .bind(skin_color_id)
        .bind(class_id)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(character)
    }
}
