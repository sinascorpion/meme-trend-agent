#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use spin::Mutex; // Import Mutex

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// --- Input and Output Structures ---

#[derive(Serialize, Deserialize)]
struct MemeAnalysisRequest {
    #[serde(rename = "memeDescription")]
    meme_description: String,
}

#[derive(Serialize, Deserialize)]
struct MemeAnalysisResponse {
    archetype: String,
    analysis: String,
    #[serde(rename = "investmentSuggestion")]
    investment_suggestion: String,
}

// --- Internal Data Structures (The Agent's Knowledge Base) ---

struct MemeArchetype {
    name: &'static str,
    keywords: Vec<&'static str>,
    analysis: &'static str,
    suggestion: &'static str,
}

fn get_archetypes() -> Vec<MemeArchetype> {
    vec![
        MemeArchetype { name: "Reaction Image/GIF", keywords: vec!["face", "reaction", "surprised", "happy", "sad", "angry", "confused", "thinking", "nodding", "crying"], analysis: "Reaction-based memes are highly versatile and have long-term utility in online conversations. Their value is stable but rarely explosive.", suggestion: "Low Potential" },
        MemeArchetype { name: "Viral Challenge / Dance", keywords: vec!["dance", "challenge", "song", "music", "tiktok", "trend", "movement"], analysis: "Viral challenges have a very high but short-lived peak. They can generate massive, rapid interest but fade just as quickly. High risk, high reward.", suggestion: "High Potential" },
        MemeArchetype { name: "Wholesome / Positive Meme", keywords: vec!["wholesome", "cute", "happy", "positive", "dog", "cat", "friendship", "love", "kindness"], analysis: "Wholesome content has a dedicated audience and consistent engagement. It fosters a positive community but rarely achieves explosive viral status.", suggestion: "Medium Potential" },
        MemeArchetype { name: "Cursed Image / Absurdist Humor", keywords: vec!["cursed", "weird", "strange", "bizarre", "surreal", "odd", "unsettling", "deep fried"], analysis: "Absurdist and 'cursed' humor appeals to niche, highly-online communities. These can become cult classics with a dedicated following, but have limited mainstream appeal.", suggestion: "Speculative" },
        MemeArchetype { name: "Exploitable Format", keywords: vec!["format", "template", "comic", "drake", "distracted boyfriend", "spongebob", "panel", "label"], analysis: "Exploitable formats are the backbone of meme creation. A successful new format can dominate the cultural landscape for weeks or months, offering significant opportunities.", suggestion: "High Potential" },
        MemeArchetype { name: "Niche Hobby / Fandom Meme", keywords: vec!["gaming", "anime", "movie", "d&d", "warhammer", "coding", "sports", "star wars"], analysis: "These memes are highly relevant within their specific communities but have almost no value outside of them. Their potential is directly tied to the size and engagement of their niche.", suggestion: "Speculative" },
    ]
}

// Use a Mutex for safe, thread-compliant access to the global buffer
static OUTPUT_BUFFER: Mutex<Vec<u8>> = Mutex::new(Vec::new());

/// This is the main entry point for the Uomi agent.
#[no_mangle]
pub extern "C" fn run(ptr: *const u8, len: usize) {
    let input_slice = unsafe { core::slice::from_raw_parts(ptr, len) };
    let input_json = core::str::from_utf8(input_slice).unwrap_or("");

    let request: MemeAnalysisRequest = serde_json::from_str(input_json).unwrap_or_else(|_| MemeAnalysisRequest {
        meme_description: String::new(),
    });

    let archetypes = get_archetypes();
    let mut best_match: Option<&MemeArchetype> = None;
    let mut max_score = 0;
    let description_lower = request.meme_description.to_lowercase();

    for archetype in &archetypes {
        let mut current_score = 0;
        for keyword in &archetype.keywords {
            if description_lower.contains(keyword) {
                current_score += 1;
            }
        }
        if current_score > max_score {
            max_score = current_score;
            best_match = Some(archetype);
        }
    }

    let response = match best_match {
        Some(archetype) if max_score > 0 => MemeAnalysisResponse {
            archetype: archetype.name.to_string(),
            analysis: archetype.analysis.to_string(),
            investment_suggestion: archetype.suggestion.to_string(),
        },
        _ => MemeAnalysisResponse {
            archetype: "Unclassified".to_string(),
            analysis: "The provided description did not match a known meme archetype. This type of meme is highly unpredictable.".to_string(),
            investment_suggestion: "Speculative".to_string(),
        },
    };

    let result_string = serde_json::to_string(&response).unwrap();

    // Lock the buffer and update its contents
    *OUTPUT_BUFFER.lock() = result_string.into_bytes();
}

/// Returns a pointer to the output buffer.
#[no_mangle]
pub extern "C" fn get_output_ptr() -> *const u8 {
    OUTPUT_BUFFER.lock().as_ptr()
}

/// Returns the length of the output buffer.
#[no_mangle]
pub extern "C" fn get_output_len() -> usize {
    OUTPUT_BUFFER.lock().len()
}

/// Required panic handler for `no_std` environments.
#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
