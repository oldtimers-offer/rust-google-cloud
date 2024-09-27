use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs;

// Structure to hold the service account information from the JSON file
#[derive(Serialize, Deserialize)]
struct ServiceAccount {
    private_key: String,
    client_email: String,
    token_uri: String,
}

// Structure for JWT claims
#[derive(Serialize, Deserialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: usize,
    iat: usize,
}

// Structure for OAuth2 token response
#[derive(Serialize, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
}

pub static GOOGLE_KEY: Lazy<String> = Lazy::new(|| {
    dotenv().ok(); // Load .env file
    let key = env::var("Google_Credentials").expect("Google_Credentials must be set");
    key.to_string()
});

pub async fn get_oauth_token() -> Result<String, Box<dyn Error>> {
    // Load service account credentials
    let service_account: ServiceAccount =
        serde_json::from_str(&fs::read_to_string(GOOGLE_KEY.clone())?)?;

    // JWT expiration time (1 hour in seconds)
    let expiration = Utc::now() + Duration::hours(1);

    // Create JWT claims
    let claims = Claims {
        iss: service_account.client_email.clone(),
        scope: "https://www.googleapis.com/auth/cloud-platform".to_string(),
        aud: service_account.token_uri.clone(),
        exp: expiration.timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
    };

    // Encode JWT
    let jwt = encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(service_account.private_key.as_bytes())?,
    )?;

    // Request OAuth2 token
    let client = Client::new();
    let res = client
        .post(&service_account.token_uri)
        .form(&[
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ])
        .send()
        .await?;

    // Parse the response for the access token
    let token_response: TokenResponse = res.json().await?;
    Ok(token_response.access_token)
}
