use macroquad::prelude::*;

const MIN_ANGLE: f32 = 45.0;
const MAX_ANGLE: f32 = -20.0;
const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;

const SPEED: f32 = -200.0;
const RETRACT_SPEED: f32 = 180.0;

pub struct Flipper {
  pub anchor: Vec2,
  pub length: f32,
  pub tip: Vec2,
  pub angle: f32,
  pub vel: f32,
}

impl Flipper {
  pub fn new(anchor: Vec2, length: f32) -> Self {
    Self {
      anchor,
      length,
      tip: anchor + Vec2::new(length, 0.0),
      angle: MIN_ANGLE,
      vel: 0.0,
    }
  }

  pub fn update(&mut self, dt: f32) {
    if is_key_pressed(KeyCode::A) {
      if self.angle > MAX_ANGLE {
        self.vel = SPEED;
      }
    }

    if self.vel == SPEED {
      if self.angle < MAX_ANGLE {
        self.vel = 0.0;
      }
    } else {
      if self.angle < MIN_ANGLE {
        self.vel = RETRACT_SPEED;
      } else {
        self.vel = 0.0;
      }
    }

    self.angle += self.vel * dt;

    self.tip = self.anchor
      + Vec2 {
        x: (self.angle * DEG_TO_RAD).cos(),
        y: (self.angle * DEG_TO_RAD).sin(),
      } * self.length;
  }

  pub fn draw(&self) {
    draw_line(
      self.anchor.x,
      self.anchor.y,
      self.tip.x,
      self.tip.y,
      5.0,
      Color::new(1.0, 1.0, 1.0, 1.0),
    );
  }
}
