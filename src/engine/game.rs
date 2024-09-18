use crate::engine::*;

pub mod levels;
mod score;

use score::ScoreSystem;
pub use score::ScoreType;

const GRAVITY: Vec2 = Vec2::new(0.0, 400.0);
const START_HEIGHT: f32 = 900.0;
const CAMERA_SPEED: f32 = 0.1;

pub struct Game {
  assets: Assets,

  ball: Ball,
  flipper: (Flipper, Flipper),
  bumpers: Vec<Bumper>,
  springs: Vec<Spring>,
  lose_zone: TriggerZone,
  score_system: ScoreSystem,
  board: Board,

  pub camera_pos: Vec2,
}

impl Game {
  pub async fn init() -> Self {
    let assets = load_assets().await;

    let ball = Ball::new(
      Vec2::new(1375.0, START_HEIGHT),
      Vec2::new(0.0, 0.0),
      &assets,
    );
    let flipper = (
      Flipper::new(Vec2::new(700.0, 990.0), 135.0, false),
      Flipper::new(Vec2::new(1125.0, 990.0), 135.0, true),
    );

    #[rustfmt::skip]
    let bumpers = vec![
      Bumper::new(Vec2::new(750.0, 700.0), None, None, &assets, BumperType::Orange),
      Bumper::new(Vec2::new(560.0, 110.0), None, Some(ScoreType::Bonus(5000)), &assets, BumperType::Pink),
      Bumper::new(Vec2::new(1050.0, 550.0), None, None, &assets, BumperType::Pink),
      Bumper::new(Vec2::new(1150.0, 350.0), None, None, &assets, BumperType::Blue),
      Bumper::new(Vec2::new(900.0, 240.0), None, None, &assets, BumperType::Blue),
      // Bumper::new( Vec2::new(1375.0, 1100.0), Some(1500.0), Some(ScoreType::Points(0)), &assets, BumperType::White),
    ];

    #[rustfmt::skip]
    let springs = vec![
      Spring::new(Vec2::new(1375.0, 1050.0), Direction::Up, Some(1200.0), &assets),
      // Spring::new(Vec2::new(900.0, 700.0), Direction::Left, None),
    ];

    let lose_zone = TriggerZone::new(Vec2::new(500.0, 1100.0), Vec2::new(800.0, 20.0));

    let score_system = ScoreSystem::new();

    let board = Board::new(&assets);

    return Self {
      assets,
      ball,
      flipper,
      bumpers,
      springs,
      lose_zone,
      score_system,
      board,
      camera_pos: Vec2::new(0.0, 0.0),
    };
  }

  pub fn update(&mut self, dt: f32) {
    self.flipper.0.update(dt);
    self.flipper.1.update(dt);

    if is_key_pressed(KeyCode::R) {
      self.reset();
    }
  }

  pub fn redraw(&mut self, scale: f32) {
    for bumper in self.bumpers.iter_mut() {
      bumper.redraw();
    }

    for spring in self.springs.iter_mut() {
      spring.redraw();
    }

    let center = Vec2::new(1920.0 / 2.0, 1080.0 / 2.0);

    let pos = center - self.ball.pos;
    // self.camera_pos = pos * scale * CAMERA_SPEED;
  }

  pub fn fixed_update(&mut self, fixed_dt: f32) {
    self.ball.fixed_update(GRAVITY, fixed_dt);
    self.flipper.0.fixed_update(fixed_dt);
    self.flipper.1.fixed_update(fixed_dt);

    for wall in self.board.walls.iter() {
      physics::ball_to_line(&mut self.ball, *wall);
    }
    physics::ball_to_flipper(&mut self.ball, &self.flipper.0);
    physics::ball_to_flipper(&mut self.ball, &self.flipper.1);

    for bumper in self.bumpers.iter_mut() {
      let score = bumper::ball_to_bumper(&mut self.ball, bumper);
      if let Some(score) = score {
        self.score_system.apply_score(score);
      }
    }

    for spring in self.springs.iter_mut() {
      physics::ball_spring(&mut self.ball, spring);
    }

    physics::ball_trigger_zone(&mut self.ball, &mut self.lose_zone);

    if let CollisionState::Enter = self.lose_zone.state {
      self.score_system.die();
      self.respawn();
      if self.score_system.lives <= 0 {
        self.reset();
      }
    }
  }

  pub fn draw(&mut self) {
    for bumper in self.bumpers.iter() {
      bumper.draw(&self.assets);
    }

    for spring in self.springs.iter() {
      spring.draw(&self.assets);
    }

    self.board.draw();

    self.ball.draw(&self.assets);
    self.flipper.0.draw();
    self.flipper.1.draw();
  }

  pub fn draw_ui(&self, _scale: f32) {
    self.score_system.draw();
  }

  pub fn respawn(&mut self) {
    self.ball = Ball::new(
      Vec2::new(1375.0, START_HEIGHT),
      Vec2::new(0.0, 0.0),
      &self.assets,
    );
  }

  pub fn reset(&mut self) {
    self.ball = Ball::new(
      Vec2::new(1375.0, START_HEIGHT),
      Vec2::new(0.0, 0.0),
      &self.assets,
    );
    self.score_system.reset();
    for bumper in self.bumpers.iter_mut() {
      bumper.reset();
    }
  }
}

// 1:13:03
// 1:07:37
// 1:23:33
// ??
// 1:14:08
// ??
