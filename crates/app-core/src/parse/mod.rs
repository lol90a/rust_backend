/// Generic parsing helpers.
use std::str::FromStr;

/// Parse a string slice into `T`, mapping the error into `anyhow::Error`.
pub fn parse<T: FromStr>(s: &str) -> anyhow::Result<T>
where
    T::Err: std::error::Error + Send + Sync + 'static,
{
    s.parse::<T>().map_err(Into::into)
}
