use std::mem;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};

// The serde libraries are still needed for JSON processing
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

// --- Internal Data Structures ---

struct MemeArchetype {
    name: &'static str,
    keywords: Vec<&'static str>,
    analysis: &'static str,
    suggestion: &'static str,
}

// --- The Agent's Internal Knowledge Base ---

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

// --- Core Logic Function ---
// This function takes a standard Rust string and returns a standard Rust string.
fn run_analysis(input_json: &str) -> String {
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

    serde_json::to_string(&response).unwrap()
}

// --- Memory Management and Entry Point for the Uomi Platform ---

static mut INPUT_LEN: usize = 0;
static mut INPUT_PTR: *mut u8 = 0 as *mut u8;
static mut OUTPUT_PTR: *const c_char = 0 as *const c_char;

/// This is the function the Uomi platform will call to run your agent.
#[no_mangle]
pub extern "C" fn run() {
    let input_json = unsafe {
        let slice = std::slice::from_raw_parts(INPUT_PTR, INPUT_LEN);
        std::str::from_utf8(slice).unwrap_or("")
    };
    
    let result_string = run_analysis(input_json);
    let c_string = CString::new(result_string).unwrap();

    unsafe {
        OUTPUT_PTR = c_string.as_ptr();
    }
    
    mem::forget(c_string);
}

/// The platform calls this to get the result after `run()` has finished.
#[no_mangle]
pub extern "C" fn get_output() -> *const c_char {
    unsafe { OUTPUT_PTR }
}

/// The platform calls this to prepare a memory buffer for the input.
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    unsafe {
        INPUT_PTR = ptr;
        INPUT_LEN = size;
    }
    ptr as *mut c_void
}

/// The platform can call this to free memory.
#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, size: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, size);
    }
}
