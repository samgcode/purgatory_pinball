use macroquad::prelude::*;

use tile_rules::TILE_RULES;

mod tile_rules;

pub struct Tileset {
  pub sprites: Vec<Image>,
}

impl Tileset {
  pub fn get_tiles_from_map(&self, map: &Vec<Vec<u8>>) -> Vec<Vec<usize>> {
    let mut tiles = Vec::new();

    for i in 0..map.len() {
      let mut row = Vec::new();
      for j in 0..map[i].len() {
        for t in 0..TILE_RULES.len() {
          let square = [
            [
              index_2d(map, i as i32 - 1, j as i32 - 1, 1),
              index_2d(map, i as i32 - 1, j as i32, 1),
              index_2d(map, i as i32 - 1, j as i32 + 1, 1),
            ],
            [
              index_2d(map, i as i32, j as i32 - 1, 1),
              index_2d(map, i as i32, j as i32, 1),
              index_2d(map, i as i32, j as i32 + 1, 1),
            ],
            [
              index_2d(map, i as i32 + 1, j as i32 - 1, 1),
              index_2d(map, i as i32 + 1, j as i32, 1),
              index_2d(map, i as i32 + 1, j as i32 + 1, 1),
            ],
          ];

          if match_rule(t, square) {
            row.push(t);
            break;
          }
        }
      }

      tiles.push(row);
    }

    return tiles;
  }
}

fn index_2d(arr: &Vec<Vec<u8>>, i: i32, j: i32, default: u8) -> u8 {
  if i < 0 || i >= arr.len() as i32 || j < 0 || j >= arr[i as usize].len() as i32 {
    return default;
  }
  return arr[i as usize][j as usize];
}

fn match_rule(ti: usize, square: [[u8; 3]; 3]) -> bool {
  for i in 0..3 {
    for j in 0..3 {
      if TILE_RULES[ti][i][j] != 9 && square[i][j] != 9 && TILE_RULES[ti][i][j] != square[i][j] {
        return false;
      }
    }
  }

  return true;
}
