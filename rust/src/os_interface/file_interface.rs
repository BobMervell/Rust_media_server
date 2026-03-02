use anyhow::{Context, Result};
use reqwest::Response;
use std::{fs, path::Path};
use tokio::io::AsyncWriteExt;

//returns a result tuple with true if the directory was created, and false if it already exists
pub fn create_dir(
    parent_folder_name: &str,
    folder_name: &str,
    file_name: &str,
) -> Result<(bool, String)> {
    let mut save_dir =
        std::env::current_dir().context("Failed to retrieve current working directory")?;
    save_dir.push("images");
    save_dir.push(parent_folder_name);
    save_dir.push(folder_name);
    let full_path = save_dir.join(file_name);

    if full_path.exists() {
        Ok((false, full_path.to_string_lossy().to_string()))
    } else {
        fs::create_dir_all(&save_dir)
            .with_context(|| format!("Failed to create directories for path {:?}", &save_dir))?;
        Ok((true, full_path.to_string_lossy().to_string()))
    }
}

pub async fn save_image(response: &mut Response, image_path: &str) -> Result<()> {
    let image_path = Path::new(&image_path);
    let mut file = tokio::fs::File::create(image_path)
        .await
        .context("Failed to create file for saving image")?;
    while let Some(chunk) = response
        .chunk()
        .await
        .context("Failed to get response chunk")?
    {
        file.write_all(&chunk)
            .await
            .context("Failed to write image file")?;
    }

    Ok(())
}
