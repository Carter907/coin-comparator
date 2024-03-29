use std::collections::HashMap;
use reqwest::Client;

pub async fn get_coin_price(id: &str) -> reqwest::Result<HashMap<String, HashMap<String, f64>>> {
    let client = Client::new();

    // Make a GET request to an example API
    let response = client.get(format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", id))
        .send()
        .await
        .unwrap();
    // Check if the request was successful
    // Return the response body as a String
    response.json::<HashMap<String, HashMap<String, f64>>>().await
}