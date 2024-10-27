mod pkg;
mod internals;
mod cli;

use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use reqwest::Client as ReqwestClient;
use sqlx::MySqlPool;
use crate::internals::ai::controllers::ai_controller::OpenAIController;
use crate::internals::ai::routers::ai_router::OpenAIRouter;
use crate::internals::ai::services::ai_service::OpenAIService;
use crate::internals::ai::usecases::ai_usecase::OpenAIUseCase;
use crate::internals::github::controller::repository_controller::GithubRepositoryController;
use crate::internals::github::repositories::repository_repository::{GithubRepositoryRepository};
use crate::internals::github::router::repository_router::{GithubRepositoryRouter};
use crate::internals::github::services::repository_service::GithubRepositoryService;
use crate::internals::github::usecases::repository_usecase::GithubRepositoryUseCase;
use crate::pkg::ai::client::client::OpenAIClient;
use crate::pkg::github::client::client::GithubClient;
use crate::pkg::github::repositories::GithubRepositoryClient;

#[tokio::main]
async fn main() {
    dotenv().ok();

    unsafe { env::set_var("RUST_LOG", "debug"); }
    env_logger::init();

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is not set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let openai_apikey = env::var("OPENAI_KEY").expect("OPENAI_KEY is not set");

    let reqwest_client = ReqwestClient::new();
    let pool = MySqlPool::connect(&database_url).await.unwrap();
    let github_client = GithubClient::new(github_token, reqwest_client.clone());
    let github_repository_client = GithubRepositoryClient::new(github_client);
    let github_repository_repository = GithubRepositoryRepository::new(pool);
    let github_repository_service =
        GithubRepositoryService::new(github_repository_client, github_repository_repository);
    let github_repository_usecase = GithubRepositoryUseCase::new(github_repository_service.clone());
    let github_repository_controller = GithubRepositoryController::new(github_repository_usecase);
    let github_repository_router = GithubRepositoryRouter::new(github_repository_controller);
    let github_repository_router = Arc::new(github_repository_router);

    let ai_client = OpenAIClient::new(openai_apikey, reqwest_client.clone());
    let ai_service = OpenAIService::new(ai_client);
    let ai_usecase = OpenAIUseCase::new(ai_service, github_repository_service);
    let ai_controller = OpenAIController::new(ai_usecase);
    let ai_router = OpenAIRouter::new(ai_controller);
    let ai_router = Arc::new(ai_router);

    HttpServer::new(move || {
        let github_router_clone = github_repository_router.clone();
        let ai_router_clone = ai_router.clone();
        App::new()
            .app_data(web::Data::from(github_router_clone))
            .app_data(web::Data::from(ai_router_clone))
            .service(github_repository_router.repository_scope())
            .service(ai_router.ai_scope())
            .wrap(Logger::default())
    })
        .bind(("0.0.0.0", 8080))
        .unwrap()
        .run()
        .await.unwrap();
}
