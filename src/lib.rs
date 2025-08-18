use std::mem;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

// Serde is used for processing the JSON input and output
use serde::{Serialize, Deserialize};
use serde_json;

// --- Input and Output Structures ---

#[derive(Serialize, Deserialize)]
pub struct MemeAnalysisRequest {
    #[serde(rename = "memeDescription")]
    pub meme_description: String,
}

#[derive(Serialize, Deserialize)]
pub struct MemeAnalysisResponse {
    pub archetype: String,
    pub analysis: String,
    #[serde(rename = "investmentSuggestion")]
    pub investment_suggestion: String,
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
        MemeArchetype {
            name: "Reaction Image/GIF",
            keywords: vec!["face", "reaction", "surprised", "happy", "sad", "angry", "confused", "thinking", "nodding", "crying"],
            analysis: "Reaction-based memes are highly versatile and have long-term utility in online conversations. Their value is stable but rarely explosive.",
            suggestion: "Low Potential",
        },
        MemeArchetype {
            name: "Viral Challenge / Dance",
            keywords: vec!["dance", "challenge", "song", "music", "tiktok", "trend", "movement"],
            analysis: "Viral challenges have a very high but short-lived peak. They can generate massive, rapid interest but fade just as quickly. High risk, high reward.",
            suggestion: "High Potential",
        },
        MemeArchetype {
            name: "Wholesome / Positive Meme",
            keywords: vec!["wholesome", "cute", "happy", "positive", "dog", "cat", "friendship", "love", "kindness"],
            analysis: "Wholesome content has a dedicated audience and consistent engagement. It fosters a positive community but rarely achieves explosive viral status.",
            suggestion: "Medium Potential",
        },
        MemeArchetype {
            name: "Cursed Image / Absurdist Humor",
            keywords: vec!["cursed", "weird", "strange", "bizarre", "surreal", "odd", "unsettling", "deep fried"],
            analysis: "Absurdist and 'cursed' humor appeals to niche, highly-online communities. These can become cult classics with a dedicated following, but have limited mainstream appeal.",
            suggestion: "Speculative",
        },
        MemeArchetype {
            name: "Exploitable Format",
            keywords: vec!["format", "template", "comic", "drake", "distracted boyfriend", "spongebob", "panel", "label"],
            analysis: "Exploitable formats are the backbone of meme creation. A successful new format can dominate the cultural landscape for weeks or months, offering significant opportunities.",
            suggestion: "High Potential",
        },
        MemeArchetype {
            name: "Niche Hobby / Fandom Meme",
            keywords: vec!["gaming", "anime", "movie", "d&d", "warhammer", "coding", "sports", "star wars"],
            analysis: "These memes are highly relevant within their specific communities but have almost no value outside of them. Their potential is directly tied to the size and engagement of their niche.",
            suggestion: "Speculative",
        },
    ]
}

/// This is the main entry point for the Uomi agent.
/// It takes a pointer to the input data and its length,
/// and returns a pointer to the C-string output.
#[no_mangle]
pub extern "C" fn run(ptr: *const u8, len: usize) -> *const c_char {
    // Convert the input pointer and length into a Rust string slice
    let input_slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let input_json = std::str::from_utf8(input_slice).unwrap_or("");

    // Deserialize the input JSON
    let request: MemeAnalysisRequest = serde_json::from_str(input_json).unwrap_or_else(|_| MemeAnalysisRequest {
        meme_description: String::new(),
    });

    // --- Core Analysis Logic ---
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

    // Serialize the output response to a JSON string
    let result_string = serde_json::to_string(&response).unwrap();
    
    // Convert the Rust string into a C-compatible string
    let c_string = CString::new(result_string).unwrap();
    let ptr = c_string.as_ptr();

    // IMPORTANT: We "forget" the CString so that Rust doesn't deallocate the memory.
    // The Uomi platform is now responsible for freeing this memory.
    mem::forget(c_string);

    // Return the pointer to the output string
    ptr
}
