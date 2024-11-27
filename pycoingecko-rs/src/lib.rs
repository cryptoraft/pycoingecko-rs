pub mod client;
pub mod endpoints;
pub mod error;
pub mod models;


// Re-export client for easier access
pub use client::CoinGeckoClient;