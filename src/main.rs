use dotenv::dotenv;
use serde_json::json;
use std::{env, error::Error};

use crate::{
    git::{diff, get_changed_files, open_repo},
    schemas::{ChatResponse, GeminiResponse, UserMessage},
};

mod git;
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

async fn handle_gemini_request(
    client: &reqwest::Client,
    messages: &[UserMessage],
) -> Result<(), Box<dyn Error>> {
    let msgs = messages
        .iter()
        .map(|msg| msg.content.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    let request_payload = json!({
          "contents": [
            {
              "parts": [
                {
                  "text": msgs
                }

              ]
            }
          ],
    });

    dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY")?;

    let response: GeminiResponse= client
                .post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",api_key))
                .json(&request_payload)
                .send()
                .await?
                .json()
                .await?;

    let raw_response = &response.candidates[0].content.parts[0].text;
    println!("{}", raw_response);
    Ok(())
}

async fn handle_openai_request(
    client: &reqwest::Client,
    messages: &[UserMessage],
) -> Result<(), Box<dyn Error>> {
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

    let response: ChatResponse = client
        .post("http://127.0.0.1:1234/v1/chat/completions")
        .json(&request_payload)
        .send()
        .await?
        .json()
        .await?;

    let raw_response = &response.choices[0].message.content;
    println!("{}", raw_response);
    Ok(())
}
