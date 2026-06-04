/// Event broker port definition.
use async_trait::async_trait;

#[async_trait]
pub trait EventBroker: Send + Sync {
    async fn publish(&self, topic: &str, payload: &[u8]) -> anyhow::Result<()>;
}
