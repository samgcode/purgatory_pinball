use macroquad::prelude::*;

use crate::{levels::*, Assets};

const SPRITE_SIZE: usize = 15;
const SCALE: f32 = 4.0;
const CENTER_OFFSET: f32 = 1920.0 / 2.0 - DECAL_WIDTH as f32 * (SPRITE_SIZE as f32) * SCALE / 2.0;

const PUMBER_PALLETTE: [[u8; 3]; 3] = [[255, 74, 74], [130, 255, 255], [255, 92, 240]];

pub struct DecalMap {
  map_texture: Texture2D,
}

impl DecalMap {
  pub fn new(assets: &Assets) -> Self {
    let tiles = assets.decals.bg_lines.get_tiles_from_map(&LEVEL_0_DECAL);

    let mut map_bytes: [u8; DECAL_WIDTH * DECAL_HEIGHT * SPRITE_SIZE * SPRITE_SIZE * 4] =
      [0; DECAL_WIDTH * DECAL_HEIGHT * SPRITE_SIZE * SPRITE_SIZE * 4];

    for i in 0..DECAL_WIDTH {
      for j in 0..DECAL_HEIGHT {
        let tile = assets.decals.bg_lines.sprites[tiles[j][i].0].get_image_data();
        for k in 0..SPRITE_SIZE {
          for l in 0..SPRITE_SIZE {
            let x = i * SPRITE_SIZE + k;
            let y = j * SPRITE_SIZE + l;

            let index = (x + y * DECAL_WIDTH * SPRITE_SIZE) * 4;

            let color = PUMBER_PALLETTE[tiles[j][i].1];

            let pixel = tile[l * SPRITE_SIZE + k];

            map_bytes[index + 0] = color[0];
            map_bytes[index + 1] = color[1];
            map_bytes[index + 2] = color[2];
            map_bytes[index + 3] = pixel[3];
          }
        }
      }
    }

    let map_texture = Texture2D::from_rgba8(
      (DECAL_WIDTH * SPRITE_SIZE) as u16,
      (DECAL_HEIGHT * SPRITE_SIZE) as u16,
      &map_bytes,
    );
    map_texture.set_filter(FilterMode::Nearest);

    Self { map_texture }
  }

  pub fn draw(&self) {
    draw_texture_ex(
      &self.map_texture,
      CENTER_OFFSET,
      0.0,
      WHITE,
      DrawTextureParams {
        dest_size: Some(Vec2::new(
          self.map_texture.width() * SCALE,
          self.map_texture.height() * SCALE,
        )),
        ..Default::default()
      },
    );
  }
}
