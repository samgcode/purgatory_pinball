use macroquad::prelude::*;

use crate::{assets, Assets};

pub struct Ball {
  pub pos: Vec2,
  pub velocity: Vec2,
  pub radius: f32,
  texture: Texture2D,
}

impl Ball {
  pub fn new(pos: Vec2, velocity: Vec2, assets: &Assets) -> Self {
    Self {
      pos,
      velocity,
      radius: 25.0,
      texture: assets.ball.create_texture(),
    }
  }

  pub fn fixed_update(&mut self, gravity: Vec2, dt: f32) {
    self.pos += self.velocity * dt;
    self.velocity += gravity * dt;
  }

  pub fn draw(&self, assets: &Assets) {
    assets.ball.draw(&self.texture, self.pos, self.radius);
  }
}
