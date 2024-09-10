use macroquad::prelude::*;

use crate::levels::*;
use crate::Assets;

const SCALE: f32 = 4.0;
const CENTER_OFFSET: f32 = 1920.0 / 2.0 - WIDTH as f32 * 8.0 * SCALE / 2.0;

pub struct Board {
  map_texture: Texture2D,
  pub walls: Vec<(Vec2, Vec2)>,
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

    let mut walls = Vec::new();
    for y in 0..LEVEL_0.len() {
      for x in 0..LEVEL_0[y].len() {
        if LEVEL_0[y][x] == 0 {
          if x < 1 || y < 1 || x >= WIDTH as usize || y >= HEIGHT as usize {
            continue;
          }
          let scale = 8.0 * SCALE;
          if LEVEL_0[y - 1][x] == 1 {
            let x = x as f32 * scale + CENTER_OFFSET;
            let y = y as f32 * scale;
            walls.push((Vec2::new(x, y), Vec2::new(x + scale, y)));
          }
          if LEVEL_0[y + 1][x] == 1 {
            let x = x as f32 * scale + CENTER_OFFSET;
            let y = y as f32 * scale;
            walls.push((Vec2::new(x, y + scale), Vec2::new(x + scale, y + scale)));
          }
          if LEVEL_0[y][x - 1] == 1 {
            let x = x as f32 * scale + CENTER_OFFSET;
            let y = y as f32 * scale;
            walls.push((Vec2::new(x, y), Vec2::new(x, y + scale)));
          }
          if LEVEL_0[y][x + 1] == 1 {
            let x = x as f32 * scale + CENTER_OFFSET;
            let y = y as f32 * scale;
            walls.push((Vec2::new(x + scale, y), Vec2::new(x + scale, y + scale)));
          }
        }
      }
    }

    return Self { map_texture, walls };
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

    for line in self.walls.iter() {
      draw_line(line.0.x, line.0.y, line.1.x, line.1.y, 2.0, RED)
    }
  }
}
