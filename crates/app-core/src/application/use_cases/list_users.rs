use std::sync::Arc;

use tracing::instrument;

use crate::{
    application::{
        dto::{ListUsersQuery, UserView},
        ports::UserRepository,
    },
    domain::errors::DomainError,
};

const DEFAULT_LIMIT: usize = 20;
const MAX_LIMIT: usize = 100;

#[derive(Debug)]
pub struct ListUsersUseCase {
    repo: Arc<dyn UserRepository>,
}

impl ListUsersUseCase {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    #[instrument(skip(self))]
    pub async fn execute(&self, query: ListUsersQuery) -> Result<Vec<UserView>, DomainError> {
        let limit = query.limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);
        let offset = query.offset.unwrap_or(0);

        let users = self.repo.list(limit, offset).await?;
        Ok(users.into_iter().map(UserView::from).collect())
    }
}
