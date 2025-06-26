use std::env;

use anyhow::{Result, anyhow};
use reqwest::blocking as reqwest;
use serde_json::json;

use crate::schemas::{GeminiResponse, OpenAIResponse, UserMessage};

pub fn handle_gemini_request(client: &reqwest::Client, messages: &[UserMessage]) -> Result<()> {
    let msgs = messages
        .iter()
        .map(|msg| &*msg.content)
        .intersperse("\n")
        .collect::<String>();

    let request_payload = json!({ "contents": [{"parts": [{ "text": msgs }] }] });

    let api_key = env::var("GEMINI_API_KEY")?;

    let response: GeminiResponse= client
        .post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",api_key))
        .json(&request_payload)
        .send()?
        .json()?;

    let raw_response = &response.candidates[0].content.parts[0].text;
    println!("{}", raw_response);
    Ok(())
}

pub fn handle_openai_request(
    client: &reqwest::Client,
    api: &str,
    messages: &[UserMessage],
) -> Result<()> {
    let request_payload = json!({
        "model": "default",
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

    let response = client
        .post(format!("{api}/chat/completions"))
        .json(&request_payload)
        .send()?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "OpenAI API request failed: {} - {}",
            response.status(),
            response.text()?
        ));
    }

    let response = &response.json::<OpenAIResponse>()?.choices[0]
        .message
        .content;

    println!("{response}");
    Ok(())
}
