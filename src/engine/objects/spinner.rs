use macroquad::prelude::*;

use crate::Assets;

pub struct Spinner {
  pos: Vec2,
  texture: Texture2D,
}

impl Spinner {
  pub fn new(pos: Vec2, assets: &Assets) -> Self {
    Self {
      pos,
      texture: assets.spinner.create_texture(),
    }
  }

  pub fn draw(&self, assets: &Assets) {
    assets.spinner.draw(&self.texture, self.pos, 15.0);
  }
}
