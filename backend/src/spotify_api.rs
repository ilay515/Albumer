use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i32
}

pub async fn get_access_token(client_id: String, client_secret: String) -> Result<String, reqwest::Error> {
    let auth_header = format!("Basic {}", base64::encode(format!("{}:{}", client_id, client_secret)));

    let client = Client::new();
    let response = client.post("https://accounts.spotify.com/api/token")
        .header(reqwest::header::AUTHORIZATION, auth_header)
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await?
        .text()
        .await?;

    let token_response = match serde_json::from_str::<TokenResponse>(&response) {
        Ok(response_token) => response_token,
        Err(err) => panic!("Error: {}", err)
    };
    Ok(token_response.access_token)
}

#[allow(dead_code)]
async fn get_resource(resource_name: &str, ids: &str, access_token: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();

    // Prepare the request headers with the access token
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
    );
    let url = &format!("https://api.spotify.com/v1/{}?ids={}", resource_name, ids);
    // Make the request to Spotify API
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await?;

    // Parse and handle the response
    let body = response.text().await?;
    Ok(body)
}

#[allow(dead_code)]
pub async fn get_albums(ids: &str, token: &str) -> Result<String, reqwest::Error> {
    get_resource("albums", ids, token).await
}
