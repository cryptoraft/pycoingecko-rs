use reqwest::{Client, Error};
use crate::error::ApiError;
use std::collections::HashMap;

/// Fetches the list of exchanges from the CoinGecko API.
pub async fn get_all_exchanges(client: &Client, base_url: &str) -> Result<HashMap<String, String>, ApiError> {
    let url = format!("{}/exchanges", base_url);
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| ApiError::RequestError(e.to_string()))?;

    if response.status().is_success() {
        let exchanges: HashMap<String, String> = response.json().await.map_err(|e| ApiError::ParsingError(e.to_string()))?;
        Ok(exchanges)
    } else {
        Err(ApiError::RequestError(format!("Failed to fetch exchanges: {}", response.status())))
    }
}

/// Fetches the details of a specific exchange by ID.
pub async fn get_exchange_details(client: &Client, base_url: &str, exchange_id: &str) -> Result<HashMap<String, String>, ApiError> {
    let url = format!("{}/exchanges/{}", base_url, exchange_id);
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| ApiError::RequestError(e.to_string()))?;

    if response.status().is_success() {
        let exchange_details: HashMap<String, String> = response.json().await.map_err(|e| ApiError::ParsingError(e.to_string()))?;
        Ok(exchange_details)
    } else {
        Err(ApiError::RequestError(format!("Failed to fetch exchange details: {}", response.status())))
    }
}
