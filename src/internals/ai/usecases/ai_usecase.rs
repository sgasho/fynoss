use std::error::Error;
use crate::internals::ai::models::dto::AIInquiryResponse;
use crate::internals::ai::models::dto::Status::ReadmeNotFound;
use crate::internals::ai::services::ai_service::AIService;
use crate::internals::github::services::repository_service::RepositoryService;

pub trait AIUseCase {
    async fn ask_how_to_contribute(&self, owner_name: &str, repository_name: &str) -> Result<AIInquiryResponse, Box<dyn Error>>;
}

#[derive(Clone)]
pub struct OpenAIUseCase<AS: AIService, RS: RepositoryService> {
    ai_service: AS,
    repository_service: RS,
}

impl<AS: AIService, RS: RepositoryService> OpenAIUseCase<AS, RS> {
    pub fn new(ais: AS, rs: RS) -> Self {
        Self {
            ai_service: ais,
            repository_service: rs
        }
    }
}

impl<AS: AIService, RS: RepositoryService> AIUseCase for OpenAIUseCase<AS, RS> {
    async fn ask_how_to_contribute(&self, owner_name: &str, repository_name: &str) -> Result<AIInquiryResponse, Box<dyn Error>> {
        let top_readme = self.repository_service.fetch_top_readme(owner_name, repository_name).await?;

        if !top_readme.found {
            return Ok(AIInquiryResponse {
                status: ReadmeNotFound,
                text: "".to_string(),
            })
        }

        let prompt = format!(
            "What do I have to study to contribute to {}/{}?\nReadme of the repository is below\n{:?}",
            owner_name, repository_name, top_readme.content
        );

        self.ai_service.inquiry(prompt).await
    }
}