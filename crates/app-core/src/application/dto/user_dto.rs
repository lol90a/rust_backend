use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::User;

// ── Commands ──────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateUserCommand {
    pub email: String,
    pub display_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserCommand {
    pub id: Uuid,
    pub display_name: String,
}

// ── Queries ───────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

// ── View models (responses) ───────────────────────────────────────────────────

/// Serializable view of a `User` – safe to send across the wire.
#[derive(Debug, Serialize)]
pub struct UserView {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserView {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            email: u.email.to_string(),
            display_name: u.display_name,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

impl From<&User> for UserView {
    fn from(u: &User) -> Self {
        Self {
            id: u.id,
            email: u.email.to_string(),
            display_name: u.display_name.clone(),
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}
