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
cargo install --path .
```

## Set your API keys and endpoints:
You can provide a configuration file named `gcg.yaml` in one of the following locations:
- In your project repository (local config)
- In the global config directory: `$HOME/.config/gcg/`
- Or rely on the default configuration shown below:

```yaml
provider:
  name: "ollama"
  api_url: "http://127.0.0.1:11434/api/generate"
  model: "llama-3.2-3b-instruct:latest"
prompt_template: |
  You are an AI assistant that generates concise, short and clear Git commit messages from code diffs.

  ---
  **Guidelines for Commit Messages:**
  * Start with a **type** (e.g., `feat`, `fix`, `docs`, `refactor`, `chore`) followed by a colon and a space, then the **subject**.
  * The subject line should be **imperative**, **50 characters or less**, and concisely describe the change.
  * Optionally, include a blank line followed by a **body** with bullet points (`-`). Each bullet point should clearly explain a specific aspect of the change.
  * Focus strictly on the changes presented in the diff.
  ---

  **Code Diff to Analyze:**
```

---

## Usage

```bash
gcg "repo/path"
```

## Example response (based on the model)
```
feat: Add error handling to auth middleware
- Refactored login logic in auth.rs
- Added error messages for invalid tokens
```
