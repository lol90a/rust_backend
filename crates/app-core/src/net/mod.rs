//! Network / URL utilities.

/// Validate that a string is an `http://` or `https://` URL (naïve check).
pub fn is_http_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}
