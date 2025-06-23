use serde_json::json;
use std::error::Error;
use std::env::args;


use crate::{git::{diff, get_changed_files, open_repo}, schemas::{ChatResponse, UserMessage}};

mod git;
mod schemas;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let repo_path = args().nth(2).expect("no repository path given");
    let repo = open_repo(&repo_path);

    let mut files: Vec<String> = Vec::new();

    get_changed_files(&repo)
        .iter()
        .for_each(|path_bufs| {
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

        let mut messages: Vec<UserMessage> = vec![];
        messages.push(system_msg);
        messages.push(send_msg);

        let request_payload = json!({
            "model": "{{model}}",
            "messages": messages,
            "response_format": {
                "type": "json_schema",
                "json_schema": {
                    "name": "commit_response",
                    "strict": "true",
                    "schema": {
                        "type": "object",
                        "properties": {
                            "message": {
                                "type": "string"
                            }
                        },
                        "required": ["message"]
                    }
                }
            },
            "temperature": 0.7,
            "max_tokens": 50,
            "stream": false
        });

        // println!("Sending JSON:\n{}", serde_json::to_string_pretty(&request_payload).unwrap());

        let client = reqwest::Client::new();

        let response: ChatResponse = client
            .post("http://127.0.0.1:1234/v1/chat/completions")
            .json(&request_payload)
            .send()
            .await?
            .json()
            .await?;

        let raw_response = &response.choices[0].message.content;
        println!("{}", raw_response);
    };

    Ok(())
}
