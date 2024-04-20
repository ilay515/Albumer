use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i32,
}

#[allow(dead_code)]
pub async fn get_access_token(
    client_id: String,
    client_secret: String,
) -> Result<String, reqwest::Error> {
    let auth_header = format!(
        "Basic {}",
        base64::encode(format!("{}:{}", client_id, client_secret))
    );

    let client = Client::new();
    let response = client
        .post("https://accounts.spotify.com/api/token")
        .header(reqwest::header::AUTHORIZATION, auth_header)
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await?
        .text()
        .await?;

    let token_response = match serde_json::from_str::<TokenResponse>(&response) {
        Ok(response_token) => response_token,
        Err(err) => panic!("Error: {}", err),
    };
    Ok(token_response.access_token)
}
