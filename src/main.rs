use anyhow::Result;
use std::fs;

use clap::Parser;
use dotenv::dotenv;
use reqwest::blocking::Client;
use std::env;

use crate::{
    git::{Repository, diff, get_changed_files},
    requests::{handle_gemini_request, handle_ollama_request, handle_openai_request},
    schemas::{PromptConfig, UserMessage},
};

mod git;
mod requests;
mod schemas;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("."))]
    repo_path: String,
    #[arg(short, long, default_value_t = String::from("gemini"))]
    provider: String,
    #[arg(short, long, default_value_t = String::from("llama-3.2-3b-instruct:latest"))]
    model: String,
}

#[derive(Clone)]
struct Provider {
    name: String,
    api_url: String,
    api_key: Option<String>,
    model: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let repo_path = args.repo_path;
    let provider_name = args.provider;

    let provider = match provider_name.as_str() {
        "gemini" => {
            dotenv().ok();
            let api_url = String::from(
                "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key=",
            );
            let api_key = env::var("GEMINI_API_KEY")?;
            Provider {
                name: provider_name,
                api_url: api_url,
                api_key: Some(api_key),
                model: None,
            }
        }
        "openai" => {
            dotenv().ok();
            let api_url = env::var("OPENAI_API_URL")?;
            Provider {
                name: provider_name,
                api_url: api_url,
                api_key: None,
                model: None,
            }
        }
        "ollama" => {
            dotenv().ok();
            let api_url = env::var("OLLAMA_API_URL")?;
            Provider {
                name: provider_name,
                api_url: api_url,
                api_key: None,
                model: Some(String::from("llama-3.2-3b-instruct:latest")),
            }
        }
        _ => return Err(anyhow::anyhow!("Unknown provider: {}", provider_name)),
    };

    let repo = Repository::open(repo_path).unwrap();

    let mut files: Vec<String> = Vec::new();
    get_changed_files(&repo).iter().for_each(|path_bufs| {
        path_bufs.iter().for_each(|changed_file| {
            files.push(changed_file.display().to_string());
        });
    });

    if !files.is_empty() {
        let file_content = fs::read_to_string("prompt_template.yaml")?;
        let prompt_config: PromptConfig = serde_yaml::from_str(&file_content)?;
        let system_msg = UserMessage {
            role: String::from("system"),
            content: format!("{}", prompt_config.prompt_template),
        };

        let send_msg = UserMessage {
            role: String::from("user"),
            content: diff(&repo, &files).unwrap(),
        };

        let messages = vec![system_msg, send_msg];

        let client = Client::new();

        match provider.name.as_str() {
            "gemini" => {
                let _ = handle_gemini_request(&client, &messages, provider);
            }
            "openai" => {
                let _ = handle_openai_request(&client, &messages, provider);
            }
            "ollama" => {
                let _ = handle_ollama_request(&client, &messages, provider);
            }
            _ => return Err(anyhow::anyhow!("Unknown provider: {}", provider.name)),
        };
    }

    Ok(())
}
