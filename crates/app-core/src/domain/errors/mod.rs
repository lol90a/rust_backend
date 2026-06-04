use thiserror::Error;

/// All errors that can originate inside the domain layer.
///
/// These are pure business-rule violations. They carry no HTTP status codes,
/// no database error codes – those mappings live in the server crate.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DomainError {
    #[error("'{0}' is not a valid e-mail address")]
    InvalidEmail(String),

    #[error("display name must be between 1 and 100 characters")]
    InvalidDisplayName,

    #[error("user with id '{0}' was not found")]
    UserNotFound(String),

    #[error("a user with e-mail '{0}' already exists")]
    UserAlreadyExists(String),
}
