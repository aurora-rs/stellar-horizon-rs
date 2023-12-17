//! Helper functions to access Horizon headers.
use std::str::FromStr;

pub use hyper::header;
pub use hyper::HeaderMap;

/// Returns the remaining requests quota in the current window.
pub fn rate_limit_remaining(headers: &HeaderMap) -> Option<u32> {
    headers
        .get("X-Ratelimit-Remaining")
        .map(|value| u32::from_str(value.to_str().unwrap_or("")).ok())
        .unwrap_or(None)
}

/// Returns the requests quota in the time window.
pub fn rate_limit_limit(headers: &HeaderMap) -> Option<u32> {
    headers
        .get("X-Ratelimit-Limit")
        .map(|value| u32::from_str(value.to_str().unwrap_or("")).ok())
        .unwrap_or(None)
}

/// Returns the time remaining in the current window, specified in seconds.
pub fn rate_limit_reset(headers: &HeaderMap) -> Option<u32> {
    headers
        .get("X-Ratelimit-Reset")
        .map(|value| u32::from_str(value.to_str().unwrap_or("")).ok())
        .unwrap_or(None)
}
