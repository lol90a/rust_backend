use std::fmt;

use crate::domain::errors::DomainError;

/// Validated e-mail address value object.
///
/// Construction always goes through [`Email::try_from`] so the invariant
/// "the string is a valid address" is guaranteed at the type level.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn is_valid(raw: &str) -> bool {
        // Minimal structural validation – replace with `email_address` crate in production.
        let trimmed = raw.trim();
        let at = trimmed.find('@');
        match at {
            None => false,
            Some(pos) => {
                let (local, rest) = trimmed.split_at(pos);
                let domain = &rest[1..];
                !local.is_empty() && domain.contains('.') && !domain.starts_with('.')
            },
        }
    }
}

impl TryFrom<String> for Email {
    type Error = DomainError;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        let trimmed = raw.trim().to_lowercase();
        if Self::is_valid(&trimmed) {
            Ok(Self(trimmed))
        } else {
            Err(DomainError::InvalidEmail(raw))
        }
    }
}

impl TryFrom<&str> for Email {
    type Error = DomainError;

    fn try_from(raw: &str) -> Result<Self, Self::Error> {
        Email::try_from(raw.to_owned())
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
