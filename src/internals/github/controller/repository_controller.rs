use std::error::Error;
use actix_web::{web};
use crate::internals::github::models::dto::{Issue, ReadmeResponse, Repositories, SearchRepositoriesRequest};
use crate::internals::github::models::entity::SearchIssuesRequest;
use crate::internals::github::usecases::repository_usecase::RepositoryUseCase;

pub trait RepositoryController {
    async fn fetch_repositories(&self, req: web::Json<SearchRepositoriesRequest>) -> Result<Repositories, Box<dyn Error>>;
    async fn fetch_top_readme(&self, owner_name: &str, repository_name: &str) -> Result<ReadmeResponse, Box<dyn Error>>;
    async fn fetch_issues(&self, owner_name: &str, repository_name: &str, req: SearchIssuesRequest) -> Result<Vec<Issue>, Box<dyn Error>>;
}

#[derive(Clone)]
pub struct GithubRepositoryController<U: RepositoryUseCase> {
    usecase: U,
}

impl<U: RepositoryUseCase> GithubRepositoryController<U> {
    pub fn new(usecase: U) -> Self {
        Self {
            usecase
        }
    }
}

impl<U: RepositoryUseCase> RepositoryController for GithubRepositoryController<U> {
    async fn fetch_repositories(&self, req: web::Json<SearchRepositoriesRequest>) -> Result<Repositories, Box<dyn Error>> {
        let search_req = SearchRepositoriesRequest {
            min_stars: req.min_stars,
            max_stars: req.max_stars,
            last_pushed: req.last_pushed.clone(),
            language: req.language.clone(),
            good_first_issues_count: req.good_first_issues_count,
            help_wanted_count: req.help_wanted_count,
        };
        self.usecase.fetch_repositories(search_req).await
    }

    async fn fetch_top_readme(&self, owner_name: &str, repository_name: &str) -> Result<ReadmeResponse, Box<dyn Error>> {
        self.usecase.fetch_top_readme(owner_name, repository_name).await
    }

    async fn fetch_issues(&self, owner_name: &str, repository_name: &str, req: SearchIssuesRequest) -> Result<Vec<Issue>, Box<dyn Error>> {
        self.usecase.fetch_issues(owner_name, repository_name, req).await
    }
}