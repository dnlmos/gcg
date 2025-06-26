use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct ChatResponse {
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
