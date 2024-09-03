use crate::engine::*;

const GRAVITY: Vec2 = Vec2::new(0.0, 400.0);

pub struct Game {
  assets: Assets,

  ball: Ball,
  flipper: (Flipper, Flipper),
  bumpers: Vec<Bumper>,
  lines: Vec<(Vec2, Vec2)>,

  score: i32,
  lives: i32,
  respawning: bool,
}

impl Game {
  pub async fn init() -> Self {
    let assets = load_assets().await;

    let ball = Ball::new(Vec2::new(300.0, 300.0), Vec2::new(0.0, 0.0));
    let flipper = (
      Flipper::new(Vec2::new(200.0, 450.0), 100.0, false),
      Flipper::new(Vec2::new(500.0, 450.0), 100.0, true),
    );
    let bumpers = vec![
      Bumper::new(Vec2::new(730.0, 650.0), 700.0, &assets, BumperType::Pink),
      Bumper::new(Vec2::new(535.0, 300.0), 600.0, &assets, BumperType::Pink),
      Bumper::new(Vec2::new(400.0, 250.0), 150.0, &assets, BumperType::White),
      Bumper::new(Vec2::new(150.0, 250.0), 150.0, &assets, BumperType::White),
      Bumper::new(Vec2::new(300.0, 150.0), 500.0, &assets, BumperType::Blue),
      Bumper::new(Vec2::new(250.0, 375.0), 500.0, &assets, BumperType::Blue),
      Bumper::new(Vec2::new(630.0, 150.0), 500.0, &assets, BumperType::Blue),
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

    return Self {
      assets,
      ball,
      flipper,
      bumpers,
      lines,
      score: 0,
      lives: 3,
      respawning: false,
    };
  }

  pub fn update(&mut self, _dt: f32) {
    self.flipper.0.update();
    self.flipper.1.update();
  }

  pub fn fixed_update(&mut self, fixed_dt: f32) {
    self.ball.fixed_update(GRAVITY, fixed_dt);
    self.flipper.0.fixed_update(fixed_dt);
    self.flipper.1.fixed_update(fixed_dt);

    for bumper in self.bumpers.iter_mut() {
      bumper.fixed_update();
    }

    physics::ball_to_flipper(&mut self.ball, &self.flipper.0);
    physics::ball_to_flipper(&mut self.ball, &self.flipper.1);

    for bumper in self.bumpers.iter_mut() {
      self.score += physics::ball_to_bumper(&mut self.ball, bumper);
    }

    for line in self.lines.iter() {
      physics::ball_to_line(&mut self.ball, *line);
    }

    if self.ball.pos.y > 490.0 && !self.respawning {
      self.lives -= 1;
      self.respawning = true;
      if self.lives < 0 {
        println!("score: {}", self.score);

        self.lives = 3;
        self.score = 0;
      }
    }

    if self.respawning && self.ball.pos.y < 490.0 {
      self.respawning = false;
    }
  }

  pub fn draw(&mut self) {
    // draw_text("pumball pingatory", 100.0, 100.0, 30.0, WHITE);

    draw_text(
      format!("score: {}", self.score).as_str(),
      200.0,
      20.0,
      30.0,
      WHITE,
    );
    draw_text(
      format!("lives: {}", self.lives).as_str(),
      500.0,
      20.0,
      30.0,
      WHITE,
    );

    self.ball.draw();
    self.flipper.0.draw();
    self.flipper.1.draw();

    for bumper in self.bumpers.iter_mut() {
      bumper.draw(&self.assets);
    }

    for line in self.lines.iter() {
      draw_line_vec(line.0, line.1, 3.0, WHITE);
    }
  }
}
