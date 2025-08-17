# Meme Trend Analysis AI Agent

This repository contains the source code for an AI agent built to run on [uomi.ai](https://uomi.ai/). The agent analyzes trending memes from the market and provides investment suggestions based on metrics like trading volume and liquidity.

The core logic is written in Rust and compiled to WebAssembly (WASM).

---

## 🚀 How to Build

Follow these steps to compile the project and generate the `.wasm` file.

1.  **Install Rust and wasm-pack:**
    If you don't have them, install the necessary tools.
    ```bash
    # Install Rust
    curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
    
    # Install wasm-pack
    cargo install wasm-pack
    ```

2.  **Build the Project:**
    Navigate to the project's root directory and run the build command. This will create a `pkg` directory containing the final `.wasm` file.
    ```bash
    wasm-pack build --target web
    ```
    The final file will be located at `pkg/meme_trend_agent_bg.wasm`.

---

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
