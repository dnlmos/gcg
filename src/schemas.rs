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
