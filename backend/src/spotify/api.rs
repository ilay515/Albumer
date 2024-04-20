use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use url::Url;

const SPOTIFY_URL: &str = "https://api.spotify.com/v1/";

async fn query_api(access_token: &str, url: Url) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
    );

    let response = client.get(url.as_str()).headers(headers).send().await?;
    Ok(response)
}

#[allow(dead_code)]
pub async fn get_resource(
    access_token: &str,
    resource_name: &str,
) -> Result<String, reqwest::Error> {
    let url: Url = Url::parse(SPOTIFY_URL)
        .unwrap()
        .join(resource_name)
        .unwrap();

    println!("{:?}", url.to_string());
    let response = query_api(access_token, url).await?.text().await?;
    Ok(response)
}

#[allow(dead_code)]
pub async fn get_resource_by_ids(
    access_token: &str,
    resource_name: &str,
    ids: &str,
) -> Result<String, reqwest::Error> {
    let mut url: Url = Url::parse(SPOTIFY_URL)
        .unwrap()
        .join(resource_name)
        .unwrap();

    let query_params = format!("ids={}", ids);
    url.set_query(Some(&query_params));

    println!("{:?}", url.to_string());
    let response = query_api(access_token, url).await?.text().await?;
    Ok(response)
}
