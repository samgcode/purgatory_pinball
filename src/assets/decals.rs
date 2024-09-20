use macroquad::prelude::*;

use super::static_sprite::StaticSprite;

use crate::include_image;

const BG_COLOR: u8 = 64;

pub enum DecalType {
  Background,
}

pub struct Decals {
  pub background: StaticSprite,
}

impl Decals {
  pub fn load_decals() -> Self {
    let bg_image = {
      let mut image = include_image!("../../assets/bgs/circuitboard.png");
      let mut data = image.bytes.clone();
      for i in 0..data.len() / 4 {
        if data[i * 4] == 255 {
          data[i * 4] = BG_COLOR;
          data[i * 4 + 1] = BG_COLOR;
          data[i * 4 + 2] = BG_COLOR;
          data[i * 4 + 3] = BG_COLOR;
        }
      }
      image.bytes = data;
      image
    };

    let background = StaticSprite {
      scale_factor: 320.0,
      aspect_ratio: 0.5,
      sprite: bg_image,
    };

    return Self { background };
  }

  pub fn create_texture(&self, decal_type: &DecalType) -> Texture2D {
    return match decal_type {
      DecalType::Background => self.background.create_texture(),
    };
  }

  pub fn draw(&self, decal_type: &DecalType, texture: &Texture2D, pos: Vec2, scale: f32) {
    match decal_type {
      DecalType::Background => &self.background,
    }
    .draw(texture, pos, scale);
  }
}
