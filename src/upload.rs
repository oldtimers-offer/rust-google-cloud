use reqwest::Client;
use std::error::Error;
use std::fs;

pub async fn upload_image_to_gcs(
    bucket_name: &str,
    object_name: &str,
    image_path: &str,
    access_token: &str,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Read the image file
    let image_data = fs::read(image_path)?;

    // Construct the upload URL
    let url = format!(
        "https://storage.googleapis.com/upload/storage/v1/b/{}/o?uploadType=media&name={}",
        bucket_name, object_name
    );

    // Send the POST request to upload the image
    let response = client
        .post(&url)
        .bearer_auth(access_token)
        .header("Content-Type", "image/jpeg") // Change this to match your image type
        .body(image_data)
        .send()
        .await?;

    if response.status().is_success() {
        println!(
            "Image uploaded successfully to {}/{}",
            bucket_name, object_name
        );
    } else {
        eprintln!("Failed to upload image. Status: {}", response.status());
    }

    Ok(())
}
