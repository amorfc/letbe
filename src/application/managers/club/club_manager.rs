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
    },
};

#[tonic::async_trait]
pub trait ClubManagerTrait: ManagerTrait<DomainClubModel> {
    async fn club_creation(&self, new_club: NewClub) -> Result<DomainClubModel, LettResError>;
    async fn check_club_name_availability(&self, name: &str) -> Result<()>;
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
    async fn club_creation(&self, new_club: NewClub) -> Result<DomainClubModel, LettResError> {
        self.check_club_name_availability(&new_club.name).await?;

        let new_club_active_model: NewClubActiveModelWrapper = new_club.into();

        let res = self.repo.create_club(new_club_active_model.0).await?;

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
}

impl ManagerTrait<DomainClubModel> for ClubManagerImpl {}
