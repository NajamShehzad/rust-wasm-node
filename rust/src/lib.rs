use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use reqwest::Client;
use image::{ImageFormat, ImageOutputFormat, io::Reader as ImageReader};
use std::io::Cursor;
use base64::encode;


#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub async fn get_image_base64_from_url(url: &str) -> Result<JsValue, JsValue> {
    let client = Client::new();
    println!("Downloading image from: {}", url);
    let resp = client.get(url).send().await.map_err(|e| JsValue::from_str(&e.to_string()))?;

    if resp.status().is_success() {
        let bytes = resp.bytes().await.map_err(|e| JsValue::from_str(&e.to_string()))?;
        let cursor = Cursor::new(bytes.clone()); // Clone the bytes for potential Base64 encoding
        let img_reader = ImageReader::new(cursor.clone()).with_guessed_format().map_err(|e| JsValue::from_str(&e.to_string()))?;
        if let Some(format) = img_reader.format() {
            println!("Image format determined as {:?}", format);
            let image = img_reader.decode().map_err(|e| JsValue::from_str(&e.to_string()))?;
            match format {
                ImageFormat::Jpeg => {
                    // If the image is already JPEG, directly encode to Base64
                    let base64 = encode(&bytes); // Use the cloned bytes
                    println!("Encoding JPEG image to base64");
                    let result = serde_json::json!({"success": true, "base64": base64, "imageType": "jpeg"});
                    Ok(JsValue::from_serde(&result).unwrap())
                },
                _ => {
                    // For non-JPEG images, convert to JPEG before encoding
                    let mut buffer = Cursor::new(Vec::new());
                    image.write_to(&mut buffer, ImageOutputFormat::Jpeg(80)).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    let base64 = encode(buffer.get_ref());
                    println!("Converted image to JPEG and encoded to base64");
                    let result = serde_json::json!({"success": true, "base64": base64, "imageType": "jpeg"});
                    Ok(JsValue::from_serde(&result).unwrap())
                }
            }
        } else {
            println!("Could not determine the image format, attempting to decode as JPEG");
            // Try to force decode as JPEG if the format couldn't be determined
            let cursor = Cursor::new(bytes);
            match ImageReader::with_format(cursor, ImageFormat::Jpeg).decode() {
                Ok(image) => {
                    let mut buffer = Cursor::new(Vec::new());
                    println!("Forced JPEG decoding and encoded to base64");
                    image.write_to(&mut buffer, ImageOutputFormat::Jpeg(80)).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    let base64 = encode(buffer.get_ref());
                    let result = serde_json::json!({"success": true, "base64": base64, "imageType": "jpeg"});
                    Ok(JsValue::from_serde(&result).unwrap())
                },
                Err(e) => {
                    println!("Failed to force decode image: {}", e);
                    Err(JsValue::from_str(&e.to_string()))
                }
            }
        }
    } else {
        Err(JsValue::from_str("Failed to download image or image is not available"))
    }
}
