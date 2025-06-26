use dotenv::dotenv;
use serde_json::json;
use std::{env, error::Error};

use crate::schemas::{GeminiResponse, OpenAIResponse, UserMessage};
pub async fn handle_gemini_request(
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

pub async fn handle_openai_request(
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

    let response: OpenAIResponse = client
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
