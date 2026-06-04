/// Queue port definition.
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait MessageQueue<T: Serialize + DeserializeOwned + Send>: Send + Sync {
    async fn publish(&self, topic: &str, message: &T) -> anyhow::Result<()>;
    async fn consume(&self, topic: &str) -> anyhow::Result<Vec<T>>;
}
