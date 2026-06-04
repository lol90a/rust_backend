//! Thin wrappers around `serde_json` for ergonomic error handling.

use serde::{de::DeserializeOwned, Serialize};

pub fn to_string_pretty<T: Serialize>(value: &T) -> anyhow::Result<String> {
    serde_json::to_string_pretty(value).map_err(Into::into)
}

pub fn from_str<T: DeserializeOwned>(s: &str) -> anyhow::Result<T> {
    serde_json::from_str(s).map_err(Into::into)
}
