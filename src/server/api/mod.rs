use actix_web::{web::scope, Scope};

pub mod account;
pub mod status;

pub fn api() -> Scope {
  scope("/api").service(account::account()).service(status::status())
}
