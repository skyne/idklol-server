use async_trait::async_trait;
use sqlx::PgPool;
use crate::repository::catalog_admin_repository::CatalogAdminRepository;
use crate::models::catalog::{Race, Gender, SkinColor, Class};

#[derive(Debug, Clone)]
pub struct PostgresCatalogAdminRepository {
    pool: PgPool,
}

impl PostgresCatalogAdminRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CatalogAdminRepository for PostgresCatalogAdminRepository {
    // Race operations
    async fn create_race(&self, name: &str) -> Result<Race, Box<dyn std::error::Error + Send + Sync>> {
        let race = sqlx::query_as::<_, Race>(
            "INSERT INTO races (name) VALUES ($1) RETURNING id, name"
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(race)
    }
    
    async fn update_race(&self, id: i32, name: &str) -> Result<Race, Box<dyn std::error::Error + Send + Sync>> {
        let race = sqlx::query_as::<_, Race>(
            "UPDATE races SET name = $1 WHERE id = $2 RETURNING id, name"
        )
        .bind(name)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(race)
    }
    
    async fn delete_race(&self, id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query("DELETE FROM races WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    // Gender operations
    async fn create_gender(&self, name: &str) -> Result<Gender, Box<dyn std::error::Error + Send + Sync>> {
        let gender = sqlx::query_as::<_, Gender>(
            "INSERT INTO genders (name) VALUES ($1) RETURNING id, name"
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(gender)
    }
    
    async fn update_gender(&self, id: i32, name: &str) -> Result<Gender, Box<dyn std::error::Error + Send + Sync>> {
        let gender = sqlx::query_as::<_, Gender>(
            "UPDATE genders SET name = $1 WHERE id = $2 RETURNING id, name"
        )
        .bind(name)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(gender)
    }
    
    async fn delete_gender(&self, id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query("DELETE FROM genders WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    // Skin color operations
    async fn create_skin_color(&self, name: &str) -> Result<SkinColor, Box<dyn std::error::Error + Send + Sync>> {
        let skin_color = sqlx::query_as::<_, SkinColor>(
            "INSERT INTO skin_colors (name) VALUES ($1) RETURNING id, name"
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(skin_color)
    }
    
    async fn update_skin_color(&self, id: i32, name: &str) -> Result<SkinColor, Box<dyn std::error::Error + Send + Sync>> {
        let skin_color = sqlx::query_as::<_, SkinColor>(
            "UPDATE skin_colors SET name = $1 WHERE id = $2 RETURNING id, name"
        )
        .bind(name)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(skin_color)
    }
    
    async fn delete_skin_color(&self, id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query("DELETE FROM skin_colors WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    // Class operations
    async fn create_class(&self, name: &str) -> Result<Class, Box<dyn std::error::Error + Send + Sync>> {
        let class = sqlx::query_as::<_, Class>(
            "INSERT INTO classes (name) VALUES ($1) RETURNING id, name"
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(class)
    }
    
    async fn update_class(&self, id: i32, name: &str) -> Result<Class, Box<dyn std::error::Error + Send + Sync>> {
        let class = sqlx::query_as::<_, Class>(
            "UPDATE classes SET name = $1 WHERE id = $2 RETURNING id, name"
        )
        .bind(name)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(class)
    }
    
    async fn delete_class(&self, id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query("DELETE FROM classes WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    // Combination operations
    async fn set_race_gender_allowed(&self, race_id: i32, gender_id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query(
            "INSERT INTO race_gender_allowed (race_id, gender_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
        )
        .bind(race_id)
        .bind(gender_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn remove_race_gender_allowed(&self, race_id: i32, gender_id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query(
            "DELETE FROM race_gender_allowed WHERE race_id = $1 AND gender_id = $2"
        )
        .bind(race_id)
        .bind(gender_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn set_race_gender_skin_color_allowed(&self, race_id: i32, gender_id: i32, skin_color_id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query(
            "INSERT INTO race_gender_skin_color_allowed (race_id, gender_id, skin_color_id) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"
        )
        .bind(race_id)
        .bind(gender_id)
        .bind(skin_color_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn remove_race_gender_skin_color_allowed(&self, race_id: i32, gender_id: i32, skin_color_id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query(
            "DELETE FROM race_gender_skin_color_allowed WHERE race_id = $1 AND gender_id = $2 AND skin_color_id = $3"
        )
        .bind(race_id)
        .bind(gender_id)
        .bind(skin_color_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn set_race_gender_class_allowed(&self, race_id: i32, gender_id: i32, class_id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query(
            "INSERT INTO race_gender_class_allowed (race_id, gender_id, class_id) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"
        )
        .bind(race_id)
        .bind(gender_id)
        .bind(class_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn remove_race_gender_class_allowed(&self, race_id: i32, gender_id: i32, class_id: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query(
            "DELETE FROM race_gender_class_allowed WHERE race_id = $1 AND gender_id = $2 AND class_id = $3"
        )
        .bind(race_id)
        .bind(gender_id)
        .bind(class_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}
