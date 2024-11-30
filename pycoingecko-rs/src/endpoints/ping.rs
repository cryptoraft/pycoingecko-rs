use reqwest::{Client, Error};
use crate::error::ApiError;

pub async fn ping(client: &Client, base_url: &str) -> Result<String, ApiError> {
    let url = format!("{}/ping", base_url);
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| ApiError::RequestError(e.to_string()))?;

    if response.status().is_success() {
        let ping_response: String = response.text().await.map_err(|e| ApiError::ParsingError(e.to_string()))?;
        Ok(ping_response)
    } else {
        Err(ApiError::RequestError(format!("Ping failed: {}", response.status())))
    }
}
