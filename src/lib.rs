//! # Stellar Horizon
//!
//! The `stellar-horizon` crate provides a client to connect to Horizon.
//! Horizon is an API to interact with the Stellar network.
//!
//! If you are looking for a way to create Stellar transactions, take
//! a look at [`stellar-base`](https://docs.rs/stellar-base).
//!
//!
//! ### Design Goals
//!
//! This crate makes some design choices that come from my experience
//! in writing server side application that interact with the Stellar
//! network:
//!
//!  * `HorizonClient` is a trait to make writing unit tests easier.
//!  * Streaming responses are just a `Stream`, again to simplify testing.
//!  * Expose response headers to follow rate limiting and avoid error
//!  response.
//!  * Responses can be serialized back to json, making it possible to
//!  write middleware services.
//!
//!
//! ## Connecting to Horizon and first request
//!
//! You connect to horizon by creating an `HorizonHttpClient`, then
//! you can use this client to send requests to Horizon.
//!
//! ```rust
//! use stellar_horizon::api;
//! use stellar_horizon::client::{HorizonClient, HorizonHttpClient};
//!
//! # async fn run() -> stellar_horizon::error::Result<()> {
//! let client = HorizonHttpClient::new_from_str("https://horizon.stellar.org")?;
//! let request = api::root::root();
//! let (headers, response) = client.request(request).await?;
//! println!("Horizon Version = {}", response.horizon_version);
//! # Ok(())
//! # }
//! ```
//!
//!
//! ## Account details
//!
//! ```rust
//! use stellar_base::PublicKey;
//! use stellar_horizon::api;
//! use stellar_horizon::client::{HorizonClient, HorizonHttpClient};
//!
//! # async fn run() -> stellar_horizon::error::Result<()> {
//! let client = HorizonHttpClient::new_from_str("https://horizon.stellar.org")?;
//! let account = PublicKey::from_account_id("GA73S4WXZG7EONFCIFDSZ6VOJKFC2PMV5574YDJC4V4UBDGPAYN4SPAC")?;
//! let request = api::accounts::single(&account);
//! let (headers, response) = client.request(request).await?;
//! println!("Account Sequence Number = {}", response.sequence);
//! # Ok(())
//! # }
//! ```
//!
//! ## Streaming responses
//!
//! This crate has full support for streaming response from
//! Horizon. Since an Horizon stream is just a `Stream`, you can use
//! all methods that work on a `Stream`.
//!
//! ```rust
//! use stellar_horizon::api;
//! use stellar_horizon::client::{HorizonClient, HorizonHttpClient};
//! use stellar_horizon::request::PageRequest;
//! use futures::stream::{Stream, StreamExt, TryStreamExt};
//!
//! # async fn run() -> stellar_horizon::error::Result<()> {
//! let client = HorizonHttpClient::new_from_str("https://horizon.stellar.org")?;
//! let request = api::transactions::all().with_cursor("now");
//! // Only take the first 5 events.
//! let mut stream = client.stream(request)?.take(5);
//! while let Some(event) = stream.try_next().await? {
//!     println!("Event = {:?}", event);
//! }
//! # Ok(())
//! # }
//! ```
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate stellar_base;

#[macro_use]
pub mod request;

pub mod api;
pub mod client;
pub mod error;
pub mod headers;
pub mod horizon_error;
pub mod link;
pub mod page;
pub mod resources;

/// The crate version.
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
