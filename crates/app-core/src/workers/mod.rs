/// Background worker abstractions.
///
/// Workers are long-running async tasks (e.g. job processors, cron tasks).
/// Define a `Worker` trait here and implement it per worker type.
use async_trait::async_trait;

#[async_trait]
pub trait Worker: Send + Sync {
    /// Human-readable identifier used in logs.
    fn name(&self) -> &'static str;
    /// Run the worker until it completes or returns an error.
    async fn run(&self) -> anyhow::Result<()>;
}
