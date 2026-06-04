use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use tracing::debug;
use uuid::Uuid;

use crate::{
    application::ports::UserRepository,
    domain::{entities::User, errors::DomainError, value_objects::Email},
};

/// Thread-safe in-memory user store backed by [`DashMap`].
///
/// Intended for testing and local development. Replace with a real DB adapter
/// (e.g. `SqlxUserRepository`) in production by implementing the same trait.
#[derive(Debug, Default)]
pub struct InMemoryUserRepository {
    store: Arc<DashMap<Uuid, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, DomainError> {
        debug!(%id, "find_by_id");
        Ok(self.store.get(id).map(|r| r.clone()))
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
        debug!(%email, "find_by_email");
        Ok(self
            .store
            .iter()
            .find(|r| r.email == *email)
            .map(|r| r.clone()))
    }

    async fn save(&self, user: &User) -> Result<(), DomainError> {
        debug!(user_id = %user.id, "save");
        self.store.insert(user.id, user.clone());
        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<(), DomainError> {
        debug!(%id, "delete");
        self.store.remove(id);
        Ok(())
    }

    async fn list(&self, limit: usize, offset: usize) -> Result<Vec<User>, DomainError> {
        debug!(limit, offset, "list");
        let mut users: Vec<User> = self.store.iter().map(|r| r.clone()).collect();
        // Stable ordering for deterministic pagination.
        users.sort_by_key(|u| u.created_at);
        Ok(users.into_iter().skip(offset).take(limit).collect())
    }
}
