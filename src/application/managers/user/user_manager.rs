#[tonic::async_trait]
pub trait UserManagerTrait {
    fn new() -> Self;
    async fn user_registration(&self) {}
}

pub struct UserManager {}

impl UserManagerTrait for UserManager {
    fn new() -> Self {
        Self {}
    }
}
