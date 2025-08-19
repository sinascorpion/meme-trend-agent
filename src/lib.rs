#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

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
        MemeArchetype { name: "Reaction Image/GIF", keywords: vec!["face", "reaction", "surprised", "happy", "sad", "angry", "confused", "thinking", "nodding", "crying"], analysis: "Reaction
