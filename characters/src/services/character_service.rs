use tonic::{Request, Response, Status};
use std::sync::Arc;
use crate::characters::{
    character_service_server::CharacterService,
    CheckCharacterCreationCatalogVersionRequest, CheckCharacterCreationCatalogVersionResponse,
    CharacterCreationCatalog,
    CreateCharacterRequest, CreateCharacterResponse,
    ListCreatedCharactersRequest, ListCreatedCharactersResponse,
};
use crate::repository::{
    CatalogRepository, CharacterRepository,
    PostgresCatalogRepository, PostgresCharacterRepository,
};
use crate::services::catalog_service::CatalogService;
use crate::services::character_management_service::CharacterManagementService;
use idklol_common::auth::jwt::jwt_validator_service::JwtValidatorService;
use sqlx::PgPool;

/// Main gRPC service that delegates to specialized sub-services
pub struct MyCharacterService {
    catalog_service: CatalogService,
    character_management_service: CharacterManagementService,
}

impl MyCharacterService {
    pub fn with_pool(pool: PgPool) -> Self {
        let catalog_repo: Arc<dyn CatalogRepository> = Arc::new(PostgresCatalogRepository::new(pool.clone()));
        let character_repo: Arc<dyn CharacterRepository> = Arc::new(PostgresCharacterRepository::new(pool));
        let jwt_validator = Arc::new(JwtValidatorService {});

        Self {
            catalog_service: CatalogService::new(catalog_repo),
            character_management_service: CharacterManagementService::new(
                character_repo,
                jwt_validator,
            ),
        }
    }
}

#[tonic::async_trait]
impl CharacterService for MyCharacterService {
    async fn check_character_creation_catalog_version(
        &self,
        request: Request<CheckCharacterCreationCatalogVersionRequest>,
    ) -> Result<Response<CheckCharacterCreationCatalogVersionResponse>, Status> {
        self.catalog_service.check_character_creation_catalog_version(request).await
    }

    async fn get_character_creation_catalog(
        &self,
        request: Request<()>,
    ) -> Result<Response<CharacterCreationCatalog>, Status> {
        self.catalog_service.get_character_creation_catalog(request).await
    }

    async fn create_character(
        &self,
        request: Request<CreateCharacterRequest>,
    ) -> Result<Response<CreateCharacterResponse>, Status> {
        self.character_management_service.create_character(request).await
    }

    async fn list_created_characters(
        &self,
        request: Request<ListCreatedCharactersRequest>,
    ) -> Result<Response<ListCreatedCharactersResponse>, Status> {
        self.character_management_service.list_created_characters(request).await
    }
}
