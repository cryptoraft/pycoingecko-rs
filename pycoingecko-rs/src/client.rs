use reqwest::Client;
use crate::error::CoinGeckoError;

pub struct CoinGeckoClient {
    base_url: String,
    http_client: Client,
}

impl CoinGeckoClient {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.coingecko.com/api/v3".to_string(),
            http_client: Client::new(),
        }
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T, CoinGeckoError>
    where 
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.http_client.get(&url).send().await?;
        if response.status().is_success() {
            // Try to desearialize the json response into type T
            response.json().await.map_err(|e| CoinGeckoError::ApiError(e.to_string()))
        } else {
            Err(CoinGeckoError::ApiError(response.text().await?))
        }
    }
}
