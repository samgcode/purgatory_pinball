use macroquad::prelude::*;

use crate::Assets;

use super::TriggerZone;

const WIDTH: f32 = 48.0;
const HEIGHT: f32 = WIDTH / 4.0;
const DEFAULT_STRENGTH: f32 = 500.0;

#[allow(unused)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

use Direction::*;

pub struct Spring {
  pub pos: Vec2,
  pub direction: Direction,
  pub normal: Vec2,
  pub strength: f32,
  pub collider: TriggerZone,
  texture: Texture2D,
  animation_frame: usize,
  animation_length: usize,
}

impl Spring {
  pub fn new(pos: Vec2, direction: Direction, strength: Option<f32>, assets: &Assets) -> Self {
    if let Direction::Right | Direction::Left = direction {}

    let collider = match direction {
      Up | Down => TriggerZone::new(
        pos - Vec2::new(WIDTH / 2.0, HEIGHT / 2.0),
        Vec2::new(WIDTH, HEIGHT),
      ),
      Right | Left => TriggerZone::new(
        pos - Vec2::new(HEIGHT / 2.0, WIDTH / 2.0),
        Vec2::new(HEIGHT, WIDTH),
      ),
    };

    let normal = match direction {
      Up => Vec2::new(0.0, -1.0),
      Down => Vec2::new(0.0, 1.0),
      Left => Vec2::new(-1.0, 0.0),
      Right => Vec2::new(1.0, 0.0),
    };

    Self {
      pos,
      direction,
      normal,
      strength: strength.unwrap_or(DEFAULT_STRENGTH),
      collider,
      texture: assets.spring.create_texture(),
      animation_frame: 0,
      animation_length: assets.spring.animation_length,
    }
  }

  pub fn hit(&mut self) {
    self.animation_frame = 1;
  }

  pub fn redraw(&mut self) {
    if self.animation_frame > 0 {
      self.animation_frame += 1;

      if self.animation_frame >= self.animation_length {
        self.animation_frame = 0;
      }
    }
  }

  pub fn draw(&self, assets: &Assets) {
    assets.spring.draw(
      &self.texture,
      self.pos + (assets.spring.scale_factor * WIDTH * 0.5 - HEIGHT * 0.5) * self.normal,
      WIDTH,
      self.animation_frame,
    );
  }
}
