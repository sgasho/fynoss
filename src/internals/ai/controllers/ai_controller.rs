use std::error::Error;
use crate::internals::ai::models::dto::AIInquiryResponse;
use crate::internals::ai::usecases::ai_usecase::AIUseCase;

pub trait AIController {
    async fn ask_how_to_contribute(&self, owner_name: &str, repository_name: &str) -> Result<AIInquiryResponse, Box<dyn Error>>;
}

#[derive(Clone)]
pub struct OpenAIController<U: AIUseCase> {
    usecase: U,
}

impl<U: AIUseCase> OpenAIController<U> {
    pub fn new(usecase: U) -> Self {
        Self {
            usecase
        }
    }
}

impl<U: AIUseCase> AIController for OpenAIController<U> {
    async fn ask_how_to_contribute(&self, owner_name: &str, repository_name: &str) -> Result<AIInquiryResponse, Box<dyn Error>> {
        self.usecase.ask_how_to_contribute(owner_name, repository_name).await
    }
}