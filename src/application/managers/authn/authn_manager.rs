use anyhow::Result;
use entity::authn as AuthnEntity;
use sea_orm::TryIntoModel;

use crate::{
    application::{
        domain::models::authn::domain_authn_model::DomainAuthnModel,
        managers::common::manager::ManagerTrait,
        repositories::authn::authn_repository::{
            AuthnRepositoryImpl, AuthnRepositoryTrait, NewAuthEntityParams,
        },
    },
    infra::db_initializor::LetDbConnection,
    services::common::response::response_status::LettResError,
    shared::utils::{
        datetime::LettDate,
        jwt::{LettJwt, LettJwtClaims},
    },
};

#[tonic::async_trait]
pub trait AuthnManagerTrait: ManagerTrait<DomainAuthnModel> {
    async fn generate_jwt_token(
        &self,
        params: NewJwtParams,
    ) -> Result<DomainAuthnModel, LettResError>;
}

// Implementation of AuthnManagerTrait
pub struct AuthnManagerImpl {
    repo: AuthnRepositoryImpl,
}

impl AuthnManagerImpl {
    pub fn new(db_connection: LetDbConnection) -> Self {
        Self {
            repo: AuthnRepositoryImpl::new(db_connection),
        }
    }
}

#[tonic::async_trait]
impl AuthnManagerTrait for AuthnManagerImpl {
    async fn generate_jwt_token(
        &self,
        params: NewJwtParams,
    ) -> Result<DomainAuthnModel, LettResError> {
        let user_id = params.user_id;
        let device_id = params.device_id.clone();

        let access_token_claims = LettJwtClaims::access_token(user_id, device_id.clone(), None);
        let refresh_token_claims = LettJwtClaims::refresh_token(user_id, device_id.clone(), None);

        let access_token = LettJwt::create_jwt(&access_token_claims)?;
        let refresh_token = LettJwt::create_jwt(&refresh_token_claims)?;
        let expired_time = LettDate::dt_with_tz(access_token_claims.exp)?;

        let new_authn_token = NewAuthEntityParams {
            user_id,
            device_id,
            access_token,
            refresh_token,
            expired_time,
        };

        let authn = self.repo.create_authn_token(new_authn_token).await?;
        Ok(authn.into())
    }
}

impl From<AuthnEntity::ActiveModel> for DomainAuthnModel {
    fn from(value: AuthnEntity::ActiveModel) -> Self {
        let value = value.try_into_model().unwrap();
        Self {
            id: value.id,
            user_id: value.user_id,
            device_id: value.device_id,
            access_token: value.access_token,
            refresh_token: value.refresh_token,
            expired_time: value.expired_time,
            refreshed_at: value.refreshed_at,
            revoked_at: value.revoked_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl ManagerTrait<DomainAuthnModel> for AuthnManagerImpl {}

pub struct NewJwtParams {
    pub user_id: i32,
    pub device_id: String,
}
