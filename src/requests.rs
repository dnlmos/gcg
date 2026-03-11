use anyhow::{Context, Result};

use crate::{get_api_key, schemas::InteractionResponse};
use serde_json::{Map, json};
use url::Url;

use crate::{
    Provider,
    schemas::{OpenAIResponse, UserMessage},
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

    let request_payload = json!({"model":"gemini-2.5-flash", "input": msgs});
    let api_key = get_api_key("gcg", "gemini_key")?;

    let raw_response: InteractionResponse = serde_json::from_str(
        &client
            .post(provider.api_url)
            .header("x-goog-api-key", api_key)
            .json(&request_payload)
            .send()?
            .text()?,
    )?;

    let (_, text) = raw_response.outputs;

    print_response(&text.text);
    Ok(())
}

pub fn handle_openai_request(
    client: &Client,
    messages: &[UserMessage],
    provider: Provider,
) -> Result<()> {
    let model_name = provider.model;

    let mut request_payload = Map::new();

    request_payload.insert("model".to_string(), json!(model_name));
    request_payload.insert("messages".to_string(), json!(messages));

    let api_url = format!("{}/v1/chat/completions", provider.api_url);

    // Because OpenAI schema is also supported by Gemini models, determine which api_key to use
    // based on api url
    let response: OpenAIResponse = match resolve_key(&provider.api_url) {
        Ok(key_name) => {
            let api_key = get_api_key("gcg", &key_name)?;
            client
                .post(api_url)
                .header("Content-type", "application/json")
                .header("Authorization", format!("Bearer {api_key}"))
                .json(&request_payload)
                .send()?
                .json()?
        }
        // Send without headers (e.g. to ollama)
        Err(_) => client
            .post(&provider.api_url)
            .json(&request_payload)
            .send()?
            .json()?,
    };

    let raw_response = &response.choices[0].message.content;
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

fn resolve_key(url: &str) -> Result<String> {
    let binding = Url::parse(url).context("Failed to parse URL")?;
    let host = binding.host_str().context("Invalid provider URL")?;

    let key_name = match host {
        h if h.ends_with("openai.com") => "openai",
        h if h.ends_with("googleapis.com") => "gemini_key",
        _ => return Err(anyhow::anyhow!("Unknown provider host: {}", host)),
    };

    Ok(key_name.to_string())
}
