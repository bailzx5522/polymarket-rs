//! # polymarket-rs
//!
//! A Rust client library for interacting with the Polymarket CLOB (Central Limit Order Book) API.
//!
//! This library provides a comprehensive, type-safe interface for:
//! - Market data queries (public)
//! - Order creation and management (authenticated)
//! - Account and balance operations (authenticated)
//! - Position tracking
//!
//! ## Features
//!
//! - **Builder Pattern**: Fluent API for constructing clients and orders
//! - **Type Safety**: Strong typing with newtypes for IDs (TokenId, OrderId, ConditionId)
//! - **Proper Error Handling**: No panics, comprehensive error types
//! - **EIP-712 Signing**: Full support for Ethereum wallet signatures
//! - **Decimal Precision**: Accurate decimal math for prices and amounts
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use polymarket_rs::client::ClobClient;
//! use polymarket_rs::types::TokenId;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ClobClient::new("https://clob.polymarket.com");
//!
//!     let token_id = TokenId::new("123456");
//!     let midpoint = client.get_midpoint(&token_id).await?;
//!
//!     println!("Midpoint: {}", midpoint.mid);
//!     Ok(())
//! }
//! ```
//!
//! ## Modules
//!
//! - [`client`]: Client implementations for different API operations
//! - [`types`]: Type definitions for API requests and responses
//! - [`config`]: Network and contract configuration
//! - [`orders`]: Order building and rounding logic
//! - [`signing`]: EIP-712 signing functionality
//! - [`error`]: Error types
//!

// Public modules
pub mod client;
pub mod config;
pub mod error;
pub mod orders;
pub mod request;
pub mod signing;
pub mod types;

// Internal modules
mod http;
mod utils;

// Re-export commonly used types
pub use alloy_primitives::Address;
pub use alloy_signer::k256;
pub use alloy_signer_local::PrivateKeySigner;
pub use error::{Error, Result};
pub use types::{
    ApiCreds, AssetType, ConditionId, CreateOrderOptions, ExtraOrderArgs, MarketOrderArgs,
    OrderArgs, OrderId, OrderType, Side, SignatureType, TokenId,
};

// Re-export clients
pub use client::{AuthenticatedClient, ClobClient, DataClient, TradingClient};

// Re-export order builder
pub use orders::OrderBuilder;

// Re-export signer trait
pub use signing::EthSigner;
