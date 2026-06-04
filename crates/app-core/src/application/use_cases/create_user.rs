use std::sync::Arc;

use tracing::{info, instrument};

use crate::{
    application::{
        dto::{CreateUserCommand, UserView},
        ports::UserRepository,
    },
    domain::{entities::User, errors::DomainError, services::UserValidator, value_objects::Email},
};

#[derive(Debug)]
pub struct CreateUserUseCase {
    repo: Arc<dyn UserRepository>,
}

impl CreateUserUseCase {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    #[instrument(skip(self), fields(email = %cmd.email))]
    pub async fn execute(&self, cmd: CreateUserCommand) -> Result<UserView, DomainError> {
        // 1. Validate inputs at the domain/application boundary.
        UserValidator::validate_display_name(&cmd.display_name)?;
        let email = Email::try_from(cmd.email)?;

        // 2. Enforce uniqueness invariant.
        if self.repo.find_by_email(&email).await?.is_some() {
            return Err(DomainError::UserAlreadyExists(email.to_string()));
        }

        // 3. Create the aggregate root.
        let user = User::new(email, cmd.display_name);

        // 4. Persist.
        self.repo.save(&user).await?;

        info!(user_id = %user.id, "user created");
        Ok(UserView::from(user))
    }
}
