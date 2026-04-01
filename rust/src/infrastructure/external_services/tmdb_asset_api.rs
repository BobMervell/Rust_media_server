use anyhow::{anyhow, Context, Result};
use axum::http::{HeaderMap, HeaderValue};
use reqwest::{
    header::{ACCEPT, AUTHORIZATION},
    Client, Response,
};

#[derive(Clone)]

pub struct TmdbAssetApi {
    client: Client,
}

impl TmdbAssetApi {
    pub fn new(token: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        //TODO WARNING HeaderValue::from_str is intended to be replaced in the future by a TryFrom.
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("bearer {}", token))
                .context("Failed to create header value with token")?,
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .context("Failed to build client")?;

        Ok(Self { client })
    }

    pub async fn get_image(&self, format: &str, picture_path: &str) -> Result<Response> {
        let url = format!("https://image.tmdb.org/t/p/{}/{}", format, picture_path);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("Failed to get response for url: {}", &url))?
            .error_for_status()
            .with_context(|| {
                format!(
                    "TMDB returned error status for image: {} , from url: {}",
                    picture_path, &url
                )
            })?;

        if !response.status().is_success() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        if let Some(content_type) = response.headers().get(reqwest::header::CONTENT_TYPE) {
            if let Ok(content_type_str) = content_type.to_str() {
                if !content_type_str.starts_with("image/") {
                    return Err(anyhow!("Response is not an image for url: {}", &url));
                }
            }
        }
        Ok(response)
    }
}
