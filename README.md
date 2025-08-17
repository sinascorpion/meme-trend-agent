# Meme Trend Analysis AI Agent

This repository contains the source code for an AI agent built to run on [uomi.ai](https://uomi.ai/). The agent analyzes trending memes from the market and provides investment suggestions based on metrics like trading volume and liquidity.

The core logic is written in Rust and compiled to WebAssembly (WASM).

## 🚀 Getting Started: A Step-by-Step Guide

This guide will walk you through setting up the project and building the final `.wasm` file from scratch.

### Prerequisites

Before you begin, make sure you have the following tools installed on your system:

* **Git:** For downloading the repository.
* **Rust:** The programming language used for the agent.
* **wasm-pack:** A tool for building Rust-generated WebAssembly.

If you don't have them, you can install them with these commands:

```bash
# Install Git, curl, and build tools (for Debian/Ubuntu)
sudo apt update && sudo apt install git curl build-essential -y

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
source $HOME/.cargo/env

# Install wasm-pack
cargo install wasm-pack
```

### Step 1: Clone the Repository

First, download the source code from this repository to your local machine using Git.

```bash
git clone https://github.com/sinascorpion/meme-trend-agent.git
cd meme-trend-agent
```

### Step 2: Configure Your API Key

The agent needs an [Apify](https://apify.com) API key to work. You'll store this key in a local `.env` file

1.  **Create the `.env` file** in the root of the project directory.

    ```bash
    nano .env
    ```

2.  **Add your API key** to the file. Replace `your-api-key-goes-here` with your actual key from Apify.

    ```
    APIFY_API_KEY=your-api-key-goes-here
    ```

3.  **Save and exit** the file (`Ctrl+X`, then `Y`, then `Enter`).

### Step 3: Build the Project

Now you can compile the Rust code into a WebAssembly file. This command reads your API key from the `.env` file and securely includes it in the final build.

```bash
wasm-pack build --target web
```

This process will create a `pkg` directory. Your final, ready-to-upload file is located at: **`pkg/meme_trend_agent_bg.wasm`**.

## 📝 Schemas

The agent communicates using a defined JSON schema for its inputs and outputs.

### Input Schema (JSON)

This is the structure of the request the agent expects.

```json
{
  "$schema": "[http://json-schema.org/draft-07/schema#](http://json-schema.org/draft-07/schema#)",
  "title": "MemeTrendRequest",
  "description": "A request to get the top trending memes for investment analysis.",
  "type": "object",
  "properties": {
    "numberOfMemes": {
      "description": "The number of top trending memes to analyze.",
      "type": "integer",
      "minimum": 1,
      "maximum": 20,
      "default": 5
    }
  },
  "required": ["numberOfMemes"]
}
```

### Output Schema (JSON)

This is the structure of the response the agent will provide.

```json
{
  "$schema": "[http://json-schema.org/draft-07/schema#](http://json-schema.org/draft-07/schema#)",
  "title": "MemeTrendResponse",
  "description": "A response containing a list of trending memes and investment suggestions.",
  "type": "object",
  "properties": {
    "trendingMemes": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "rank": {
            "type": "integer"
          },
          "name": {
            "type": "string"
          },
          "description": {
            "type": "string"
          },
          "source": {
            "type": "string",
            "format": "uri"
          },
          "investmentAnalysis": {
            "type": "string"
          },
          "investmentSuggestion": {
            "type": "string",
            "enum": ["High Potential", "Medium Potential", "Low Potential", "Not Recommended"]
          }
        },
        "required": ["rank", "name", "description", "source", "investmentAnalysis", "investmentSuggestion"]
      }
    }
  },
  "required": ["trendingMemes"]
}
