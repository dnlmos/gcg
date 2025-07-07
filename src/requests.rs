use anyhow::Result;
use dotenv::dotenv;
use serde_json::json;
use std::env;

use crate::schemas::{GeminiResponse, OpenAIResponse, UserMessage};

use reqwest::blocking::Client;
pub fn handle_gemini_request(client: &Client, messages: &[UserMessage]) -> Result<()> {
    let msgs = messages
        .iter()
        .map(|msg| msg.content.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    let request_payload = json!({ "contents": [ { "parts": [ { "text": msgs } ] } ], });

    dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY")?;

    let api_url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key=";
    let response: GeminiResponse = client
        .post(format!("{api_url}{api_key}"))
        .json(&request_payload)
        .send()?
        .json()?;

    let raw_response = &response.candidates[0].content.parts[0].text;
    println!("{}", raw_response);
    Ok(())
}

pub fn handle_openai_request(client: &Client, messages: &[UserMessage]) -> Result<()> {
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

    dotenv().ok();
    let api_url = env::var("OPENAI_API_URL")?;
    let response: OpenAIResponse = client
        .post(format!("{api_url}/v1/chat/completions"))
        .json(&request_payload)
        .send()?
        .json()?;

    let raw_response = &response.choices[0].message.content;
    println!("{}", raw_response);
    Ok(())
}
