use actix_web::{web::scope, Scope};

pub mod get;

pub fn status() -> Scope {
  scope("/status").service(get::get_status)
}
