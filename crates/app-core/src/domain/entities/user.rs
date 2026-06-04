use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::value_objects::email::Email;

/// Core user entity.
///
/// This is the single source of truth for user identity within the domain.
/// It must never hold HTTP types, database row types, or serialization derives
/// directly – those belong in DTOs or infrastructure adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub email: Email,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    /// Create a brand-new user with a generated ID and current timestamp.
    pub fn new(email: Email, display_name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            display_name,
            created_at: now,
            updated_at: now,
        }
    }

    /// Reconstitute a user from persisted state (repository layer).
    pub fn reconstitute(
        id: Uuid,
        email: Email,
        display_name: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            email,
            display_name,
            created_at,
            updated_at,
        }
    }

    pub fn update_display_name(&mut self, new_name: String) {
        self.display_name = new_name;
        self.updated_at = Utc::now();
    }
}
