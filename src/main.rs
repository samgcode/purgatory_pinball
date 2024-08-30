use macroquad::prelude::*;

use ball::*;
use bumper::*;
use flipper::*;

mod ball;
mod bumper;
mod drawing;
mod flipper;
mod physics;

const GRAVITY: f32 = 200.0;

#[macroquad::main("pumball pingatory")]
async fn main() {
  let gravity = Vec2::new(0.0, GRAVITY);

  let mut ball = Ball::new(Vec2::new(300.0, 300.0), Vec2::new(0.0, 0.0));
  let mut flipper = Flipper::new(Vec2::new(200.0, 500.0), 100.0);
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
    let mut dt = 0.016;
    clear_background(BLACK);

    if is_key_down(KeyCode::Space) {
      dt = 0.0;
      if is_key_pressed(KeyCode::Right) {
        dt = 0.016;
      }
    }

    draw_text("pumber pinbatory", 100.0, 100.0, 30.0, WHITE);

    ball.update(gravity, dt);
    flipper.update(dt);

    ball.draw();
    flipper.draw();
    for bumper in bumpers.iter() {
      bumper.draw();
    }

    physics::update_collision(&mut ball, &mut flipper);
    for bumper in bumpers.iter() {
      ball.update_collision(&bumper);
    }

    next_frame().await
  }
}
