use reqwest::blocking::Client;
use serde_json::json;
use std::collections::HashMap;

pub fn get_jenga_payment_token() -> Result<String, Box<dyn std::error::Error>> {
    let url = std::env::var("JENGA_API_URL")?;
    let merchant_code = std::env::var("JENGA_MERCHANT_CODE")?;
    let consumer_secret = std::env::var("JENGA_CONSUMER_SECRET")?;
    let api_key = std::env::var("JENGA_API_KEY")?;

    let payload = json!({
        "merchantCode": merchant_code,
        "consumerSecret": consumer_secret
    });

    let client = Client::new();
    let res = client
        .post(&url)
        .header("Api-Key", api_key)
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()?;

    let json_response: HashMap<String, String> = res.json()?;
    Ok(json_response.get("accessToken").cloned().unwrap_or_default())
}


