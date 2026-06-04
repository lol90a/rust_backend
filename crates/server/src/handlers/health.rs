use actix_web::{get, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

/// GET /health
///
/// Returns `200 OK` while the process is alive. Add deeper probes (DB ping,
/// cache ping) as `readiness` / `liveness` sub-checks if needed.
#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse { status: "ok" })
}
