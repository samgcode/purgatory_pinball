use macroquad::prelude::*;

const MIN_ANGLE: f32 = 30.0;
const MAX_ANGLE: f32 = -20.0;
const MIN_ANGLE_REV: f32 = 150.0;
const MAX_ANGLE_REV: f32 = 200.0;

const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;

const SPEED: f32 = 14.0;
const RETRACT_SPEED: f32 = 8.0;

pub struct Flipper {
  pub anchor: Vec2,
  pub length: f32,
  pub tip: Vec2,
  pub angle: f32,
  position: f32,
  pub vel: f32,
  reversed: bool,
  key_pressed: bool,
}

impl Flipper {
  pub fn new(anchor: Vec2, length: f32, reversed: bool) -> Self {
    Self {
      anchor,
      length,
      tip: anchor + Vec2::new(length, 0.0),
      angle: 0.0,
      position: 0.0,
      vel: 0.0,
      reversed,
      key_pressed: false,
    }
  }

  pub fn update(&mut self) {
    self.key_pressed = is_key_down(KeyCode::A);
  }

  pub fn fixed_update(&mut self, dt: f32) {
    if self.key_pressed {
      if self.position < 1.0 {
        self.position += SPEED * dt;
        self.vel = -lerp(SPEED * dt, MIN_ANGLE_REV, MAX_ANGLE_REV) * 2.0;
      } else {
        self.position = 1.0;
        self.vel = 0.0;
      }
    } else {
      if self.position > 0.0 {
        self.position -= RETRACT_SPEED * dt;
        self.vel = -lerp(RETRACT_SPEED * dt, MIN_ANGLE_REV, MAX_ANGLE_REV) * 2.0;
      } else {
        self.position = 0.0;
        self.vel = 0.0;
      }
    }

    self.angle = if self.reversed {
      lerp(self.position, MIN_ANGLE_REV, MAX_ANGLE_REV)
    } else {
      lerp(self.position, MIN_ANGLE, MAX_ANGLE)
    };

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

pub fn lerp(t: f32, a: f32, b: f32) -> f32 {
  return a + (b - a) * t;
}
