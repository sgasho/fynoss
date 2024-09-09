use std::error::Error;
use actix_web::{web};
use crate::internals::github::models::dto::{Repositories, SearchRepositoriesRequest};
use crate::internals::github::usecases::repository_usecase::RepositoryUseCase;

pub trait RepositoryController {
    async fn fetch_repositories(&self, req: web::Json<SearchRepositoriesRequest>) -> Result<Repositories, Box<dyn Error>>;
}

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
}