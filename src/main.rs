use drawing::draw_line_vec;
use macroquad::prelude::*;

use assets::*;
use ball::*;
use bumper::*;
use flipper::*;

mod assets;
mod ball;
mod bumper;
mod drawing;
mod flipper;
mod physics;

const GRAVITY: f32 = 400.0;

#[macroquad::main("purgatory pinball")]
async fn main() {
  let gravity = Vec2::new(0.0, GRAVITY);

  let assets = load_assets().await;

  let mut ball = Ball::new(Vec2::new(300.0, 300.0), Vec2::new(0.0, 0.0));
  let mut flipper_1 = Flipper::new(Vec2::new(200.0, 450.0), 100.0, false);
  let mut flipper_2 = Flipper::new(Vec2::new(500.0, 450.0), 100.0, true);
  let mut bumpers = vec![
    Bumper::new(Vec2::new(730.0, 650.0), 700.0, &assets, Type::Pink),
    Bumper::new(Vec2::new(535.0, 300.0), 600.0, &assets, Type::Pink),
    Bumper::new(Vec2::new(400.0, 250.0), 150.0, &assets, Type::White),
    Bumper::new(Vec2::new(150.0, 250.0), 150.0, &assets, Type::White),
    Bumper::new(Vec2::new(300.0, 150.0), 500.0, &assets, Type::Blue),
    Bumper::new(Vec2::new(250.0, 375.0), 500.0, &assets, Type::Blue),
    Bumper::new(Vec2::new(630.0, 150.0), 500.0, &assets, Type::Blue),
  ];

  let lines = vec![
    (Vec2::new(50.0, 50.0), Vec2::new(750.0, 50.0)), // top wall
    (Vec2::new(50.0, 50.0), Vec2::new(50.0, 520.0)), // left wall
    (Vec2::new(50.0, 520.0), Vec2::new(700.0, 600.0)), // bottom wall
    (Vec2::new(700.0, 50.0), Vec2::new(750.0, 100.0)), // the booper
    //
    (Vec2::new(750.0, 50.0), Vec2::new(750.0, 600.0)), // right wall
    (Vec2::new(675.0, 150.0), Vec2::new(675.0, 550.0)), // channel wall
    //
    (Vec2::new(50.0, 300.0), Vec2::new(200.0, 450.0)), // left ramp
    (Vec2::new(675.0, 300.0), Vec2::new(500.0, 450.0)), // right ramp
  ];

  let mut dt = 0.0;

  let mut score = 0;

  loop {
    dt += get_frame_time();

    if dt >= 1.0 / 61.0 {
      clear_background(BLACK);

      if dt > 0.0 {
        ball.physics_update(gravity, dt);
        flipper_1.physics_update(dt);
        flipper_2.physics_update(dt);

        for bumper in bumpers.iter_mut() {
          bumper.physics_update();
        }

        physics::ball_to_flipper(&mut ball, &flipper_1);
        physics::ball_to_flipper(&mut ball, &flipper_2);

        for bumper in bumpers.iter_mut() {
          score += physics::ball_to_bumper(&mut ball, bumper);
        }

        for line in lines.iter() {
          physics::ball_to_line(&mut ball, *line);
        }
      }
      dt = 0.0;
    }

    flipper_1.update();
    flipper_2.update();

    draw_text("pumball pingatory", 100.0, 100.0, 30.0, WHITE);
    draw_text("[V0.11]", 0.0, 20.0, 30.0, WHITE);
    draw_text(
      format!("score: {}", score).as_str(),
      100.0,
      20.0,
      30.0,
      WHITE,
    );

    ball.draw();
    flipper_1.draw();
    flipper_2.draw();

    for bumper in bumpers.iter_mut() {
      bumper.draw(&assets);
    }

    for line in lines.iter() {
      draw_line_vec(line.0, line.1, 3.0, WHITE);
    }

    next_frame().await
  }
}
