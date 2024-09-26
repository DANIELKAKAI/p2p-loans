use reqwest::blocking::{Client, Response};
use serde_json::json;
use std::{collections::HashMap, env};
use dotenvy::dotenv;

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
    Ok(json_response
        .get("accessToken")
        .cloned()
        .unwrap_or_default())
}


use reqwest::header::AUTHORIZATION;
use base64::encode;
use chrono::Utc;
use serde::Deserialize;
use std::error::Error;



fn get_access_token() -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    let client = Client::new();
    let token_url = env::var("MPESA_TOKEN_URL")?;
    let consumer_key = env::var("MPESA_CONSUMER_KEY")?;
    let consumer_secret = env::var("MPESA_CONSUMER_SECRET")?;
    
    let response: Response = client
        .get(token_url)
        .basic_auth(consumer_key, Some(consumer_secret))
        .send()?;

    let token_response: TokenResponse = response.json()?;

    Ok(token_response.access_token)
}


fn initiate_transaction(phone_number: &str, reference: &str, amount: &str) -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    let short_code = env::var("MPESA_SHORTCODE")?;
    let passkey = env::var("MPESA_PASSKEY")?;
    let api_url = env::var("MPESA_API_URL")?;
    let callback_url = env::var("MPESA_CALLBACK_URL")?;

    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let password_string = format!("{}{}{}", short_code, passkey, &timestamp);
    let password = encode(password_string);

    let request_body = serde_json::json!({
        "BusinessShortCode": short_code,
        "Password": password,
        "Timestamp": timestamp,
        "TransactionType": "CustomerPayBillOnline",
        "Amount": amount,
        "PartyA": phone_number,
        "PartyB": short_code,
        "PhoneNumber": phone_number,
        "CallBackURL": callback_url,
        "AccountReference": reference,
        "TransactionDesc": "testing mpesa"
    });

    let client = Client::new();
    let access_token = get_access_token()?;

    let response: Response = client
        .post(api_url)
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
        .json(&request_body)
        .send()?;

    Ok(response.text()?)
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}
