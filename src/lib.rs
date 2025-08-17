// src/lib.rs
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;

// Load environment variables from .env file
use dotenvy::dotenv;

#[derive(Serialize, Deserialize)]
pub struct MemeTrendRequest {
    #[serde(rename = "numberOfMemes")]
    pub number_of_memes: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MemeTrendResponse {
    #[serde(rename = "trendingMemes")]
    pub trending_memes: Vec<Meme>,
}

#[derive(Serialize, Deserialize)]
pub struct Meme {
    pub rank: i32,
    pub name: String,
    pub description: String,
    pub source: String,
    #[serde(rename = "investmentAnalysis")]
    pub investment_analysis: String,
    #[serde(rename = "investmentSuggestion")]
    pub investment_suggestion: String,
}

#[wasm_bindgen]
pub async fn analyze_meme_trends(input: String) -> String {
    // This line loads the .env file
    dotenv().ok();
    
    // This macro reads the API_KEY from the environment at compile time
    let api_key = env!("APIFY_API_KEY");

    let request: MemeTrendRequest = serde_json::from_str(&input).unwrap();
    let url = format!(
        "https://api.apify.com/v2/acts/muhammetakkurtt~dexscan-meme-explorer-scraper/run-sync-get-dataset-items?token={}",
        api_key
    );
    let client = reqwest::Client::new();
    let api_input = serde_json::json!({
        "category": "newCreations",
        "limit": request.number_of_memes,
    });
    let res = client.post(&url).json(&api_input).send().await.unwrap().text().await.unwrap();
    let api_response: serde_json::Value = serde_json::from_str(&res).unwrap();
    let memes_data = api_response.as_array().unwrap();
    let mut trending_memes = Vec::new();
    for (i, meme_data) in memes_data.iter().enumerate() {
        let name = meme_data["name"].as_str().unwrap_or("N/A").to_string();
        let description = meme_data["description"].as_str().unwrap_or("No description available.").to_string();
        let source = meme_data["url"].as_str().unwrap_or("").to_string();
        let volume = meme_data["volume"].as_f64().unwrap_or(0.0);
        let liquidity = meme_data["liquidity"].as_f64().unwrap_or(0.0);
        let (investment_analysis, investment_suggestion) = if volume > 100000.0 && liquidity > 50000.0 {
            ("This meme coin has high trading volume and good liquidity, indicating strong market interest.".to_string(), "High Potential".to_string())
        } else if volume > 50000.0 && liquidity > 20000.0 {
            ("This meme coin has moderate trading volume and liquidity. It shows some promise but carries risk.".to_string(), "Medium Potential".to_string())
        } else {
            ("This meme coin has low trading volume and liquidity, making it a high-risk investment.".to_string(), "Low Potential".to_string())
        };
        trending_memes.push(Meme {
            rank: (i + 1) as i32,
            name,
            description,
            source,
            investment_analysis,
            investment_suggestion,
        });
    }
    let response = MemeTrendResponse { trending_memes };
    serde_json::to_string(&response).unwrap()
}