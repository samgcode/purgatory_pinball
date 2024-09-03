use macroquad::prelude::*;

use assets::*;
use ball::*;
use bumper::*;
use engine::*;
use flipper::*;

mod assets;
mod ball;
mod bumper;
mod drawing;
mod engine;
mod flipper;
mod physics;

#[macroquad::main("purgatory pinball")]
async fn main() {
  let mut game = Game::init().await;

  let mut fixed_dt: f32 = 0.0;

  loop {
    let dt = get_frame_time();
    fixed_dt += dt;

    if fixed_dt >= 1.0 / 61.0 {
      game.physics_update(fixed_dt);

      fixed_dt = 0.0;
    }

    game.update(dt);
    game.draw();

    next_frame().await
  }
}
