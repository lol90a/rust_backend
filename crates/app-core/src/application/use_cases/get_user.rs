use std::sync::Arc;

use tracing::instrument;
use uuid::Uuid;

use crate::{
    application::{dto::UserView, ports::UserRepository},
    domain::errors::DomainError,
};

#[derive(Debug)]
pub struct GetUserUseCase {
    repo: Arc<dyn UserRepository>,
}

impl GetUserUseCase {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    #[instrument(skip(self), fields(user_id = %id))]
    pub async fn execute(&self, id: Uuid) -> Result<UserView, DomainError> {
        let user = self
            .repo
            .find_by_id(&id)
            .await?
            .ok_or_else(|| DomainError::UserNotFound(id.to_string()))?;

        Ok(UserView::from(user))
    }
}
