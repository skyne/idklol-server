use tonic::{Request, Response, Status};
use std::sync::Arc;
use tracing::{debug, error, warn};
use crate::characters::{
    CreateCharacterRequest, CreateCharacterResponse,
    ListCreatedCharactersRequest, ListCreatedCharactersResponse,
    Character,
};
use crate::repository::{CharacterRepository};
use crate::services::auth::authenticate_user;
use idklol_common::auth::jwt::jwt_validator_service::JwtValidatorService;

/// Service for handling character management operations (create, list)
/// These operations require authentication
pub struct CharacterManagementService {
    character_repo: Arc<dyn CharacterRepository>,
    jwt_validator: Arc<JwtValidatorService>,
}

impl CharacterManagementService {
    pub fn new(
        character_repo: Arc<dyn CharacterRepository>,
        jwt_validator: Arc<JwtValidatorService>,
    ) -> Self {
        Self {
            character_repo,
            jwt_validator,
        }
    }

    pub async fn create_character(
        &self,
        request: Request<CreateCharacterRequest>,
    ) -> Result<Response<CreateCharacterResponse>, Status> {
        let user_email = authenticate_user(&*self.jwt_validator, request.metadata()).await?;
        let req = request.into_inner();
        
        let race_id = req.race;
        let gender_id = req.gender;
        let skin_color_id = req.skin_color;
        let class_id = req.character_class;
        
        debug!(
            %user_email,
            name = %req.name,
            race_id,
            gender_id,
            skin_color_id,
            class_id,
            "creating character"
        );

        // Validate character name
        if req.name.trim().is_empty() {
            return Err(Status::invalid_argument("character name cannot be empty"));
        }

        // Check if name already exists
        let name_exists = self.character_repo
            .name_exists(&req.name)
            .await
            .map_err(|err| {
                error!(error = %err, "failed to check name uniqueness");
                Status::internal("failed to validate character name")
            })?;

        if name_exists {
            warn!(name = %req.name, "character name already taken");
            return Err(Status::already_exists("character name is already taken"));
        }

        // Create the character
        let character_data = self.character_repo
            .create_character(
                &req.name,
                &user_email,
                race_id,
                gender_id,
                skin_color_id,
                class_id,
            )
            .await
            .map_err(|err| {
                error!(error = %err, "failed to create character");
                // Check if it's a foreign key violation (invalid combination)
                let err_msg = err.to_string();
                if err_msg.contains("violates foreign key constraint") {
                    Status::invalid_argument("invalid race/gender/skin color/class combination")
                } else {
                    Status::internal("failed to create character")
                }
            })?;

        let response = CreateCharacterResponse {
            character_id: character_data.id.to_string(),
            name: character_data.name,
            race: character_data.race_id,
            gender: character_data.gender_id,
            skin_color: character_data.skin_color_id,
            character_class: character_data.class_id,
            created_at: character_data.created_at.to_rfc3339(),
        };

        debug!(character_id = %response.character_id, "character created successfully");
        Ok(Response::new(response))
    }

    pub async fn list_created_characters(
        &self,
        request: Request<ListCreatedCharactersRequest>,
    ) -> Result<Response<ListCreatedCharactersResponse>, Status> {
        let user_email = authenticate_user(&*self.jwt_validator, request.metadata()).await?;
        debug!(%user_email, "listing characters for user");

        let characters_data = self.character_repo
            .list_characters_by_user(&user_email)
            .await
            .map_err(|err| {
                error!(error = %err, %user_email, "failed to fetch user characters");
                Status::internal("failed to fetch characters")
            })?;

        let characters = characters_data
            .into_iter()
            .map(|data| Character {
                character_id: data.id.to_string(),
                name: data.name,
                race: data.race_id,
                gender: data.gender_id,
                skin_color: data.skin_color_id,
                character_class: data.class_id,
                created_at: data.created_at.to_rfc3339(),
            })
            .collect();

        Ok(Response::new(ListCreatedCharactersResponse { characters }))
    }
}
