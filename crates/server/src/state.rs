use std::sync::Arc;

use app_core::{
    application::use_cases::{CreateUserUseCase, GetUserUseCase, ListUsersUseCase},
    infrastructure::database::InMemoryUserRepository,
};

/// Shared application state injected into every Actix handler via `web::Data`.
///
/// All fields are `Arc`-wrapped so clones are cheap; the actual data is
/// reference-counted and lives for the lifetime of the server process.
#[derive(Clone, Debug)]
pub struct AppState {
    pub create_user: Arc<CreateUserUseCase>,
    pub get_user: Arc<GetUserUseCase>,
    pub list_users: Arc<ListUsersUseCase>,
}

impl AppState {
    pub fn new_with_in_memory_repo() -> Self {
        // Coerce the concrete type to `Arc<dyn UserRepository>` before cloning
        // so all use cases share the same backing store.
        let repo: Arc<dyn app_core::application::ports::UserRepository> =
            Arc::new(InMemoryUserRepository::new());
        Self {
            create_user: Arc::new(CreateUserUseCase::new(Arc::clone(&repo))),
            get_user: Arc::new(GetUserUseCase::new(Arc::clone(&repo))),
            list_users: Arc::new(ListUsersUseCase::new(Arc::clone(&repo))),
        }
    }
}
