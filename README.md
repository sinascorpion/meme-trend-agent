# Uomi.ai Meme Archetype Analysis Agent

This repository contains the source code for a self-contained AI agent built specifically for the [uomi.ai](https://uomi.ai/) platform, following its official documentation. This agent **does not require any external API calls**.

The agent analyzes a user-provided description of a meme, classifies it into a known archetype (e.g., "Reaction Image", "Viral Challenge"), and provides a simulated investment potential based on the historical performance patterns of that archetype.

The core logic is written in Rust and compiled to a single, self-contained WebAssembly (WASM) file with the entry points required by the Uomi platform.

---

## 🚀 Getting Started: A Step-by-Step Guide

This guide will walk you through setting up the project and building the final `.wasm` file.

### Prerequisites

You will need the following tools installed on your system:

* **Git:** For downloading the repository.
* **Rust & Cargo:** The programming language and its build tool.

If you don't have them, you can install them with these commands:

```bash
# Install Git, curl, and build tools (for Debian/Ubuntu)
sudo apt update && sudo apt install git curl build-essential -y

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
source $HOME/.cargo/env
```

### Step 1: Clone the Repository

Download the source code from this repository to your local machine.

```bash
git clone [https://github.com/sinascorpion/meme-trend-agent.git](https://github.com/sinascorpion/meme-trend-agent.git)
cd meme-trend-agent
```

### Step 2: Build the Project

Now you can compile the Rust code into a WebAssembly file. This is a simpler build process that doesn't require `wasm-pack`.

```bash
cargo build --target wasm32-unknown-unknown --release
```

This command creates a `target` directory. Your final, ready-to-upload file is located at: **`target/wasm32-unknown-unknown/release/meme_trend_agent.wasm`**.

---

## 📝 Schemas

The agent uses the following JSON schemas for its inputs and outputs.

### Input Schema (JSON)

The agent expects a simple text description of the meme to be analyzed.

```json
{
  "$schema": "[http://json-schema.org/draft-07/schema#](http://json-schema.org/draft-07/schema#)",
  "title": "MemeAnalysisRequest",
  "description": "A request to analyze a meme based on its description.",
  "type": "object",
  "properties": {
    "memeDescription": {
      "description": "A text description of the meme (e.g., 'a cat looking surprised', 'someone dancing to a popular song').",
      "type": "string"
    }
  },
  "required": ["memeDescription"]
}
```

### Output Schema (JSON)

The agent returns the identified archetype and a corresponding analysis.

```json
{
  "$schema": "[http://json-schema.org/draft-07/schema#](http://json-schema.org/draft-07/schema#)",
  "title": "MemeAnalysisResponse",
  "description": "A response containing the analysis of the described meme.",
  "type": "object",
  "properties": {
    "archetype": {
      "type": "string"
    },
    "analysis": {
      "type": "string"
    },
    "investmentSuggestion": {
      "type": "string",
      "enum": ["High Potential", "Medium Potential", "Low Potential", "Speculative"]
    }
  },
  "required": ["archetype", "analysis", "investmentSuggestion"]
}
