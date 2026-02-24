use async_trait::async_trait;
use sqlx::PgPool;
use crate::repository::catalog_repository::CatalogRepository;
use crate::models::catalog::{Race, Gender, SkinColor, Class, RaceGenderAllowed, RaceGenderSkinColorAllowed, RaceGenderClassAllowed};

#[derive(Debug, Clone)]
pub struct PostgresCatalogRepository {
    pool: PgPool,
}

impl PostgresCatalogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CatalogRepository for PostgresCatalogRepository {
    async fn get_races(&self) -> Result<Vec<Race>, Box<dyn std::error::Error + Send + Sync>> {
        let races = sqlx::query_as::<_, Race>(
            "SELECT id, name FROM races ORDER BY id"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(races)
    }
    
    async fn get_all_genders(&self) -> Result<Vec<Gender>, Box<dyn std::error::Error + Send + Sync>> {
        let genders = sqlx::query_as::<_, Gender>(
            "SELECT id, name FROM genders ORDER BY id"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(genders)
    }
    
    async fn get_all_skin_colors(&self) -> Result<Vec<SkinColor>, Box<dyn std::error::Error + Send + Sync>> {
        let skin_colors = sqlx::query_as::<_, SkinColor>(
            "SELECT id, name FROM skin_colors ORDER BY id"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(skin_colors)
    }
    
    async fn get_all_classes(&self) -> Result<Vec<Class>, Box<dyn std::error::Error + Send + Sync>> {
        let classes = sqlx::query_as::<_, Class>(
            "SELECT id, name FROM classes ORDER BY id"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(classes)
    }
    
    async fn get_allowed_race_gender(&self) -> Result<Vec<RaceGenderAllowed>, Box<dyn std::error::Error + Send + Sync>> {
        let combinations = sqlx::query_as::<_, RaceGenderAllowed>(
            "SELECT race_id, gender_id FROM race_gender_allowed ORDER BY race_id, gender_id"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(combinations)
    }
    
    async fn get_allowed_race_gender_skin_color(&self) -> Result<Vec<RaceGenderSkinColorAllowed>, Box<dyn std::error::Error + Send + Sync>> {
        let combinations = sqlx::query_as::<_, RaceGenderSkinColorAllowed>(
            "SELECT race_id, gender_id, skin_color_id FROM race_gender_skin_color_allowed ORDER BY race_id, gender_id, skin_color_id"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(combinations)
    }
    
    async fn get_allowed_race_gender_class(&self) -> Result<Vec<RaceGenderClassAllowed>, Box<dyn std::error::Error + Send + Sync>> {
        let combinations = sqlx::query_as::<_, RaceGenderClassAllowed>(
            "SELECT race_id, gender_id, class_id FROM race_gender_class_allowed ORDER BY race_id, gender_id, class_id"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(combinations)
    }
}
