use tonic::{Request, Response, Status};

use crate::{
    application::managers::{
        club::club_manager::{ClubManagerImpl, ClubManagerTrait},
        user::user_manager::UserManagerImpl,
    },
    infra::db_initializor::LetDbConnection,
    services::{
        common::request::request_validator::RequestValidator,
        extensions::utils::ExtensionExtractor,
        proto::club::{club_server::Club as ClubServer, CreateClubRequest, CreateClubResponse},
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
}
