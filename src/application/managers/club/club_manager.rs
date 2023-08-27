use anyhow::{bail, Result};

use crate::{
    application::{
        domain::models::club::domain_club_model::DomainClubModel,
        managers::common::manager::ManagerTrait,
        repositories::club::club_repository::{ClubRepositoryImpl, ClubRepositoryTrait},
    },
    infra::db_initializor::LetDbConnection,
    services::{
        club::creation::request::{NewClub, NewClubActiveModelWrapper},
        common::response::response_status::LettResError,
        extensions::user_context::UserContextExt,
    },
};

#[tonic::async_trait]
pub trait ClubManagerTrait: ManagerTrait<DomainClubModel> {
    async fn club_creation(
        &self,
        new_club: NewClub,
        user_context: Option<UserContextExt>,
    ) -> Result<DomainClubModel, LettResError>;
    async fn check_club_name_availability(&self, name: &str) -> Result<()>;
    async fn find_user_club_by_id(&self, user_id: i32) -> Result<DomainClubModel, LettResError>;
    async fn add_club_to_user(&self, user_id: i32, club_id: i32) -> Result<(), LettResError>;
}

// Implementation of UserManagerTrait
pub struct ClubManagerImpl {
    repo: ClubRepositoryImpl,
}

impl ClubManagerImpl {
    pub fn new(db_connection: LetDbConnection) -> Self {
        Self {
            repo: ClubRepositoryImpl::new(db_connection),
        }
    }
}

#[tonic::async_trait]
impl ClubManagerTrait for ClubManagerImpl {
    async fn club_creation(
        &self,
        new_club: NewClub,
        user_context: Option<UserContextExt>,
    ) -> Result<DomainClubModel, LettResError> {
        self.check_club_name_availability(&new_club.name).await?;

        let owner_id = user_context.map(|user| user.user_id);
        let new_club_active_model: NewClubActiveModelWrapper = new_club.into();

        let res = self
            .repo
            .create_club(new_club_active_model.0, owner_id)
            .await?;

        let club_domain_model = DomainClubModel::from(res);
        Ok(club_domain_model)
    }

    async fn check_club_name_availability(&self, name: &str) -> Result<()> {
        let find_club = self.repo.find_club_by_name(name).await?;
        if let Some(exists_club) = find_club {
            bail!("Club with name {} already exists", exists_club.name);
        }

        Ok(())
    }

    async fn find_user_club_by_id(&self, user_id: i32) -> Result<DomainClubModel, LettResError> {
        let find_club = self.repo.find_user_club_by_id(user_id).await?;
        let club = find_club.ok_or(LettResError::NotFound {
            entity: "Club".to_string(),
            id: "".to_string(),
        })?;

        let club_domain_model = DomainClubModel::from(club);
        Ok(club_domain_model)
    }

    async fn add_club_to_user(&self, user_id: i32, club_id: i32) -> Result<(), LettResError> {
        self.repo.update_user_club_id(user_id, club_id).await?;

        Ok(())
    }
}

impl ManagerTrait<DomainClubModel> for ClubManagerImpl {}
