use actix_web::{get, web::scope, HttpResponse, Responder, Scope};

pub mod api;

// "/".
#[get("/")]
async fn root() -> impl Responder {
  HttpResponse::Ok().body("/")
}

pub fn server() -> Scope {
  scope("").service(root).service(api::api())
}

pub mod prelude {
  pub use super::api::prelude::*;
  pub use super::server;
}
