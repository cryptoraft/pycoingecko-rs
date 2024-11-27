use pycoingecko_rs::CoinGeckoClient;

#[tokio::main]
async fn main() {
    let client = CoinGeckoClient::new();

    match client.get_coin("bitcoin").await {
        Ok(coin) => println!("{:?}", coin),
        Err(e) => eprintln!("Error fetching coin data: {}", e),
    }
}
