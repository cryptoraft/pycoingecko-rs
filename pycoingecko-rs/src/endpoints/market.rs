use reqwest::Client;
use crate::error::ApiError;
use std::collections::HashMap;

pub async fn get_market_data(client: &Client, base_url: &str) -> Result<HashMap<String, f64>, ApiError> {
    let url = format!("{}/market_data", base_url);
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| ApiError::RequestError(e.to_string()))?;

    if response.status().is_success() {
        let data: HashMap<String, f64> = response.json().await.map_err(|e| ApiError::ParsingError(e.to_string()))?;
        Ok(data)
    } else {
        Err(ApiError::RequestError(format!("Failed to fetch market data: {}", response.status())))
    }
}
