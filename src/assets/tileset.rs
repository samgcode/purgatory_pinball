use macroquad::prelude::*;

pub struct Tileset {
  pub sprites: Vec<Image>,
}

impl Tileset {
  pub fn get_tiles_from_map(&self, map: &Vec<Vec<u8>>) -> Vec<Vec<usize>> {
    let mut tiles = Vec::new();

    for i in 0..map.len() {
      let mut row = Vec::new();
      for j in 0..map[i].len() {
        if map[i][j] == 1 {
          row.push(0 as usize);
          continue;
        }
        row.push(1 as usize);
      }

      tiles.push(row);
    }

    return tiles;
  }
}
