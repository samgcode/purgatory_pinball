use macroquad::prelude::*;

use crate::levels::*;
use crate::Assets;

pub struct Board {
  map_texture: Texture2D,
}

impl Board {
  pub fn new(assets: &Assets) -> Self {
    let tiles = assets.tileset.get_tiles_from_map(&LEVEL_0);

    let mut map_bytes: [u8; WIDTH * HEIGHT * 64 * 4] = [0; WIDTH * HEIGHT * 64 * 4];

    for i in 0..WIDTH {
      for j in 0..HEIGHT {
        let tile = assets.tileset.sprites[tiles[j][i]].get_image_data();
        for k in tile.iter().enumerate() {
          let x = i * 8 + k.0 % 8;
          let y = j * 8 + k.0 / 8;
          let index = (x + y * WIDTH * 8) * 4;

          map_bytes[index + 0] = k.1[0];
          map_bytes[index + 1] = k.1[1];
          map_bytes[index + 2] = k.1[2];
          map_bytes[index + 3] = k.1[3];
        }
      }
    }

    let map_texture = Texture2D::from_rgba8((WIDTH * 8) as u16, (HEIGHT * 8) as u16, &map_bytes);
    map_texture.set_filter(FilterMode::Nearest);

    return Self { map_texture };
  }

  pub fn draw(&self) {
    draw_texture_ex(
      &self.map_texture,
      0.0,
      0.0,
      WHITE,
      DrawTextureParams {
        dest_size: Some(Vec2::new(
          self.map_texture.width() * 8.0,
          self.map_texture.height() * 8.0,
        )),
        ..Default::default()
      },
    );
  }
}
