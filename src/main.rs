use actix_web::{App, HttpServer};
use downdelving::prelude::*;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
  HttpServer::new(|| App::new().service(server()))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
