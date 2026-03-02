use std::sync::Arc;
use crate::repository::{CatalogAdminRepository, CatalogRepository, PostgresCatalogAdminRepository, PostgresCatalogRepository};
use crate::models::catalog::{Race, Gender, SkinColor, Class, RaceGenderAllowed, RaceGenderSkinColorAllowed, RaceGenderClassAllowed};
use crate::services::catalog_cache::CatalogCache;
use tracing::info;

pub struct CatalogAdminService {
    catalog_admin_repo: Arc<dyn CatalogAdminRepository>,
    catalog_repo: Arc<dyn CatalogRepository>,
}

impl CatalogAdminService {
    pub fn new(catalog_admin_repo: Arc<dyn CatalogAdminRepository>, catalog_repo: Arc<dyn CatalogRepository>) -> Self {
        Self { catalog_admin_repo, catalog_repo }
    }

    pub fn with_pool(pool: sqlx::PgPool) -> Self {
        let catalog_admin_repo = Arc::new(PostgresCatalogAdminRepository::new(pool.clone()));
        let catalog_repo = Arc::new(PostgresCatalogRepository::new(pool));
        Self::new(catalog_admin_repo, catalog_repo)
    }

    // Helper to invalidate cache after mutations
    fn invalidate_cache() {
        info!("Invalidating catalog cache due to admin mutation");
        CatalogCache::invalidate();
    }

    // Race operations
    pub async fn create_race(&self, name: &str) -> Result<Race, String> {
        let race = self.catalog_admin_repo.create_race(name)
            .await
            .map_err(|e| format!("Failed to create race: {}", e))?;
        
        Self::invalidate_cache();
        Ok(race)
    }

    pub async fn update_race(&self, id: i32, name: &str) -> Result<Race, String> {
        let race = self.catalog_admin_repo.update_race(id, name)
            .await
            .map_err(|e| format!("Failed to update race: {}", e))?;
        
        Self::invalidate_cache();
        Ok(race)
    }

    pub async fn delete_race(&self, id: i32) -> Result<(), String> {
        self.catalog_admin_repo.delete_race(id)
            .await
            .map_err(|e| format!("Failed to delete race: {}", e))?;
        
        Self::invalidate_cache();
        Ok(())
    }

    // Gender operations
    pub async fn create_gender(&self, name: &str) -> Result<Gender, String> {
        let gender = self.catalog_admin_repo.create_gender(name)
            .await
            .map_err(|e| format!("Failed to create gender: {}", e))?;
        
        Self::invalidate_cache();
        Ok(gender)
    }

    pub async fn update_gender(&self, id: i32, name: &str) -> Result<Gender, String> {
        let gender = self.catalog_admin_repo.update_gender(id, name)
            .await
            .map_err(|e| format!("Failed to update gender: {}", e))?;
        
        Self::invalidate_cache();
        Ok(gender)
    }

    pub async fn delete_gender(&self, id: i32) -> Result<(), String> {
        self.catalog_admin_repo.delete_gender(id)
            .await
            .map_err(|e| format!("Failed to delete gender: {}", e))?;
        
        Self::invalidate_cache();
        Ok(())
    }

    // Skin color operations
    pub async fn create_skin_color(&self, name: &str) -> Result<SkinColor, String> {
        let skin_color = self.catalog_admin_repo.create_skin_color(name)
            .await
            .map_err(|e| format!("Failed to create skin color: {}", e))?;
        
        Self::invalidate_cache();
        Ok(skin_color)
    }

    pub async fn update_skin_color(&self, id: i32, name: &str) -> Result<SkinColor, String> {
        let skin_color = self.catalog_admin_repo.update_skin_color(id, name)
            .await
            .map_err(|e| format!("Failed to update skin color: {}", e))?;
        
        Self::invalidate_cache();
        Ok(skin_color)
    }

    pub async fn delete_skin_color(&self, id: i32) -> Result<(), String> {
        self.catalog_admin_repo.delete_skin_color(id)
            .await
            .map_err(|e| format!("Failed to delete skin color: {}", e))?;
        
        Self::invalidate_cache();
        Ok(())
    }

    // Class operations
    pub async fn create_class(&self, name: &str) -> Result<Class, String> {
        let class = self.catalog_admin_repo.create_class(name)
            .await
            .map_err(|e| format!("Failed to create class: {}", e))?;
        
        Self::invalidate_cache();
        Ok(class)
    }

    pub async fn update_class(&self, id: i32, name: &str) -> Result<Class, String> {
        let class = self.catalog_admin_repo.update_class(id, name)
            .await
            .map_err(|e| format!("Failed to update class: {}", e))?;
        
        Self::invalidate_cache();
        Ok(class)
    }

    pub async fn delete_class(&self, id: i32) -> Result<(), String> {
        self.catalog_admin_repo.delete_class(id)
            .await
            .map_err(|e| format!("Failed to delete class: {}", e))?;
        
        Self::invalidate_cache();
        Ok(())
    }

    // Combination operations
    pub async fn set_race_gender_allowed(&self, race_id: i32, gender_id: i32) -> Result<(), String> {
        self.catalog_admin_repo.set_race_gender_allowed(race_id, gender_id)
            .await
            .map_err(|e| format!("Failed to set race-gender allowed: {}", e))?;
        
        Self::invalidate_cache();
        Ok(())
    }

    pub async fn remove_race_gender_allowed(&self, race_id: i32, gender_id: i32) -> Result<(), String> {
        self.catalog_admin_repo.remove_race_gender_allowed(race_id, gender_id)
            .await
            .map_err(|e| format!("Failed to remove race-gender allowed: {}", e))?;
        
        Self::invalidate_cache();
        Ok(())
    }

    pub async fn set_race_gender_skin_color_allowed(&self, race_id: i32, gender_id: i32, skin_color_id: i32) -> Result<(), String> {
        self.catalog_admin_repo.set_race_gender_skin_color_allowed(race_id, gender_id, skin_color_id)
            .await
            .map_err(|e| format!("Failed to set race-gender-skin color allowed: {}", e))?;
        
        Self::invalidate_cache();
        Ok(())
    }

    pub async fn remove_race_gender_skin_color_allowed(&self, race_id: i32, gender_id: i32, skin_color_id: i32) -> Result<(), String> {
        self.catalog_admin_repo.remove_race_gender_skin_color_allowed(race_id, gender_id, skin_color_id)
            .await
            .map_err(|e| format!("Failed to remove race-gender-skin color allowed: {}", e))?;
        
        Self::invalidate_cache();
        Ok(())
    }

    pub async fn set_race_gender_class_allowed(&self, race_id: i32, gender_id: i32, class_id: i32) -> Result<(), String> {
        self.catalog_admin_repo.set_race_gender_class_allowed(race_id, gender_id, class_id)
            .await
            .map_err(|e| format!("Failed to set race-gender-class allowed: {}", e))?;
        
        Self::invalidate_cache();
        Ok(())
    }

    pub async fn remove_race_gender_class_allowed(&self, race_id: i32, gender_id: i32, class_id: i32) -> Result<(), String> {
        self.catalog_admin_repo.remove_race_gender_class_allowed(race_id, gender_id, class_id)
            .await
            .map_err(|e| format!("Failed to remove race-gender-class allowed: {}", e))?;
        
        Self::invalidate_cache();
        Ok(())
    }

    // List operations (read-only, for admin UI)
    pub async fn list_races(&self) -> Result<Vec<Race>, String> {
        self.catalog_repo.get_races()
            .await
            .map_err(|e| format!("Failed to list races: {}", e))
    }

    pub async fn list_genders(&self) -> Result<Vec<Gender>, String> {
        self.catalog_repo.get_all_genders()
            .await
            .map_err(|e| format!("Failed to list genders: {}", e))
    }

    pub async fn list_skin_colors(&self) -> Result<Vec<SkinColor>, String> {
        self.catalog_repo.get_all_skin_colors()
            .await
            .map_err(|e| format!("Failed to list skin colors: {}", e))
    }

    pub async fn list_classes(&self) -> Result<Vec<Class>, String> {
        self.catalog_repo.get_all_classes()
            .await
            .map_err(|e| format!("Failed to list classes: {}", e))
    }

    pub async fn list_race_gender_allowed(&self) -> Result<Vec<RaceGenderAllowed>, String> {
        self.catalog_repo.get_allowed_race_gender()
            .await
            .map_err(|e| format!("Failed to list race-gender combinations: {}", e))
    }

    pub async fn list_race_gender_skin_color_allowed(&self) -> Result<Vec<RaceGenderSkinColorAllowed>, String> {
        self.catalog_repo.get_allowed_race_gender_skin_color()
            .await
            .map_err(|e| format!("Failed to list race-gender-skin color combinations: {}", e))
    }

    pub async fn list_race_gender_class_allowed(&self) -> Result<Vec<RaceGenderClassAllowed>, String> {
        self.catalog_repo.get_allowed_race_gender_class()
            .await
            .map_err(|e| format!("Failed to list race-gender-class combinations: {}", e))
    }

    // Get current catalog version for cache validation
    pub fn get_catalog_version(&self) -> String {
        CatalogCache::current_version().unwrap_or_else(|| "none".to_string())
    }
}
