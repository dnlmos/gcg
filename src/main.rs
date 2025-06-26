use std::{env, error::Error};

use crate::{
    git::{diff, get_changed_files, open_repo},
    requests::{handle_gemini_request, handle_openai_request},
    schemas::UserMessage,
};

mod git;
mod requests;
mod schemas;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // if path not provided, use current working dir
    let repo_path = env::args().nth(1).unwrap_or_else(|| {
        env::current_dir()
            .expect("Failed to get current directory")
            .to_string_lossy()
            .to_string()
    });
    let repo = open_repo(&repo_path);

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

        let messages: Vec<UserMessage> = vec![system_msg, send_msg];

        let client = reqwest::Client::new();
        if use_gemini {
            handle_gemini_request(&client, &messages).await?;
        } else {
            handle_openai_request(&client, &messages).await?;
        }
        // println!("Sending JSON:\n{}", serde_json::to_string_pretty(&request_payload).unwrap());
    }
    Ok(())
}
