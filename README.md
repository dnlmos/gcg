# 🚀 [G]it [C]ommit [G]enerate (gcg) — Auto-generate Git Commit Messages with AI

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.88+-orange?logo=rust)](https://www.rust-lang.org/)

gcg is a fast and lightweight CLI tool written in **Rust** that uses **LLMs** to automatically generate meaningful Git commit messages from your code changes.

---

## ✨ Features

- 🔍 Parses your `git diff` files and suggests a relevant commit message.
- 🤖 Supports **multiple LLM providers**:
  - Google **Gemini** (free tier)
  - Local models via **Ollama**
  - **OpenAI-compatible APIs** (Such as LM Studio)
- ⚡ Fast and efficient — written in Rust

---

## 🔧 Setup

1. **Clone the repository and build:**

```bash
git clone https://github.com/daniel-mosko/gcg.git
cd gcg
cargo build --release
```

## Set your API keys and endpoints:
Create a .env file in the project root:

```
GEMINI_API_KEY="your_gemini_api_key"
OPENAI_API_URL="http://127.0.0.1:1234/v1/chat/completions"
OLLAMA_API_URL="http://127.0.0.1:11434/api/generate"
```
💡 You can use one or all of the providers. The tool will use the one you specify via CLI or configuration.

---

## Usage

```bash
gcg "repo/path" -p gemini
gcg "repo/path" -p ollama -m llama-3.2-3b-instruct:latest
```

## Example response (based on the model)
```
feat: Add error handling to auth middleware
- Refactored login logic in auth.rs
- Added error messages for invalid tokens
```
