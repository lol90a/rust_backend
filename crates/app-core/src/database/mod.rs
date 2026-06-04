/// Database connection pool configuration types.
///
/// Concrete pool wrappers (`SQLx`, `Diesel`) live in `infrastructure::database`.

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}
