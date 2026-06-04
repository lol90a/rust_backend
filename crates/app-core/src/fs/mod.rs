/// Async-friendly file-system helpers.
use std::path::Path;

/// Read a file to a UTF-8 string asynchronously.
pub async fn read_to_string(path: &Path) -> anyhow::Result<String> {
    tokio::fs::read_to_string(path).await.map_err(Into::into)
}

/// Write bytes to a file asynchronously, creating or truncating it.
pub async fn write(path: &Path, contents: &[u8]) -> anyhow::Result<()> {
    tokio::fs::write(path, contents).await.map_err(Into::into)
}
