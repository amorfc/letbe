use anyhow::Result;
use entity::authn as AuthnEntity;
use sea_orm::{entity::prelude::*, ActiveValue, TransactionTrait};

use crate::{
    application::repositories::common::repository::{DbConnectionProvider, RepositoryTrait},
    infra::db_initializor::LetDbConnection,
    shared::utils::datetime::LettDate,
};

// User Manager Trait that requires AuthnRepositoryTrait
#[tonic::async_trait]
pub trait AuthnRepositoryTrait:
    RepositoryTrait<AuthnEntity::ActiveModel, AuthnEntity::Entity>
{
    async fn create_authn_token(
        &self,
        new_authn: NewAuthEntityParams,
    ) -> Result<AuthnEntity::ActiveModel> {
        let now_dt_with_tz = LettDate::now_dt_with_tz();
        let db_tx = self.db_connection().begin().await?;

        AuthnEntity::Entity::update_many()
            .col_expr(AuthnEntity::Column::RevokedAt, Expr::value(now_dt_with_tz))
            .filter(AuthnEntity::Column::UserId.eq(new_authn.user_id))
            .filter(AuthnEntity::Column::DeviceId.eq(new_authn.device_id.clone()))
            .exec_with_returning(&db_tx)
            .await?;

        let active_model = NewAuthActiveModelWrapper::from(new_authn).0;

        let authn_token = self.save(active_model).await?;

        db_tx.commit().await?;

        Ok(authn_token)
    }
}

// Implementation of AuthnRepositoryTrait
#[derive(Clone, Debug)]
pub struct AuthnRepositoryImpl {
    db_connection: LetDbConnection,
}

#[tonic::async_trait]
impl AuthnRepositoryTrait for AuthnRepositoryImpl {}

impl AuthnRepositoryImpl {
    pub fn new(db_connection: LetDbConnection) -> Self {
        Self { db_connection }
    }
}

impl DbConnectionProvider for AuthnRepositoryImpl {
    fn db_connection(&self) -> &DatabaseConnection {
        &self.db_connection
    }
}
impl RepositoryTrait<AuthnEntity::ActiveModel, AuthnEntity::Entity> for AuthnRepositoryImpl {}

pub struct NewAuthActiveModelWrapper(AuthnEntity::ActiveModel);

pub struct NewAuthEntityParams {
    pub user_id: i32,
    pub device_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expired_time: DateTimeWithTimeZone,
}

impl From<NewAuthEntityParams> for NewAuthActiveModelWrapper {
    fn from(params: NewAuthEntityParams) -> Self {
        let now_dt_with_tz = LettDate::now_dt_with_tz();
        let active_model = AuthnEntity::ActiveModel {
            user_id: ActiveValue::set(params.user_id),
            device_id: ActiveValue::set(params.device_id),
            access_token: ActiveValue::set(params.access_token),
            refresh_token: ActiveValue::set(params.refresh_token),
            expired_time: ActiveValue::set(params.expired_time),
            created_at: ActiveValue::set(now_dt_with_tz),
            ..Default::default()
        };
        Self(active_model)
    }
}
