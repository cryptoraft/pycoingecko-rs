use std::collections::HashMap;
use reqwest::{Error, Client};
use pycoingecko_rs::client::CoinGeckoAPI;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Set your actual API key here
    let api_key: Option<String> = Some("CG-F49RzomXupv1usPxpaWFqNi4".to_string());
    let demo_api_key: Option<String> = Some("CG-F49RzomXupv1usPxpaWFqNi4".to_string());

    // Ensure the key is valid before proceeding
    if demo_api_key.is_none() {
        eprintln!("API Key is missing or invalid.");
        return Ok(());
    }

    println!("Using API Key: {:?}", api_key); // Debug print to check the API key

    let cloned_api_key = demo_api_key.clone();
    let coin_gecko_api = CoinGeckoAPI::new(api_key, 3, demo_api_key);

    let url = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids=bitcoin,ethereum";
    let mut params = HashMap::new();
    params.insert("vs_currency".to_string(), "usd".to_string());
    params.insert("ids".to_string(), "bitcoin,ethereum".to_string());

    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36") // Add User-Agent header
        .header("Authorization", format!("Bearer {}", cloned_api_key.unwrap()))
        .query(&params)
        .send()
        .await?;

    // Log the status code and the raw response body
    println!("Response Status: {}", response.status());
    let body = response.text().await?;
    println!("Raw Response Body: {}", body);

    match serde_json::from_str::<serde_json::Value>(&body) {
        Ok(data) => {
            println!("Market Data: {:?}", data); // Print the fetched market data
        }
        Err(e) => {
            eprintln!("Error decoding response body: {}", e);
        }
    }

    Ok(())
}