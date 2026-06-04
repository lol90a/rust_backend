use crate::domain::errors::DomainError;

/// Pure domain service: validates user field constraints.
/// Lives here (not in the entity) because validation may involve rules
/// that span multiple entities or require contextual knowledge.
#[derive(Debug)]
pub struct UserValidator;

impl UserValidator {
    pub fn validate_display_name(name: &str) -> Result<(), DomainError> {
        let trimmed = name.trim();
        if trimmed.is_empty() || trimmed.len() > 100 {
            return Err(DomainError::InvalidDisplayName);
        }
        Ok(())
    }
}
