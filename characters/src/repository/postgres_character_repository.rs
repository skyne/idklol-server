use async_trait::async_trait;
use sqlx::PgPool;
use crate::repository::character_repository::CharacterRepository;
use crate::models::character::Character;

#[derive(Debug, Clone)]
pub struct PostgresCharacterRepository {
    pool: PgPool,
}

impl PostgresCharacterRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CharacterRepository for PostgresCharacterRepository {
    async fn name_exists(&self, name: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let result: (bool,) = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM characters WHERE name = $1)"
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(result.0)
    }
    
    async fn create_character(
        &self,
        name: &str,
        user_email: &str,
        race_id: i32,
        gender_id: i32,
        skin_color_id: i32,
        class_id: i32,
    ) -> Result<Character, Box<dyn std::error::Error + Send + Sync>> {
        let character = sqlx::query_as::<_, Character>(
            r#"
            INSERT INTO characters (name, user_email, race_id, gender_id, skin_color_id, class_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, name, user_email, race_id, gender_id, skin_color_id, class_id, created_at
            "#
        )
        .bind(name)
        .bind(user_email)
        .bind(race_id)
        .bind(gender_id)
        .bind(skin_color_id)
        .bind(class_id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(character)
    }
    
    async fn list_characters_by_user(&self, user_email: &str) -> Result<Vec<Character>, Box<dyn std::error::Error + Send + Sync>> {
        let characters = sqlx::query_as::<_, Character>(
            r#"
            SELECT id, name, user_email, race_id, gender_id, skin_color_id, class_id, created_at
            FROM characters
            WHERE user_email = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(user_email)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(characters)
    }
}
