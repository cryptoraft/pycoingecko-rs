use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CoinData {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub market_data: MarketData,
}

#[derive(Deserialize, Debug)]
pub struct MarketData {
    pub current_price: serde_json::Value,
}