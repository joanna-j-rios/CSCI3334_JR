use serde::Deserialize;
use std::error::Error;

use std::fs;
use std::io::Read;


#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum ApiResult {
    Success(DogImage),
    ApiError(String),
    NetworkError(String),
}


fn fetch_random_dog_image() -> ApiResult {
    let url = "https://dog.ceo/api/breeds/image/random";
    
    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => ApiResult::Success(dog_image),
                    Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                }
            } else {
                ApiResult::ApiError(format!("HTTP error: {}", response.status()))
            }
        },
        Err(e) => {
            let error_details = format!("Request failed: {}", e);
            ApiResult::NetworkError(error_details)
        },
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Fetcher");
    println!("=================\n");

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);
        match fetch_random_dog_image() {
            ApiResult::Success(dog_image) => {
                println!("✅ Success!");
                println!("🖼️ Image URL: {}", dog_image.message);
                println!("📊 Status: {}", dog_image.status);

                //  ---- for download of image ----

                let image_url = dog_image.message;
                let filename = format!("dog_{}.jpg", i); // formating file name for each picture
                println!("   Downloading image ...");
                
                // get the network response for the image URL.
                // same as in fetch_random_dog_image() : match ureq::get(url).call()
                // only using ? so i dont need all the match stuff
                let response = ureq::get(&image_url).call()?; // The '?' propagates network errors

                // read ALL the image data (bytes) from the network stream into memory.
                let image_bytes = response
                    .into_reader()
                    .bytes()
                    .collect::<Result<Vec<u8>, _>>()?; 

                // write bytes from memory to the file
                fs::write(&filename, image_bytes)?; // The '?' propagates file system errors
                
                println!("   Saved successfully as: {}", filename);
                println!("   Download Complete!");

                // note: used the '?' operator for cleaner code as seen in 
                // 'Comprehensive Guide to Error Handling in Rust' section 9

                //  ---- for download of image ----

            },
            ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
            ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
        }
        println!();
    }

    Ok(())
}