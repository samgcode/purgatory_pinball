use macroquad::prelude::*;

const MIN_ANGLE: f32 = 30.0;
const MAX_ANGLE: f32 = -20.0;
const MIN_ANGLE_REV: f32 = 150.0;
const MAX_ANGLE_REV: f32 = 200.0;

const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;

const SPEED: f32 = -300.0;
const RETRACT_SPEED: f32 = 200.0;

pub struct Flipper {
  pub anchor: Vec2,
  pub length: f32,
  pub tip: Vec2,
  pub angle: f32,
  pub vel: f32,
  reversed: bool,
}

impl Flipper {
  pub fn new(anchor: Vec2, length: f32, reversed: bool) -> Self {
    Self {
      anchor,
      length,
      tip: anchor + Vec2::new(length, 0.0),
      angle: if reversed { MIN_ANGLE_REV } else { MIN_ANGLE },
      vel: 0.0,
      reversed,
    }
  }

  pub fn update(&mut self) {
    if is_key_pressed(KeyCode::A) {
      if self.reversed {
        if self.angle < MAX_ANGLE_REV {
          self.vel = SPEED;
        }
      } else {
        if self.angle > MAX_ANGLE {
          self.vel = SPEED;
        }
      }
    }
  }

  pub fn physics_update(&mut self, dt: f32) {
    if self.reversed {
      if self.vel == SPEED {
        if self.angle > MAX_ANGLE_REV {
          self.vel = 0.0;
        }
      } else {
        if self.angle > MIN_ANGLE_REV {
          self.vel = RETRACT_SPEED;
        } else {
          self.vel = 0.0;
        }
      }
    } else {
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
    }

    if self.reversed {
      self.angle -= self.vel * dt;
    } else {
      self.angle += self.vel * dt;
    }

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
