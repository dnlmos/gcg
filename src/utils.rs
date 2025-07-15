use anyhow::Result;

use colored::*;
use keyring::Entry;
use rpassword::read_password;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::schemas::Config;
use crate::schemas::Provider;

pub fn load_config(repo_path: &Path) -> Result<Config> {
    // Define config file paths in priority order
    let config_paths = [
        repo_path.join("gcg.yaml"),                 // Project config
        get_xdg_config_home().join("gcg/gcg.yaml"), // Global config
    ];

    // Try each config path in order
    for path in &config_paths {
        match fs::read_to_string(path) {
            Ok(content) => match serde_yaml::from_str::<Config>(&content) {
                Ok(config) => return Ok(config),
                Err(e) => {
                    _ = format!("Warning: Failed to parse config at {{path.display()}}: {e}");
                    continue;
                }
            },
            Err(_) => continue, // File doesn't exist or can't be read, try next
        }
    }

    Ok(get_default_config())
}

pub fn get_xdg_config_home() -> PathBuf {
    // Get the XDG_CONFIG_HOME environment variable, defaulting to ~/.config if not set
    let xdg_config_home = env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
        // If XDG_CONFIG_HOME is not set, use the default path
        let home = env::var("HOME").unwrap_or_else(|_| String::from("/home/user")); // Fallback if HOME is also not set
        format!("{home}/.config")
    });

    PathBuf::from(xdg_config_home)
}

pub fn get_default_config() -> Config {
    Config {
        provider: Provider {
            name: "ollama".to_string(),
            api_url: "http://127.0.0.1:11434/api/generate".to_string(),
            model: Some("llama-3.2-3b-instruct:latest".to_string()),
        },
        prompt_template: String::from(
            " You are an AI assistant that generates concise, short and clear Git commit messages from code diffs.\n--- **Guidelines for Commit Messages:**\n* Start with a **type** (e.g., `feat`, `fix`, `docs`, `refactor`, `chore`) followed by a colon and a space, then the **subject**.\n * The subject line should be **imperative**, **50 characters or less**, and concisely describe the change.\n * Optionally, include a blank line followed by a **body** with bullet points (`-`). Each bullet point should clearly explain a specific aspect of the change.\n * Focus strictly on the changes presented in the diff.\n --- **Code Diff to Analyze:** ",
        ),
    }
}

/// Get API key from keyring, prompting user if not found
pub fn get_api_key(service: String, key_name: String) -> Result<String> {
    let entry = Entry::new(&service, &key_name)?;

    // Try to retrieve stored password
    match entry.get_password() {
        Ok(password) => Ok(password),
        _ => {
            // Prompt user
            println!(
                "{} API key for '{}' not found in keyring.",
                "✗".red().bold(),
                key_name.bright_green().bold()
            );

            print!(
                "{} Please enter your {} API key: ",
                "→".blue(),
                key_name.yellow().bold()
            );
            std::io::Write::flush(&mut std::io::stdout())?;

            let api_key = read_password()?;
            if api_key.trim().is_empty() {
                return Err(anyhow::anyhow!("API key cannot be empty"));
            }

            // Store the API key securely
            let _ = entry.set_password(api_key.trim());

            println!("✓ Saved {key_name} API key to keyring");
            Ok(api_key.trim().to_string())
        }
    }
}
