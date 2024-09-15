use reqwest::Client as ReqwestClient;
use std::error::Error;
use crate::pkg::ai::model::openai::{GPTResponse, Message, OpenAIInquiryRequest};
use crate::pkg::ai::model::openai::Role::User;

pub trait GenAIClient {
    async fn inquire(&self, content: String) -> Result<GPTResponse, Box<dyn Error>>;
}

#[derive(Clone)]
pub struct OpenAIClient {
    api_key: String,
    reqwest_client: ReqwestClient,
}

impl OpenAIClient {
    pub fn new(api_key: String, reqwest_client: ReqwestClient) -> Self {
        Self {
            api_key,
            reqwest_client,
        }
    }
}

impl GenAIClient for OpenAIClient {
    async fn inquire(&self, content: String) -> Result<GPTResponse, Box<dyn Error>> {
        let url = "https://api.openai.com/v1/chat/completions";

        let request_body = OpenAIInquiryRequest {
            model: "gpt-4o-mini-2024-07-18".to_string(),
            messages: vec![Message {
                role: User,
                content,
            }],
        };

        let res = self.reqwest_client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;

        Ok(GPTResponse {
            status: status.as_u16(),
            text
        })
    }
}

