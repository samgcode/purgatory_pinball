use macroquad::prelude::*;

use ball::*;
use bumper::*;

mod ball;
mod bumper;

const GRAVITY: f32 = 200.0;

#[macroquad::main("purgatory pinball")]
async fn main() {
  let gravity = Vec2::new(0.0, GRAVITY);

  let mut ball = Ball::new(Vec2::new(400.0, 450.0), Vec2::new(0.0, 0.0));
  let bumpers = vec![
    Bumper::new(
      Vec2::new(400.0, 500.0),
      Color::new(1.0, 1.0, 0.0, 1.0),
      300.0,
    ),
    Bumper::new(
      Vec2::new(400.0, 300.0),
      Color::new(1.0, 1.0, 0.0, 1.0),
      300.0,
    ),
  ];

  loop {
    let dt = get_frame_time();
    clear_background(BLACK);

    draw_text("pumber pinbatory", 100.0, 100.0, 30.0, WHITE);

    ball.update(gravity, dt);

    for bumper in bumpers.iter() {
      ball.update_collision(&bumper);
    }

    ball.draw();
    for bumper in bumpers.iter() {
      bumper.draw();
    }

    next_frame().await
  }
}
