use tonic::{Request, Response, Status};
use std::sync::Arc;
use tracing::{debug, error, info};
use crate::characters::{
    CheckCharacterCreationCatalogVersionRequest, CheckCharacterCreationCatalogVersionResponse,
    CharacterCreationCatalog, RaceOption, GenderOption, SkinColorOption, ClassOption,
    RaceGenderCombination, RaceGenderSkinColorCombination, RaceGenderClassCombination,
};
use crate::repository::CatalogRepository;
use crate::services::catalog_cache::CatalogCache;

/// Service for handling catalog-related operations
/// These operations are read-only and do not require authentication
pub struct CatalogService {
    catalog_repo: Arc<dyn CatalogRepository>,
}

impl CatalogService {
    pub fn new(catalog_repo: Arc<dyn CatalogRepository>) -> Self {
        Self { catalog_repo }
    }

    pub async fn check_character_creation_catalog_version(
        &self,
        request: Request<CheckCharacterCreationCatalogVersionRequest>,
    ) -> Result<Response<CheckCharacterCreationCatalogVersionResponse>, Status> {
        let cached_version = request.into_inner().cached_version;
        debug!(cached_version = %cached_version, "checking character creation catalog version");
        let latest_version = CatalogCache::current_version().unwrap_or_default();
        let is_current = !cached_version.is_empty() && !latest_version.is_empty() && cached_version == latest_version;
        if is_current {
            info!(cached_version = %cached_version, "client cache is current");
        }
        Ok(Response::new(CheckCharacterCreationCatalogVersionResponse {
            is_current,
            latest_version,
        }))
    }

    pub async fn get_character_creation_catalog(
        &self,
        request: Request<()>,
    ) -> Result<Response<CharacterCreationCatalog>, Status> {
        debug!("fetching character creation catalog");
        // ...existing code...
        if let Some((version, catalog)) = CatalogCache::get() {
            debug!(version = %version, "returning cached catalog");
            return Ok(Response::new(catalog));
        }
        info!("building character creation catalog from database");
        let races_future = self.catalog_repo.get_races();
        let genders_future = self.catalog_repo.get_all_genders();
        let skin_colors_future = self.catalog_repo.get_all_skin_colors();
        let classes_future = self.catalog_repo.get_all_classes();
        let race_gender_future = self.catalog_repo.get_allowed_race_gender();
        let race_gender_skin_future = self.catalog_repo.get_allowed_race_gender_skin_color();
        let race_gender_class_future = self.catalog_repo.get_allowed_race_gender_class();
        let (
            races_result,
            genders_result,
            skin_colors_result,
            classes_result,
            race_gender_result,
            race_gender_skin_result,
            race_gender_class_result,
        ) = tokio::join!(
            races_future,
            genders_future,
            skin_colors_future,
            classes_future,
            race_gender_future,
            race_gender_skin_future,
            race_gender_class_future,
        );
        let races = races_result.map_err(|err| {
            error!(error = %err, "failed to fetch races");
            Status::internal("failed to fetch catalog data")
        })?;
        let genders = genders_result.map_err(|err| {
            error!(error = %err, "failed to fetch genders");
            Status::internal("failed to fetch catalog data")
        })?;
        let skin_colors = skin_colors_result.map_err(|err| {
            error!(error = %err, "failed to fetch skin colors");
            Status::internal("failed to fetch catalog data")
        })?;
        let classes = classes_result.map_err(|err| {
            error!(error = %err, "failed to fetch classes");
            Status::internal("failed to fetch catalog data")
        })?;
        let race_gender = race_gender_result.map_err(|err| {
            error!(error = %err, "failed to fetch race-gender combinations");
            Status::internal("failed to fetch catalog data")
        })?;
        let race_gender_skin = race_gender_skin_result.map_err(|err| {
            error!(error = %err, "failed to fetch race-gender-skin combinations");
            Status::internal("failed to fetch catalog data")
        })?;
        let race_gender_class = race_gender_class_result.map_err(|err| {
            error!(error = %err, "failed to fetch race-gender-class combinations");
            Status::internal("failed to fetch catalog data")
        })?;
        let catalog = CharacterCreationCatalog {
            version: String::new(), // Will be set by cache
            races: races.into_iter().map(|r| RaceOption {
                race: r.id,
                name: r.name,
            }).collect(),
            genders: genders.into_iter().map(|g| GenderOption {
                gender: g.id,
                name: g.name,
            }).collect(),
            skin_colors: skin_colors.into_iter().map(|sc| SkinColorOption {
                skin_color: sc.id,
                name: sc.name,
            }).collect(),
            classes: classes.into_iter().map(|c| ClassOption {
                character_class: c.id,
                name: c.name,
            }).collect(),
            allowed_race_gender: race_gender.into_iter().map(|rg| RaceGenderCombination {
                race: rg.race_id,
                gender: rg.gender_id,
            }).collect(),
            allowed_race_gender_skin_color: race_gender_skin.into_iter().map(|rgs| RaceGenderSkinColorCombination {
                race: rgs.race_id,
                gender: rgs.gender_id,
                skin_color: rgs.skin_color_id,
            }).collect(),
            allowed_race_gender_class: race_gender_class.into_iter().map(|rgc| RaceGenderClassCombination {
                race: rgc.race_id,
                gender: rgc.gender_id,
                character_class: rgc.class_id,
            }).collect(),
        };
        let version = CatalogCache::set(catalog.clone());
        info!(version = %version, "catalog cached");
        Ok(Response::new(catalog))
    }
}
