use anyhow::{anyhow, Context, Result};
use reqwest::{Client, Response};

use crate::infrastructure::os_infra::file_system::{create_dir, save_image};

pub async fn fetch_and_store_image(
    client: &Client,
    picture_path: &str,
    category: &str,
    name: &str,
    subdir: &str,
    image_size: &str,
    placeholder_path: &str,
) -> Result<String> {
    let (created, file_path) = create_dir(category, subdir, name)
        .with_context(|| format!("Error creating directory for: {}", name))?;

    if !created {
        tracing::debug!("Picture path already exists: {}", file_path);
        return Ok(file_path);
    }

    match get_image(client, image_size, picture_path).await {
        Ok(mut image) => {
            save_image(&mut image, &file_path)
                .await
                .with_context(|| format!("Failed to save image for: {}", name))?;

            return Ok(file_path);
        }
        Err(e) => {
            tracing::debug!("Failed to get image for: {}.\n Caused by {}", name, e);
            return Ok(placeholder_path.to_owned());
        }
    }
}

async fn get_image(client: &Client, format: &str, picture_path: &str) -> Result<Response> {
    let url = format!("https://image.tmdb.org/t/p/{}/{}", format, picture_path);

    let response = client
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
    return Ok(response);
}
