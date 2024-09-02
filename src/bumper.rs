use macroquad::prelude::*;

use crate::Assets;

pub struct Bumper {
  pub pos: Vec2,
  texture: Texture2D,
  pub strength: f32,
  pub radius: f32,
  pub animation_frame: usize,
}

impl Bumper {
  pub fn new(pos: Vec2, strength: f32, assets: &Assets) -> Self {
    let texture = Texture2D::empty();
    texture.set_filter(FilterMode::Nearest);

    Self {
      pos,
      texture: assets.create_bumper_texture(),
      strength,
      radius: 40.0,
      animation_frame: 0,
    }
  }

  pub fn hit(&mut self) {
    self.animation_frame = 1;
  }

  pub fn draw(&mut self, assets: &Assets) {
    if self.animation_frame > 0 {
      self.animation_frame += 1;
      if self.animation_frame >= assets.bumper_blue.animation_length {
        self.animation_frame = 0;
      }
    }

    if self.animation_frame > assets.bumper_blue.animation_length {
      self.animation_frame = 0;
    }

    assets
      .bumper_blue
      .draw(&self.texture, self.pos, self.radius, self.animation_frame);
  }
}
