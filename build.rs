// build.rs
use std::env;

fn main() {
    // Load .env file
    dotenvy::dotenv().expect("Failed to load .env file");

    // Pass the API key to the main compilation process
    println!(
        "cargo:rustc-env=APIFY_API_KEY={}",
        env::var("APIFY_API_KEY").expect("APIFY_API_KEY not found in .env file")
    );
}
