use macroquad::prelude::*;

use super::Ball;
use crate::game::ScoreType;
use crate::Assets;

pub enum BumperType {
  White,
  Blue,
  Pink,
  Orange,
}

pub struct Bumper {
  pub pos: Vec2,
  texture: Texture2D,
  effect_type: BumperType,
  score: ScoreType,
  pub strength: f32,
  pub radius: f32,
  pub disabled: bool,
  triggered: bool,
  animation_frame: usize,
  animation_length: usize,
}

impl Bumper {
  pub fn new(
    pos: Vec2,
    custom_strength: Option<f32>,
    custom_score: Option<ScoreType>,
    assets: &Assets,
    effect_type: BumperType,
  ) -> Self {
    let texture = Texture2D::empty();
    texture.set_filter(FilterMode::Nearest);

    let strength = custom_strength.unwrap_or(match &effect_type {
      BumperType::White => 200.0,
      BumperType::Blue => 500.0,
      BumperType::Pink => 1000.0,
      BumperType::Orange => 500.0,
    });

    let animation_length = match &effect_type {
      BumperType::White => assets.bumper_white.animation_length,
      BumperType::Blue => assets.bumper_blue.animation_length,
      BumperType::Pink => assets.bumper_pink.animation_length,
      BumperType::Orange => assets.bumper_orange.animation_length,
    };

    let score = custom_score.unwrap_or(match &effect_type {
      BumperType::White => ScoreType::Points(100),
      BumperType::Blue => ScoreType::Points(250),
      BumperType::Pink => ScoreType::Multiplier(2.0),
      BumperType::Orange => ScoreType::Points(1000),
    });

    Self {
      pos,
      texture: assets.create_bumper_texture(),
      strength,
      radius: 50.0,
      disabled: false,
      triggered: false,
      animation_frame: 0,
      animation_length,
      score,
      effect_type,
    }
  }

  pub fn hit(&mut self) {
    self.animation_frame = 1;
    self.disabled = true;
    self.triggered = true;

    if let BumperType::Orange = self.effect_type {
      self.strength = 0.0;
      self.score = ScoreType::SetMulti(-1.0);
    }
  }

  pub fn redraw(&mut self) {
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

  pub fn reset(&mut self) {
    self.disabled = false;
    self.triggered = false;
    if let BumperType::Orange = self.effect_type {
      self.score = ScoreType::Points(1000);
    }
  }

  pub fn draw(&self, assets: &Assets) {
    match self.effect_type {
      BumperType::White => {
        assets
          .bumper_white
          .draw(&self.texture, self.pos, self.radius, self.animation_frame)
      }
      BumperType::Blue => {
        assets
          .bumper_blue
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

pub fn ball_to_bumper(ball: &mut Ball, bumper: &mut Bumper) -> Option<ScoreType> {
  if bumper.disabled {
    return None;
  }

  let distance = (ball.pos - bumper.pos).length();
  let overlap = ball.radius + bumper.radius - distance;

  if overlap > 0.0 {
    let normal: Vec2 = (ball.pos - bumper.pos).normalize();
    ball.pos += normal * overlap;
    ball.velocity = normal * bumper.strength;

    let score = bumper.score.clone();
    bumper.hit();

    return Some(score);
  }

  return None;
}
