use actix::{Actor, AsyncContext, Context};
use std::time::Duration;

pub mod messages;

// Define a simple actor for managing game state
pub struct GameState {
  pub count: u32,
}

impl GameState {
  pub fn new() -> Self {
    GameState { count: 0 }
  }
}

impl Default for GameState {
  fn default() -> Self {
    GameState::new()
  }
}

impl Actor for GameState {
  type Context = Context<Self>;

  // Initialize with periodic updates
  fn started(&mut self, ctx: &mut Self::Context) {
    self.periodic_update(ctx)
  }
}

impl GameState {
  fn periodic_update(&self, ctx: &mut Context<Self>) {
    ctx.run_interval(Duration::from_secs(1), |actor, _ctx| {
      log::info!("Periodic update");
      println!("Periodic update: {}", actor.count);
      actor.count += 1;
    });
  }
}
