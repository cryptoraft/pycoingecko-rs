use reqwest::Client;
use crate::error::ApiError;

pub async fn get_all_coins(_client: &Client, base_url: &str) -> Result<Vec<String>, ApiError> {
    let url = format!("{}/coins/list", base_url);
    let api_key = Some("your_api_key_here".to_string()); // Option<String> for API key
    let formatted_api_key = api_key.unwrap_or_else(|| "default_api_key".to_string()); // Unwrap Option or use a default value
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", formatted_api_key))  
        .send()
        .await
        .map_err(|e| ApiError::RequestError(e.to_string()))?;

    if response.status().is_success() {
        let coins: Vec<String> = response.json().await.map_err(|e| ApiError::ParsingError(e.to_string()))?;
        Ok(coins)
    } else {
        Err(ApiError::RequestError(format!("Failed to fetch coins: {}", response.status())))
    }
}
