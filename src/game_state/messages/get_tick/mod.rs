use crate::game_state::GameState;
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "u32")]
pub struct GetTick;

impl Handler<GetTick> for GameState {
  type Result = u32;

  fn handle(&mut self, _msg: GetTick, _ctx: &mut Context<Self>) -> Self::Result {
    self.count
  }
}
