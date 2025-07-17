use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct OpenAIResponse {
    pub choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessage,
}

#[derive(Deserialize)]
pub struct ChatMessage {
    pub content: String,
}

#[derive(Serialize)] // to be able to convert to JSON
pub struct UserMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
pub struct Candidate {
    pub content: Content,
}

#[derive(Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Deserialize)]
pub struct OllamaResponse {
    pub response: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Model {
    pub name: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Provider {
    pub name: String,
    pub api_url: String,
    pub model: Option<Model>,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub provider: Provider,
    pub prompt_template: String,
}
