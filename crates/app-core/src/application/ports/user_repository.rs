use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{entities::User, errors::DomainError, value_objects::Email};

/// Port (abstract interface) for user persistence.
///
/// The application layer owns this trait. Infrastructure crates provide
/// concrete implementations; the server wires them together via DI.
#[async_trait]
pub trait UserRepository: Send + Sync + std::fmt::Debug + 'static {
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    async fn delete(&self, id: &Uuid) -> Result<(), DomainError>;
    async fn list(&self, limit: usize, offset: usize) -> Result<Vec<User>, DomainError>;
}
