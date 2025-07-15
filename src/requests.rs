use anyhow::Result;
use serde_json::json;

use crate::{
    Provider,
    schemas::{GeminiResponse, OllamaResponse, OpenAIResponse, UserMessage},
};
use colored::*;

use reqwest::blocking::Client;
pub fn handle_gemini_request(
    client: &Client,
    messages: &[UserMessage],
    provider: Provider,
) -> Result<()> {
    let msgs = messages
        .iter()
        .map(|msg| msg.content.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    let request_payload = json!({ "contents": [ { "parts": [ { "text": msgs } ] } ], });

    let api_endpoint = if let Some(key) = &provider.api_key {
        format!("{}{}", provider.api_url, key)
    } else {
        provider.api_url.clone()
    };

    let response: GeminiResponse = client
        .post(api_endpoint)
        .json(&request_payload)
        .send()?
        .json()?;

    let raw_response = &response.candidates[0].content.parts[0].text;
    print_response(raw_response);
    Ok(())
}

pub fn handle_openai_request(
    client: &Client,
    messages: &[UserMessage],
    provider: Provider,
) -> Result<()> {
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

    // println!(
    //     "Sending JSON:\n{}",
    //     serde_json::to_string_pretty(&request_payload).unwrap()
    // );

    let response: OpenAIResponse = client
        .post(provider.api_url)
        .json(&request_payload)
        .send()?
        .json()?;

    let raw_response = &response.choices[0].message.content;
    print_response(raw_response);
    Ok(())
}

pub fn handle_ollama_request(
    client: &Client,
    messages: &[UserMessage],
    provider: Provider,
) -> Result<()> {
    let request_payload = json!({
        "model": provider.model,
        "prompt": format!("{}\n{}", messages[0].content, messages[1].content),
        "stream": false,
    });

    println!(
        "Sending JSON:\n{}",
        serde_json::to_string_pretty(&request_payload).unwrap()
    );

    let response: OllamaResponse = client
        .post(provider.api_url)
        .json(&request_payload)
        .send()?
        .json()?;

    let raw_response = &response.response;
    print_response(raw_response);
    Ok(())
}

fn print_response(raw_response: &str) {
    println!(
        "{}\n{}",
        "Generated Commit Message:".bright_green().bold(),
        raw_response
    );
}
