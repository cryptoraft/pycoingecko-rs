use reqwest::Client;
use mockito::mock;
use pycoingecko_rs::endpoints::coins::get_all_coins;  // Adjusting the module name if necessary
use pycoingecko_rs::error::ApiError;


#[tokio::test]
async fn test_get_all_coins_success() {
    // Setup the mock response using mockito
    let _m = mock("GET", "/coins/list")
        .with_header("Authorization", "Bearer your_api_key_here")
        .with_status(200)
        .with_body(r#"["bitcoin", "ethereum", "dogecoin"]"#)  // Example response body
        .create();

    let base_url = mockito::server_url(); // Get the mock server URL
    let client = Client::new();

    // Call the function under test
    let result = get_all_coins(&client, &base_url).await;

    // Assert the result is a success and contains the expected data
    match result {
        Ok(coins) => {
            assert_eq!(coins, vec!["bitcoin", "ethereum", "dogecoin"]);
        },
        Err(e) => panic!("Expected successful result, but got error: {:?}", e),
    }
}

#[tokio::test]
async fn test_get_all_coins_failure() {
    // Setup a mock that returns an error status (e.g., 500)
    let _m = mock("GET", "/coins/list")
        .with_header("Authorization", "Bearer your_api_key_here")
        .with_status(500) // Simulating server error
        .create();

    let base_url = mockito::server_url();
    let client = Client::new();

    // Call the function under test
    let result = get_all_coins(&client, &base_url).await;

    // Assert the result is a failure with the expected error message
    match result {
        Ok(_) => panic!("Expected failure, but got successful response"),
        Err(ApiError::RequestError(msg)) => {
            assert!(msg.contains("Failed to fetch coins"));
        },
        Err(_) => panic!("Expected a RequestError, but got a different error"),
    }
}

#[tokio::test]
async fn test_get_all_coins_parsing_error() {
    // Setup a mock that returns invalid JSON
    let _m = mock("GET", "/coins/list")
        .with_header("Authorization", "Bearer your_api_key_here")
        .with_status(200)
        .with_body("not a valid json") // Invalid JSON to trigger parsing error
        .create();

    let base_url = mockito::server_url();
    let client = Client::new();

    // Call the function under test
    let result = get_all_coins(&client, &base_url).await;

    // Assert the result is a failure with the expected parsing error message
    match result {
        Ok(_) => panic!("Expected parsing error, but got successful response"),
        Err(ApiError::ParsingError(msg)) => {
            assert!(msg.contains("invalid JSON"));
        },
        Err(_) => panic!("Expected a ParsingError, but got a different error"),
    }
}
