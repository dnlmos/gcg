use crate::schemas::Provider;
use crate::utils::load_config;
use anyhow::Result;
use clap::Parser;
use colored::*;
use reqwest::blocking::Client;
use std::path::PathBuf;

use crate::{
    git::{Repository, diff, get_changed_files},
    requests::{handle_gemini_request, handle_ollama_request, handle_openai_request},
    schemas::{Config, UserMessage},
};

mod git;
mod requests;
mod schemas;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("."))]
    repo_path: String,
    // #[arg(short, long, default_value_t = String::from("gemini"))]
    // provider: String,
    // #[arg(short, long, default_value_t = String::from("llama-3.2-3b-instruct:latest"))]
    // model: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let repo_path = PathBuf::from(args.repo_path);
    // let provider_name = args.provider;

    let config: Config = load_config(&repo_path)?;

    let repo = Repository::open(&repo_path).unwrap();

    let mut files: Vec<String> = Vec::new();
    get_changed_files(&repo).iter().for_each(|path_bufs| {
        path_bufs.iter().for_each(|changed_file| {
            files.push(changed_file.display().to_string());
        });
    });

    if files.is_empty() {
        println!("{}", "⚠️  No staged files found".bright_yellow().bold());
        println!(
            "{}",
            "Hint: Use `git add <file>` to stage changes.".dimmed()
        );
        return Ok(());
    }

    let system_msg = UserMessage {
        role: String::from("system"),
        content: format!("{}", config.prompt_template),
    };

    let send_msg = UserMessage {
        role: String::from("user"),
        content: diff(&repo, &files).unwrap(),
    };

    let messages = vec![system_msg, send_msg];

    let client = Client::new();

    match config.provider.name.as_str() {
        "gemini" => {
            let _ = handle_gemini_request(&client, &messages, config.provider);
        }
        "openai" => {
            let _ = handle_openai_request(&client, &messages, config.provider);
        }
        "ollama" => {
            let _ = handle_ollama_request(&client, &messages, config.provider);
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Unknown provider: {}",
                config.provider.name
            ));
        }
    };

    Ok(())
}
