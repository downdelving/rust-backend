use actix_web::{delete, get, patch, put, web, web::scope, HttpResponse, Responder, Scope};

pub mod post;

// GET /account/{id}
#[get("/{id}")]
async fn get_account_id(path: web::Path<String>) -> impl Responder {
  let id = path.into_inner();
  HttpResponse::Ok().body(format!("/api/account/{}", id))
}

// PUT /account/{id}
#[put("/{id}")]
async fn put_account_id(path: web::Path<String>) -> impl Responder {
  let id = path.into_inner();
  HttpResponse::Ok().body(format!("PUT /api/account/{}", id))
}

// PATCH /account/{id}
#[patch("/{id}")]
async fn patch_account_id(path: web::Path<String>) -> impl Responder {
  let id = path.into_inner();
  HttpResponse::Ok().body(format!("PATCH /api/account/{}", id))
}

// DELETE /account/{id}
#[delete("/{id}")]
async fn delete_account_id(path: web::Path<String>) -> impl Responder {
  let id = path.into_inner();
  HttpResponse::Ok().body(format!("DELETE /api/account/{}", id))
}

pub fn account() -> Scope {
  scope("/account")
    .service(post::post_account)
    .service(get_account_id)
    .service(put_account_id)
    .service(patch_account_id)
    .service(delete_account_id)
}
