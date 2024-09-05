use macroquad::prelude::*;

use super::Ball;
use crate::Assets;

pub enum BumperType {
  Blue,
  White,
  Pink,
  Orange,
}

pub struct Bumper {
  pub pos: Vec2,
  texture: Texture2D,
  effect_type: BumperType,
  pub strength: f32,
  pub radius: f32,
  pub disabled: bool,
  triggered: bool,
  animation_frame: usize,
  animation_length: usize,
}

impl Bumper {
  pub fn new(pos: Vec2, strength: f32, assets: &Assets, effect_type: BumperType) -> Self {
    let texture = Texture2D::empty();
    texture.set_filter(FilterMode::Nearest);

    Self {
      pos,
      texture: assets.create_bumper_texture(),
      strength,
      radius: 30.0,
      disabled: false,
      triggered: false,
      animation_frame: 0,
      animation_length: match &effect_type {
        BumperType::Blue => assets.bumper_blue.animation_length,
        BumperType::White => assets.bumper_white.animation_length,
        BumperType::Pink => assets.bumper_pink.animation_length,
        BumperType::Orange => assets.bumper_orange.animation_length,
      },
      effect_type,
    }
  }

  pub fn hit(&mut self) {
    self.animation_frame = 1;
    self.disabled = true;
    self.triggered = true;

    if let BumperType::Orange = self.effect_type {
      self.strength = 0.0;
    }
  }

  pub fn fixed_update(&mut self) {
    if self.animation_frame > 0 {
      self.animation_frame += 1;
      if self.animation_frame >= self.animation_length {
        self.animation_frame = 0;
      }
    }

    if self.animation_frame > self.animation_length {
      self.animation_frame = 0;
    }

    if self.disabled {
      if self.animation_frame == 0 {
        self.disabled = false;
      }
    }
  }

  pub fn draw(&self, assets: &Assets) {
    match self.effect_type {
      BumperType::Blue => {
        assets
          .bumper_blue
          .draw(&self.texture, self.pos, self.radius, self.animation_frame)
      }
      BumperType::White => {
        assets
          .bumper_white
          .draw(&self.texture, self.pos, self.radius, self.animation_frame)
      }
      BumperType::Pink => {
        assets
          .bumper_pink
          .draw(&self.texture, self.pos, self.radius, self.animation_frame)
      }
      BumperType::Orange => {
        if self.triggered && self.animation_frame == 0 {
          assets.bumper_orange.draw(
            &self.texture,
            self.pos,
            self.radius,
            self.animation_length - 1,
          )
        } else {
          assets
            .bumper_orange
            .draw(&self.texture, self.pos, self.radius, self.animation_frame)
        }
      }
    }
  }
}

pub fn ball_to_bumper(ball: &mut Ball, bumper: &mut Bumper) -> i32 {
  if bumper.disabled {
    return 0;
  }

  let distance = (ball.pos - bumper.pos).length();
  let overlap = ball.radius + bumper.radius - distance;

  if overlap > 0.0 {
    let normal: Vec2 = (ball.pos - bumper.pos).normalize();
    ball.pos += normal * overlap;
    ball.velocity = normal * bumper.strength;

    bumper.hit();

    return bumper.strength as i32;
  }

  return 0;
}
