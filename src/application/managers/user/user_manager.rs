use crate::{
    application::{
        domain::models::user::domain_user_model::DomainUserModel,
        repositories::user::user_repository::UserRepositoryTrait,
    },
    infra::db_initializor::LetDbConnection,
    services::user::user_request::NewUserActiveModelWrapper,
};

#[tonic::async_trait]
pub trait UserManagerTrait {
    fn new(db_connection: LetDbConnection) -> Self;
    async fn user_registration<T: Into<NewUserActiveModelWrapper> + Send>(
        &self,
        new_user: T,
    ) -> Result<DomainUserModel, String>;
}

pub struct UserManager<T: UserRepositoryTrait> {
    user_repo: T,
}

#[tonic::async_trait]
impl<R: UserRepositoryTrait + Send + Sync> UserManagerTrait for UserManager<R> {
    fn new(db_conneciton: LetDbConnection) -> Self {
        let user_repo = R::new(db_conneciton);
        Self { user_repo }
    }
    async fn user_registration<T: Into<NewUserActiveModelWrapper> + Send>(
        &self,
        new_user: T,
    ) -> Result<DomainUserModel, String> {
        let NewUserActiveModelWrapper(active_model) = new_user.into();

        let registered_user = self.user_repo.create_user(active_model).await?;

        let domain_user_model = DomainUserModel::from(registered_user);

        Ok(domain_user_model)
    }
}
