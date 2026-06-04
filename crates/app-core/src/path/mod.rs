/// Path utilities – wrappers over `std::path`.
use std::path::{Path, PathBuf};

/// Ensure a directory exists, creating it (and parents) if necessary.
pub fn ensure_dir(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

/// Join a base directory with a relative path, rejecting path traversal.
pub fn safe_join(base: &Path, rel: &Path) -> Option<PathBuf> {
    let joined = base.join(rel);
    if joined.starts_with(base) {
        Some(joined)
    } else {
        None
    }
}
