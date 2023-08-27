use tonic::{Request, Response, Status};

use crate::{
    application::managers::{
        club::club_manager::{ClubManagerImpl, ClubManagerTrait},
        user::user_manager::{UserManagerImpl, UserManagerTrait},
    },
    infra::db_initializor::LetDbConnection,
    services::{
        common::{
            request::request_validator::RequestValidator, response::response_status::LettResError,
        },
        extensions::utils::ExtensionExtractor,
        proto::club::{
            club_server::Club as ClubServer, AddClubUserRequest, AddClubUserResponse,
            AddClubUserResponseData, CreateClubRequest, CreateClubResponse,
        },
    },
};

use super::creation::request::NewClub;

pub struct ClubService<T: ClubManagerTrait> {
    manager: T,
    user_manager: UserManagerImpl,
}

impl ClubService<ClubManagerImpl> {
    pub fn new(db_connection: LetDbConnection) -> Self {
        Self {
            manager: ClubManagerImpl::new(db_connection.clone()),
            user_manager: UserManagerImpl::new(db_connection),
        }
    }
}

#[tonic::async_trait]
impl ClubServer for ClubService<ClubManagerImpl> {
    async fn create_club(
        &self,
        request: Request<CreateClubRequest>,
    ) -> Result<Response<CreateClubResponse>, Status> {
        let user_context = ExtensionExtractor::auth_user_ext(&request)?;

        let new_club = NewClub::from(request.into_inner());

        RequestValidator::new(&new_club).validate_for_response()?;

        self.user_manager
            .can_create_club(user_context.user_id)
            .await?;

        let created_club = self
            .manager
            .club_creation(new_club, Some(user_context))
            .await?;

        let response_data = created_club.into();

        let data = Some(response_data);
        let response = CreateClubResponse { data };
        Ok(Response::new(response))
    }

    async fn add_club_user(
        &self,
        request: Request<AddClubUserRequest>,
    ) -> Result<Response<AddClubUserResponse>, Status> {
        let user_context = ExtensionExtractor::auth_user_ext(&request)?;
        let request = request.into_inner();
        let owner_user_id = user_context.user_id;
        let target_user_id = request.user_id;

        self.user_manager
            .can_add_user_to_club(owner_user_id)
            .await?;

        let target_user = self.user_manager.get_user_by_id(target_user_id).await?;

        if target_user.is_corporation() {
            return Err(LettResError::InternalServerError(
                "Cannot add corporation user to a club".to_string(),
            )
            .into());
        }

        let club = self.manager.find_user_club_by_id(owner_user_id).await?;

        self.manager
            .add_club_to_user(target_user_id, club.id)
            .await?;

        let data = AddClubUserResponseData {
            user_id: target_user_id,
            club_id: club.id,
        };

        Ok(Response::new(AddClubUserResponse { data: Some(data) }))
    }
}
