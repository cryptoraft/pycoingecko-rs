use reqwest::{Client, Response};
use crate::error::ApiError;
use std::collections::HashMap;

pub struct CoinGeckoAPI {
    api_base_url: String,
    extra_params: Option<HashMap<String, String>>,
    request_timeout: u64,
    session: Client,
}

impl CoinGeckoAPI {
    pub fn new(api_key: Option<String>, _retries: usize, demo_api_key: Option<String>) -> Self {
        let mut extra_params = None;

        let api_base_url = if let Some(demo_api_key) = demo_api_key {
            extra_params = Some([("x_cg_demo_api_key".to_string(), demo_api_key)].iter().cloned().collect());
            "https://api.coingecko.com/api/v3/".to_string()
        } else {
            "https://api.coingecko.com/api/v3/".to_string()
        };

        let session = Client::new();
        
        CoinGeckoAPI {
            api_base_url,
            extra_params,
            request_timeout: 120,
            session,
        }
    }

    pub async fn request(&self, url: &str, params: Option<HashMap<String, String>>) -> Result<HashMap<String, String>, ApiError> {
        let mut final_params = params.unwrap_or_else(HashMap::new);
    
        if !final_params.contains_key("vs_currency") {
            return Err(ApiError::RequestError("Missing required parameter: vs_currency".to_string()));
        }
    
        // Add extra parameters, including the demo API key, if available
        if let Some(ref extra_params) = self.extra_params {
            final_params.extend(extra_params.clone());
        }
    
        let response: Response = self.session.get(url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36")
            .query(&final_params)
            .timeout(std::time::Duration::from_secs(self.request_timeout))
            .send()
            .await
            .map_err(|e| ApiError::RequestError(e.to_string()))?;
    
        // Extract status and response body
        let status = response.status();
        let body = response.text().await.unwrap_or_else(|_| String::new());
    
        // Log the raw response body
        let body_preview = &body[..std::cmp::min(500, body.len())];
        println!("Raw Response Body: {}", body_preview);
    
        // Check for API errors based on the response body
        if status.is_success() {
            // Try to parse the body (expecting valid JSON)
            let content: Result<HashMap<String, String>, _> = serde_json::from_str(&body);
            content.map_err(|e| ApiError::ParsingError(e.to_string()))
        } else {
            // If the status is not success, check for known error codes
            if let Ok(error_response) = serde_json::from_str::<HashMap<String, HashMap<String, String>>>(&body) {
                if let Some(status) = error_response.get("status") {
                    if let Some(error_code) = status.get("error_code") {
                        if error_code == "10002" {
                            return Err(ApiError::RequestError("API Key Missing".to_string()));
                        }
                    }
                }
            }
            Err(ApiError::RequestError(format!("Failed to fetch data: {}", status)))
        }
    }
    
    
}
