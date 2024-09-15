use actix_web::{web, HttpResponse, Responder, Scope};
use crate::internals::ai::controllers::ai_controller::AIController;

pub trait AIRouter {
    fn ai_scope(&self) -> Scope;
}

#[derive(Clone)]
pub struct OpenAIRouter<C: AIController> {
    controller: C,
}

impl<C: AIController> OpenAIRouter<C> {
    pub fn new(controller: C) -> Self {
        Self { controller }
    }

    async fn ask_how_to_contribute(router: web::Data<OpenAIRouter<C>>, path: web::Path<(String, String)>) -> impl Responder {
        let (owner_name, repo_name) = path.into_inner();
        match router.controller.ask_how_to_contribute(&owner_name, &repo_name).await {
            Ok(inquiry_res) => HttpResponse::Ok().json(inquiry_res),
            Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        }
    }
}

impl<C: AIController + 'static> OpenAIRouter<C> {
    pub fn ai_scope(&self) -> Scope {
        web::scope("/ai")
            .service(
                web::scope("/inquiry")
                    .route("/how-to-contribute/{owner_name}/{repo_name}", web::get().to(Self::ask_how_to_contribute))
            )
    }
}

