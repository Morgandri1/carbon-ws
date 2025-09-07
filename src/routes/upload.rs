use std::io::Cursor;
use reqwest::Client as HttpClient;
use image::ImageFormat;
use uuid::Uuid;

use crate::{constants::BUCKET_URL, result::{CarbonError, CarbonResult}};

/// Upload a PNG buffer 
pub async fn upload_png_buffer(user_id: Uuid, buf: Vec<u8>) -> CarbonResult<String> {
    let client = HttpClient::new();
    let resp = client.post(BUCKET_URL.as_str().to_string() + "upload")
        .header("Content-Type", "image/png")
        .header("filename", user_id.to_string() + ".png")
        .body(buf)
        .send()
        .await
        .map_err(|_| CarbonError::InternalError { message: "Failed to upload image".to_string() })?;
    if !resp.status().is_success() {
        return Err(CarbonError::InternalError { message: "Failed to upload image".to_string() });
    }
    Ok(BUCKET_URL.as_str().to_string() + "download/" + &user_id.to_string() + ".png")
}

/// Download an image from a URL and convert it to PNG
///
/// # Arguments
/// * `image_url` - The URL of the image to download
pub async fn download_image_as_png(image_url: &str) -> CarbonResult<Vec<u8>> {
    let client = HttpClient::new();
    let resp = client.get(image_url).send().await.map_err(|_| 
        CarbonError::InternalError { message: "Failed to fetch image".to_string() }
    )?;
    if !resp.status().is_success() {
        return Err(CarbonError::InternalError { message: "Failed to fetch image".to_string() });
    }
    let bytes = resp.bytes().await.map_err(|_| {
        CarbonError::InternalError { message: "Failed to read image bytes".to_string() }
    })?;

    // Convert to PNG using the image crate
    let img = image::load_from_memory(&bytes)
        .map_err(|_| CarbonError::InternalError { message: "Image decode error".to_string() })?;
    let mut png_buf = Cursor::new(Vec::new());
    img.write_to(&mut png_buf, ImageFormat::Png).map_err(|_| CarbonError::SerializerError)?;
    Ok(png_buf.into_inner())
}