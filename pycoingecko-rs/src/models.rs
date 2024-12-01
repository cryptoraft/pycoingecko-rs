#[derive(Debug, serde::Deserialize)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct MarketData {
    pub id: String,
    pub market_cap: f64,
    pub volume: f64,
}
