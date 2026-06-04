use serde::Deserialize;

/// All runtime configuration, resolved from env vars + config files.
///
/// Loaded once at startup by `bootstrap::init_config`.
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default = "AppConfig::default_host")]
    pub host: String,

    #[serde(default = "AppConfig::default_port")]
    pub port: u16,

    #[serde(default = "AppConfig::default_log_level")]
    pub log_level: String,

    #[serde(default = "AppConfig::default_env")]
    pub env: Environment,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl AppConfig {
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    fn default_host() -> String {
        "0.0.0.0".to_owned()
    }

    fn default_port() -> u16 {
        8080
    }

    fn default_log_level() -> String {
        "info".to_owned()
    }

    fn default_env() -> Environment {
        Environment::Development
    }
}

pub fn load() -> anyhow::Result<AppConfig> {
    let cfg = config::Config::builder()
        // Defaults baked in via serde defaults above.
        // Override with APP__HOST, APP__PORT, APP__LOG_LEVEL, APP__ENV.
        .add_source(config::Environment::with_prefix("APP").separator("__"))
        .build()?
        .try_deserialize::<AppConfig>()?;

    Ok(cfg)
}
