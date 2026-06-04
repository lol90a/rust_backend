use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use tracing::info;

mod bootstrap;
mod config;
mod errors;
mod handlers;
mod middleware;
mod routes;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load configuration (env vars → defaults).
    let cfg = config::load()?;

    // 2. Initialise structured logging.
    bootstrap::init_tracing(&cfg);

    info!(
        env = ?cfg.env,
        addr = cfg.bind_addr(),
        "starting server"
    );

    // 3. Build application state (wires use cases + repositories).
    let app_state = state::AppState::new_with_in_memory_repo();
    let state_data = web::Data::new(app_state);

    // 4. Launch Actix Web.
    let bind_addr = cfg.bind_addr();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(state_data.clone())
            // ── Middleware (outermost = last applied on request path) ──────
            .wrap(cors)
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(Logger::default())
            .wrap(middleware::RequestId)
            // ── Routes ────────────────────────────────────────────────────
            .configure(routes::configure)
    })
    .bind(&bind_addr)?
    .run()
    .await?;

    Ok(())
}
