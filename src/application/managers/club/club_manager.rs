use anyhow::Result;

use crate::{
    application::{
        domain::models::club::domain_club_model::DomainClubModel,
        managers::common::manager::ManagerTrait,
        repositories::user::user_repository::ClubRepositoryImpl,
    },
    infra::db_initializor::LetDbConnection,
    services::common::response::response_status::LettResError,
    shared::utils::datetime::LettDate,
};

#[tonic::async_trait]
pub trait ClubManagerTrait: ManagerTrait<DomainClubModel> {
    async fn club_creation(&self) -> Result<DomainClubModel, LettResError>;
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
    async fn club_creation(&self) -> Result<DomainClubModel, LettResError> {
        let club_domain_model = DomainClubModel {
            id: 0,
            name: "test".to_string(),
            legal_name: "test".to_string(),
            created_at: LettDate::now_dt_with_tz(),
            updated_at: None,
            deleted_at: None,
        };
        Ok(club_domain_model)
    }
}

impl ManagerTrait<DomainClubModel> for ClubManagerImpl {}
