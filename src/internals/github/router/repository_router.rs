use actix_web::{web, HttpResponse, Responder, Scope};
use log::info;
use crate::internals::github::controller::repository_controller::{RepositoryController};
use crate::internals::github::models::dto::SearchRepositoriesRequest;
use crate::internals::github::models::entity::SearchIssuesRequestQueries;

pub trait RepositoryRouter {
    fn repository_scope(&self) -> Scope;
}

#[derive(Clone)]
pub struct GithubRepositoryRouter<C: RepositoryController> {
    controller: C,
}

impl<C: RepositoryController> GithubRepositoryRouter<C> {
    pub fn new(controller: C) -> Self {
        Self { controller }
    }

    async fn search_repositories_handler(router: web::Data<GithubRepositoryRouter<C>>, req: web::Json<SearchRepositoriesRequest>) -> impl Responder {
        match router.controller.fetch_repositories(req).await {
            Ok(repositories) => HttpResponse::Ok().json(repositories),
            Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        }
    }

    async fn search_repository_top_readme_handler(router: web::Data<GithubRepositoryRouter<C>>, path: web::Path<(String, String)>) -> impl Responder {
        let (owner_name, repo_name) = path.into_inner();
        match router.controller.fetch_top_readme(&owner_name, &repo_name).await {
            Ok(top_readme) => HttpResponse::Ok().json(top_readme),
            Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        }
    }

    async fn search_repository_issues_handler(router: web::Data<GithubRepositoryRouter<C>>, path: web::Path<(String, String)>, query: web::Query<SearchIssuesRequestQueries>) -> impl Responder {
        let (owner_name, repo_name) = path.into_inner();
        let query_params = query.into_inner();
        match router.controller.fetch_issues(&owner_name, &repo_name, query_params.into()).await {
            Ok(issues) => HttpResponse::Ok().json(issues),
            Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        }
    }
}

impl<C: RepositoryController + 'static> GithubRepositoryRouter<C> {
    pub fn repository_scope(&self) -> Scope {
        web::scope("/github")
            .service(
                web::scope("/repositories")
                    .route("/search-list", web::post().to(Self::search_repositories_handler))
                    .route("/{owner_name}/{repo_name}/top-readme", web::get().to(Self::search_repository_top_readme_handler))
                    .route("/{owner_name}/{repo_name}/issues", web::get().to(Self::search_repository_issues_handler))
            )
    }
}

