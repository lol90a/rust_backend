/// Cross-cutting types shared across layers (pagination, cursors, etc.).
use serde::{Deserialize, Serialize};

/// Generic paginated response envelope.
#[derive(Debug, Serialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub limit: usize,
    pub offset: usize,
}

/// Opaque request ID threaded through the call stack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestId(pub String);
