use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GPTResponse {
    pub status: u16,
    pub text: String,
}

#[derive(Serialize)]
pub struct OpenAIInquiryRequest {
    pub model: String,
    pub messages: Vec<Message>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    System,
    User,
    Assistant,
}
