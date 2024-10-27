use std::error::Error;
use crate::internals::github::models::dto::{Issue, ReadmeResponse, Repositories, SearchRepositoriesRequest};
use crate::internals::github::models::entity::SearchIssuesRequest;
use crate::internals::github::services::repository_service::RepositoryService;

pub trait RepositoryUseCase {
    async fn fetch_repositories(&self, req: SearchRepositoriesRequest) -> Result<Repositories, Box<dyn Error>>;
    async fn fetch_top_readme(&self, owner_name: &str, repository_name: &str) -> Result<ReadmeResponse, Box<dyn Error>>;
    async fn fetch_issues(&self, owner_name: &str, repository_name: &str, req: SearchIssuesRequest) -> Result<Vec<Issue>, Box<dyn Error>>;
}

#[derive(Clone)]
pub struct GithubRepositoryUseCase<S: RepositoryService> {
    service: S
}

impl<S: RepositoryService> GithubRepositoryUseCase<S> {
    pub fn new(service: S) -> Self {
        Self {
            service
        }
    }
}

impl<S: RepositoryService> RepositoryUseCase for GithubRepositoryUseCase<S> {
    async fn fetch_repositories(&self, req: SearchRepositoriesRequest) -> Result<Repositories, Box<dyn Error>> {
        self.service.fetch_repositories(req).await
    }

    async fn fetch_top_readme(&self, owner_name: &str, repository_name: &str) -> Result<ReadmeResponse, Box<dyn Error>> {
        self.service.fetch_top_readme(owner_name, repository_name).await
    }

    async fn fetch_issues(&self, owner_name: &str, repository_name: &str, req: SearchIssuesRequest) -> Result<Vec<Issue>, Box<dyn Error>> {
        self.service.fetch_issues(owner_name, repository_name, req).await
    }
}

