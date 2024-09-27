mod token;
mod upload;
use crate::token::get_oauth_token;
use crate::upload::upload_image_to_gcs;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Step 1: Get the OAuth2 token
    let access_token = get_oauth_token().await?;

    // Step 2: Upload the image
    let bucket_name = env::var("Bucket_Name").expect("Bucket_Name must be set");
    let object_name = env::var("Object_Name").expect("Object_Name must be set");
    let image_path = env::var("Image_Path").expect("Image_Path must be set");

    upload_image_to_gcs(&bucket_name, &object_name, &image_path, &access_token).await?;

    Ok(())
}
