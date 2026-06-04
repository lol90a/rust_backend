use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use app_core::application::dto::{CreateUserCommand, ListUsersQuery};

use crate::{errors::ApiError, state::AppState};

// ── Request bodies ────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct ListUsersParams {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

// ── Handlers ──────────────────────────────────────────────────────────────────

/// POST /users
#[post("/users")]
pub async fn create_user(
    state: web::Data<AppState>,
    body: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, ApiError> {
    let cmd = CreateUserCommand {
        email: body.email.clone(),
        display_name: body.display_name.clone(),
    };
    let view = state.create_user.execute(cmd).await?;
    Ok(HttpResponse::Created().json(view))
}

/// GET /users
#[get("/users")]
pub async fn list_users(
    state: web::Data<AppState>,
    query: web::Query<ListUsersParams>,
) -> Result<HttpResponse, ApiError> {
    let q = ListUsersQuery {
        limit: query.limit,
        offset: query.offset,
    };
    let users = state.list_users.execute(q).await?;
    Ok(HttpResponse::Ok().json(users))
}

/// GET /users/{id}
#[get("/users/{id}")]
pub async fn get_user(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let user = state.get_user.execute(*path).await?;
    Ok(HttpResponse::Ok().json(user))
}
