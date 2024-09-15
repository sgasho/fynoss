use std::error::Error;
use crate::internals::ai::models::dto::AIInquiryResponse;
use crate::internals::ai::models::dto::Status::StatusCode;
use crate::pkg::ai::client::client::GenAIClient;

pub trait AIService {
    async fn inquiry(&self, content: String) -> Result<AIInquiryResponse, Box<dyn Error>>;
}

#[derive(Clone)]
pub struct OpenAIService<C: GenAIClient> {
    client: C,
}

impl<C: GenAIClient> OpenAIService<C> {
    pub fn new(client: C) -> Self {
        Self {
            client
        }
    }
}

impl<C: GenAIClient> AIService for OpenAIService<C> {
    async fn inquiry(&self, content: String) -> Result<AIInquiryResponse, Box<dyn Error>> {
        let gpt_res = self.client.inquire(content).await?;
        Ok(AIInquiryResponse {
            status: StatusCode(gpt_res.status),
            text: gpt_res.text,
        })
    }
}