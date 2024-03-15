use crate::game_state::GameState;
use actix::Addr;
use actix_web::{get, web, HttpResponse, Responder};

// GET /status
#[get("")]
async fn get_status(game_state_addr: web::Data<Addr<GameState>>) -> impl Responder {
  let tick = game_state_addr
    .send(crate::game_state::messages::get_tick::GetTick)
    .await;
  HttpResponse::Ok().body(tick.unwrap().to_string())
}
