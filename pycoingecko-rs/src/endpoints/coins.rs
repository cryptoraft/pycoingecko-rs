use crate::client::CoinGeckoClient;
use crate::models::CoinData;
use crate::error::CoinGeckoError;

impl CoinGeckoClient {
  pub async fn get_coin(&self, coin_id: &str) -> Result<CoinData, CoinGeckoError> {
    self.get(&format!("coins/{}", coin_id)).await
  }
}