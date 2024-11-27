use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoinGeckoError {
    #[error("API request failed with error: {0}")]
    ApiError(String),

    #[error("HTTP request error: {0}")]
    HttpRequestError(#[from] reqwest::Error),
}