use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::AppConfig;

/// Initialise the `tracing` subscriber.
///
/// JSON format in non-development environments; pretty human-readable format
/// locally. The `RUST_LOG` env var always overrides `config.log_level`.
pub fn init_tracing(config: &AppConfig) {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.log_level));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_target(true))
        .init();
}
