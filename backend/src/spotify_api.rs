use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;

async fn get_resource(resource_name: &str, ids: &str, access_token: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();

    // Prepare the request headers with the access token
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
    );
    let url = &format!("https://api.spotify.com/v1/{}?ids={}", resource_name, ids);
    print!("{}", url);
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

pub async fn get_albums(ids: &str) -> Result<String, reqwest::Error> {
    let access_token = "BQAyRxi2HmpMdpGGZ0yhGcwD6EUDnoJrNg7G-Fm5KyJGQnQFcZUrGOWDQfjoTovcq9cOmdjBUw39We4fC9UTooA-xFAFqn9pCWDGhpR8-XOtpTKHavo";
    get_resource("albums", ids, access_token).await
}
