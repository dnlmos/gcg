use anyhow::Result;
use clap::Parser;
use reqwest::blocking::Client;

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

fn main() -> Result<()> {
    let args = Args::parse();

    let repo_path = args.repo_path;
    let provider = args.provider;

    let repo = Repository::open(repo_path).unwrap();

    let mut files: Vec<String> = Vec::new();
    get_changed_files(&repo).iter().for_each(|path_bufs| {
        path_bufs.iter().for_each(|changed_file| {
            files.push(changed_file.display().to_string());
        });
    });

    if !files.is_empty() {
        let system_msg = UserMessage {
            role: "system".to_string(),
            content:
                "You are an AI assistant that generates concise, short and clear Git commit messages from code diffs: \n".to_string(),
        };

        let send_msg = UserMessage {
            role: "user".to_string(),
            content: diff(&repo, &files).unwrap(),
        };

        let messages = vec![system_msg, send_msg];

        let client = Client::new();
        if provider == "gemini" {
            let _ = handle_gemini_request(&client, &messages);
        } else {
            let _ = handle_openai_request(&client, &messages);
        }
        // println!("Sending JSON:\n{}", serde_json::to_string_pretty(&request_payload).unwrap());
    }
    Ok(())
}
