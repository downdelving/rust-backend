use actix_web::{post, web, HttpResponse, Responder};

#[derive(Deserialize)]
pub struct PostBody {
  pub username: String,
  pub password: String,
  pub email: Option<String>,
}

// POST /account
#[post("")]
async fn post_account(_body: web::Json<PostBody>) -> impl Responder {
  HttpResponse::Ok().body("POST /api/account")
}
