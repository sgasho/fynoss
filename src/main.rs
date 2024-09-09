mod pkg;
mod internals;
mod cli;

use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use sqlx::MySqlPool;
use crate::internals::github::controller::repository_controller::GithubRepositoryController;
use crate::internals::github::repositories::repository_repository::{GithubRepositoryRepository};
use crate::internals::github::router::repository_router::{GithubRepositoryRouter, RepositoryRouter};
use crate::internals::github::services::repository_service::GithubRepositoryService;
use crate::internals::github::usecases::repository_usecase::GithubRepositoryUseCase;
use crate::pkg::github::client::client::GithubClient;
use crate::pkg::github::repositories::GithubRepositoryClient;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is not set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = MySqlPool::connect(&database_url).await.unwrap();
    let github_client = GithubClient::new(github_token);
    let github_repository_client = GithubRepositoryClient::new(github_client);
    let github_repository_repository = GithubRepositoryRepository::new(pool);
    let github_repository_service =
        GithubRepositoryService::new(github_repository_client, github_repository_repository);
    let github_repository_usecase = GithubRepositoryUseCase::new(github_repository_service);
    let github_repository_controller = GithubRepositoryController::new(github_repository_usecase);
    let github_repository_router = GithubRepositoryRouter::new(github_repository_controller);
    let github_repository_router = Arc::new(github_repository_router);

    HttpServer::new(move || {
        let router_clone = github_repository_router.clone();
        App::new()
            .app_data(web::Data::from(router_clone))
            .service(github_repository_router.repository_scope())
    })
        .bind(("0.0.0.0", 8080))
        .unwrap()
        .run()
        .await.unwrap();
}
