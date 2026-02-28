use std::sync::{RwLock, OnceLock};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::characters::{
    CharacterCreationCatalog,
};

#[derive(Debug, Clone)]
struct CachedCatalog {
    version: String,
    catalog: CharacterCreationCatalog,
}

static CATALOG_CACHE: OnceLock<RwLock<Option<CachedCatalog>>> = OnceLock::new();

pub struct CatalogCache;

impl CatalogCache {
    fn cache() -> &'static RwLock<Option<CachedCatalog>> {
        CATALOG_CACHE.get_or_init(|| RwLock::new(None))
    }

    /// Generate a version hash from the catalog data
    fn generate_version(catalog: &CharacterCreationCatalog) -> String {
        let mut hasher = DefaultHasher::new();
        
        // Hash all the catalog data
        for race in &catalog.races {
            race.race.hash(&mut hasher);
            race.name.hash(&mut hasher);
        }
        for gender in &catalog.genders {
            gender.gender.hash(&mut hasher);
            gender.name.hash(&mut hasher);
        }
        for skin_color in &catalog.skin_colors {
            skin_color.skin_color.hash(&mut hasher);
            skin_color.name.hash(&mut hasher);
        }
        for class in &catalog.classes {
            class.character_class.hash(&mut hasher);
            class.name.hash(&mut hasher);
        }
        for combo in &catalog.allowed_race_gender {
            combo.race.hash(&mut hasher);
            combo.gender.hash(&mut hasher);
        }
        for combo in &catalog.allowed_race_gender_skin_color {
            combo.race.hash(&mut hasher);
            combo.gender.hash(&mut hasher);
            combo.skin_color.hash(&mut hasher);
        }
        for combo in &catalog.allowed_race_gender_class {
            combo.race.hash(&mut hasher);
            combo.gender.hash(&mut hasher);
            combo.character_class.hash(&mut hasher);
        }
        
        format!("{:x}", hasher.finish())
    }

    /// Get the cached catalog and its version
    pub fn get() -> Option<(String, CharacterCreationCatalog)> {
        let cache = Self::cache();
        let guard = cache.read().ok()?;
        
        match guard.as_ref() {
            Some(cached) => Some((cached.version.clone(), cached.catalog.clone())),
            None => None,
        }
    }

    /// Update the cache with new catalog data
    pub fn set(catalog: CharacterCreationCatalog) -> String {
        let mut catalog = catalog;
        let version = Self::generate_version(&catalog);
        catalog.version = version.clone();
        
        let cache = Self::cache();
        if let Ok(mut guard) = cache.write() {
            *guard = Some(CachedCatalog {
                version: version.clone(),
                catalog,
            });
        }
        
        version
    }

    /// Get the current catalog version from cache
    pub fn current_version() -> Option<String> {
        let cache = Self::cache();
        let guard = cache.read().ok()?;
        guard.as_ref().map(|cached| cached.version.clone())
    }

    /// Invalidate the catalog cache (called when admin modifies catalog data)
    pub fn invalidate() {
        let cache = Self::cache();
        if let Ok(mut guard) = cache.write() {
            *guard = None;
        }
    }
}
