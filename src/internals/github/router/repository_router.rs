use actix_web::{web, HttpResponse, Responder, Scope};
use crate::internals::github::controller::repository_controller::{RepositoryController};
use crate::internals::github::models::dto::SearchRepositoriesRequest;

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
            Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
        }
    }
}

impl<C: RepositoryController + 'static> RepositoryRouter for GithubRepositoryRouter<C> {
    fn repository_scope(&self) -> Scope {
        web::scope("/github")
            .route("/search", web::post().to(
                GithubRepositoryRouter::<C>::
                search_repositories_handler))
    }
}
