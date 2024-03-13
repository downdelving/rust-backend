use actix_web::{get, web::scope, HttpResponse, Responder, Scope};

// "/api".
#[get("")]
async fn empty() -> impl Responder {
  HttpResponse::Ok().body("/api")
}

// "/api/".
#[get("/")]
async fn index() -> impl Responder {
  HttpResponse::Ok().body("/api/")
}

pub fn api() -> Scope {
  scope("/api").service(empty).service(index)
}

pub mod prelude {
  pub use super::api;
}
