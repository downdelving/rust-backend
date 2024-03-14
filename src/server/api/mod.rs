use actix_web::{web::scope, Scope};

pub mod account;

pub fn api() -> Scope {
  scope("/api").service(account::account())
}
