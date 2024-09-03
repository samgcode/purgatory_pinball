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
  animation_frame: usize,
  animation_length: usize,
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
      animation_length: match &effect_type {
        Type::Blue => assets.bumper_blue.animation_length,
        Type::White => assets.bumper_white.animation_length,
        Type::Pink => assets.bumper_pink.animation_length,
        Type::Orange => assets.bumper_orange.animation_length,
      },
      effect_type,
    }
  }

  pub fn hit(&mut self) {
    self.animation_frame = 1;
  }

  pub fn physics_update(&mut self) {
    if self.animation_frame > 0 {
      self.animation_frame += 1;
      if self.animation_frame >= self.animation_length {
        self.animation_frame = 0;
      }
    }

    if self.animation_frame > self.animation_length {
      self.animation_frame = 0;
    }
  }

  pub fn draw(&self, assets: &Assets) {
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
