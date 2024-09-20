use macroquad::prelude::*;

use crate::assets::Assets;
use crate::assets::DecalType;

pub struct Decal {
  pub pos: Vec2,
  decal_type: DecalType,
  texture: Texture2D,
}

impl Decal {
  pub fn new(pos: Vec2, assets: &Assets, decal_type: DecalType) -> Self {
    let texture = assets.decals.create_texture(&decal_type);
    return Decal {
      pos,
      decal_type,
      texture,
    };
  }

  pub fn draw(&self, assets: &Assets) {
    assets
      .decals
      .draw(&self.decal_type, &self.texture, self.pos, 4.0);
  }
}
