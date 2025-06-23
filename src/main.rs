use serde_json::json;
use serde::{Deserialize, Serialize};
use std::error::Error;


use crate::git::{diff, get_changed_files, open_repo};

mod git;

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Deserialize)]
struct ChatMessage {
    content: String,
}

#[derive(Serialize)] // to be able to convert to JSON
struct Message {
    role: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let repo_path = String::from(".");
    let repo = open_repo(&repo_path);

    let mut files: Vec<String> = Vec::new();

    match get_changed_files(&repo) {
        Ok(changed_files) => {
            for file in changed_files {
                files.push(file.display().to_string());
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    let system_msg = Message {
        role: "system".to_string(),
        content: 
            "You are an AI assistant that generates concise, short and clear Git commit messages from code diffs: \n".to_string(),
    };

    let send_msg = Message {
        role: "user".to_string(),
        content: diff(&repo, &files).unwrap(),
    };

    let mut messages: Vec<Message> = vec![];
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


    let raw_content = &response.choices[0].message.content;
    println!("{}", raw_content);

    Ok(())
}
