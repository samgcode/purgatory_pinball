use macroquad::prelude::*;

use crate::Assets;

pub enum Type {
  Blue,
  White,
  Pink,
  Orange,
}

pub struct Bumper {
  pub pos: Vec2,
  texture: Texture2D,
  effect_type: Type,
  pub strength: f32,
  pub radius: f32,
  pub animation_frame: usize,
}

impl Bumper {
  pub fn new(pos: Vec2, strength: f32, assets: &Assets, effect_type: Type) -> Self {
    let texture = Texture2D::empty();
    texture.set_filter(FilterMode::Nearest);

    Self {
      pos,
      texture: assets.create_bumper_texture(),
      strength,
      radius: 30.0,
      animation_frame: 0,
      effect_type,
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

    match self.effect_type {
      Type::Blue => {
        assets
          .bumper_blue
          .draw(&self.texture, self.pos, self.radius, self.animation_frame)
      }
      Type::White => {
        assets
          .bumper_white
          .draw(&self.texture, self.pos, self.radius, self.animation_frame)
      }
      Type::Pink => {
        assets
          .bumper_pink
          .draw(&self.texture, self.pos, self.radius, self.animation_frame)
      }
      Type::Orange => {
        assets
          .bumper_orange
          .draw(&self.texture, self.pos, self.radius, self.animation_frame)
      }
    }
  }
}
