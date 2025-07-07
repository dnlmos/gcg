use anyhow::Result;
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

fn main() -> Result<()> {
    // if path not provided, use current working dir
    let repo_path = env::args().nth(1).unwrap_or_else(|| {
        env::current_dir()
            .expect("Failed to get current directory")
            .to_string_lossy()
            .to_string()
    });
    let repo = Repository::open(repo_path).unwrap();

    let use_gemini = true;

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
        if use_gemini {
            let _ = handle_gemini_request(&client, &messages);
        } else {
            let _ = handle_openai_request(&client, &messages);
        }
        // println!("Sending JSON:\n{}", serde_json::to_string_pretty(&request_payload).unwrap());
    }
    Ok(())
}
