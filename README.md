# Uomi.ai Meme Calculator Agent

This repository contains the source code for a self-contained AI agent built for the [uomi.ai](https://uomi.ai/) platform.

The agent performs a simple calculation: it takes two numbers (`a` and `b`) and an optional text string, then returns a message and the sum of the two numbers.

The core logic is written in Rust and compiled to a single, `no_std` compliant WebAssembly (WASM) file.

---

## 🚀 Getting Started: A Step-by-Step Guide

This guide will walk you through building the final `.wasm` file from scratch.

### Prerequisites

You will need the following tools installed on your system:

* **Git:** For downloading the repository.
* **Rust & Cargo:** The programming language and its build tool.

If you don't have them, you can install them with these commands:

```bash
# Install Git, curl, and build tools (for Debian/Ubuntu)
sudo apt update && sudo apt install git curl build-essential -y

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Step 1: Clone the Repository

Download the source code from this repository to your local machine.

```bash
git clone https://github.com/sinascorpion/meme-trend-agent.git
cd meme-trend-agent
```

### Step 2: Build the Project

Compile the Rust code into a WebAssembly file.

```bash
cargo build --target wasm32-unknown-unknown --release
```

This command creates a `target` directory. Your final, ready-to-upload file is located at: **`target/wasm32-unknown-unknown/release/meme_trend_agent.wasm`**.

---

## 📝 Schemas

The agent uses the following JSON schemas for its inputs and outputs.

### Input Schema (JSON)

```json
{
  "type": "object",
  "properties": {
    "a": {
      "type": "integer",
      "description": "First number for calculation"
    },
    "b": {
      "type": "integer",
      "description": "Second number for calculation"
    },
    "text": {
      "type": "string",
      "description": "Optional text message to pass to the Agent"
    }
  },
  "required": ["a", "b"]
}
```

### Output Schema (JSON)

```json
{
  "type": "object",
  "properties": {
    "message": {
      "type": "string",
      "description": "The output message from the Agent"
    },
    "calculation_result": {
      "type": "integer",
      "description": "The result of adding a and b"
    }
  },
  "required": ["message", "calculation_result"]
}
