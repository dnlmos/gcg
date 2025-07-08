use anyhow::Result;

use clap::Parser;
use dotenv::dotenv;
use reqwest::blocking::Client;
use std::env;

use crate::{
    git::{Repository, diff, get_changed_files},
    requests::{handle_gemini_request, handle_openai_request},
    schemas::UserMessage,
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
}

#[derive(Clone)]
struct Provider {
    name: String,
    api_url: String,
    api_key: Option<String>,
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
            }
        }
        "openai" => {
            dotenv().ok();
            let api_url = env::var("OPENAI_API_URL")?;
            Provider {
                name: provider_name,
                api_url: api_url,
                api_key: None,
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
        let system_msg = UserMessage {
            role: String::from("system"),
            content: String::from(
                "You are an AI assistant that generates concise, short and clear Git commit messages from code diffs: \n",
            ),
        };

        let send_msg = UserMessage {
            role: String::from("user"),
            content: diff(&repo, &files).unwrap(),
        };

        let messages = vec![system_msg, send_msg];

        let client = Client::new();
        if provider.name == "gemini" {
            let _ = handle_gemini_request(&client, &messages, provider);
        } else {
            let _ = handle_openai_request(&client, &messages, provider);
        }
    }
    Ok(())
}
