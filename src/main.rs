use actix::Actor;
use actix_web::{web, App, HttpServer};
use downdelving::prelude::*;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
  let game_state_addr = GameState::default().start();
  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(game_state_addr.clone()))
      .service(server())
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
