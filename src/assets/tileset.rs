use macroquad::prelude::*;

use crate::levels;
use tile_rules::TILE_RULES;

mod tile_rules;

const OOB: u8 = 1;

#[macro_export]
macro_rules! include_tileset {
  ( ) => {
    tileset::Tileset {
      sprites: vec![
        include_image!(concat!("../assets/tiles/slope_corner/0.png")),
        include_image!(concat!("../assets/tiles/slope_corner/1.png")),
        include_image!(concat!("../assets/tiles/slope_corner/2.png")),
        include_image!(concat!("../assets/tiles/slope_corner/3.png")),
        // single corner with edge
        include_image!(concat!("../assets/tiles/slope_corner/15.png")),
        include_image!(concat!("../assets/tiles/slope_corner/16.png")),
        include_image!(concat!("../assets/tiles/slope_corner/17.png")),
        include_image!(concat!("../assets/tiles/slope_corner/18.png")),
        include_image!(concat!("../assets/tiles/slope_corner/19.png")),
        include_image!(concat!("../assets/tiles/slope_corner/20.png")),
        include_image!(concat!("../assets/tiles/slope_corner/21.png")),
        include_image!(concat!("../assets/tiles/slope_corner/22.png")),
        //
        include_image!(concat!("../assets/tiles/slope_corner/4.png")),
        include_image!(concat!("../assets/tiles/slope_corner/5.png")),
        include_image!(concat!("../assets/tiles/slope_corner/6.png")),
        include_image!(concat!("../assets/tiles/slope_corner/7.png")),
        include_image!(concat!("../assets/tiles/slope_corner/8.png")),
        include_image!(concat!("../assets/tiles/slope_corner/9.png")),
        //
        include_image!(concat!("../assets/tiles/slope_corner/10.png")),
        include_image!(concat!("../assets/tiles/slope_corner/11.png")),
        include_image!(concat!("../assets/tiles/slope_corner/12.png")),
        include_image!(concat!("../assets/tiles/slope_corner/13.png")),
        //
        include_image!(concat!("../assets/tiles/slope_corner/14.png")),
        //
        include_image!(concat!("../assets/tiles/slope_corner/23.png")),
        include_image!(concat!("../assets/tiles/slope_corner/24.png")),
        include_image!(concat!("../assets/tiles/slope_corner/25.png")),
        include_image!(concat!("../assets/tiles/slope_corner/26.png")),
        //
        include_image!(concat!("../assets/tiles/solid/0.png")),
        //
        include_image!(concat!("../assets/tiles/solid/1.png")),
        include_image!(concat!("../assets/tiles/solid/2.png")),
        include_image!(concat!("../assets/tiles/solid/3.png")),
        include_image!(concat!("../assets/tiles/solid/4.png")),
        //
        include_image!(concat!("../assets/tiles/slope/0.png")),
        include_image!(concat!("../assets/tiles/slope/1.png")),
        include_image!(concat!("../assets/tiles/slope/2.png")),
        include_image!(concat!("../assets/tiles/slope/3.png")),
        //
        include_image!(concat!("../assets/tiles/slope/4.png")),
        include_image!(concat!("../assets/tiles/slope/5.png")),
        include_image!(concat!("../assets/tiles/slope/6.png")),
        include_image!(concat!("../assets/tiles/slope/7.png")),
        include_image!(concat!("../assets/tiles/slope/8.png")),
        include_image!(concat!("../assets/tiles/slope/9.png")),
        include_image!(concat!("../assets/tiles/slope/10.png")),
        include_image!(concat!("../assets/tiles/slope/11.png")),
        //
        include_image!(concat!("../assets/tiles/solid/9.png")),
        include_image!(concat!("../assets/tiles/solid/10.png")),
        include_image!(concat!("../assets/tiles/solid/11.png")),
        include_image!(concat!("../assets/tiles/solid/12.png")),
        include_image!(concat!("../assets/tiles/solid/13.png")),
        include_image!(concat!("../assets/tiles/solid/14.png")),
        //
        include_image!(concat!("../assets/tiles/solid/15.png")),
        //
        include_image!(concat!("../assets/tiles/empty.png")),
        include_image!(concat!("../assets/tiles/solid/15.png")),
      ],
    }
  };
}

#[macro_export]
macro_rules! include_basic_tileset {
  ( ) => {
    tileset::BasicTileset {
      sprites: vec![
        include_image!(concat!("../../assets/decals/empty.png")),
        include_image!(concat!("../../assets/decals/line_a.png")),
        include_image!(concat!("../../assets/decals/line_b.png")),
        include_image!(concat!("../../assets/decals/line_c.png")),
        include_image!(concat!("../../assets/decals/line_d.png")),
        include_image!(concat!("../../assets/decals/diag_a.png")),
        include_image!(concat!("../../assets/decals/diag_b.png")),
        include_image!(concat!("../../assets/decals/circle_a.png")),
        include_image!(concat!("../../assets/decals/circle_b.png")),
        include_image!(concat!("../../assets/decals/circle_c.png")),
        include_image!(concat!("../../assets/decals/circle_d.png")),
        include_image!(concat!("../../assets/decals/corner_a.png")),
        include_image!(concat!("../../assets/decals/corner_b.png")),
        include_image!(concat!("../../assets/decals/corner_c.png")),
        include_image!(concat!("../../assets/decals/corner_d.png")),
      ],
    }
  };
}

pub struct Tileset {
  pub sprites: Vec<Image>,
}

impl Tileset {
  pub fn get_tiles_from_map(&self, map: &[[u8; levels::WIDTH]; levels::HEIGHT]) -> Vec<Vec<usize>> {
    let mut tiles = Vec::new();

    for i in 0..map.len() {
      let mut row = Vec::new();
      for j in 0..map[i].len() {
        for t in 0..TILE_RULES.len() {
          let square = [
            [
              index_2d(map, i as i32 - 1, j as i32 - 1, OOB),
              index_2d(map, i as i32 - 1, j as i32, OOB),
              index_2d(map, i as i32 - 1, j as i32 + 1, OOB),
            ],
            [
              index_2d(map, i as i32, j as i32 - 1, OOB),
              index_2d(map, i as i32, j as i32, OOB),
              index_2d(map, i as i32, j as i32 + 1, OOB),
            ],
            [
              index_2d(map, i as i32 + 1, j as i32 - 1, OOB),
              index_2d(map, i as i32 + 1, j as i32, OOB),
              index_2d(map, i as i32 + 1, j as i32 + 1, OOB),
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

fn index_2d(arr: &[[u8; levels::WIDTH]; levels::HEIGHT], i: i32, j: i32, default: u8) -> u8 {
  if i < 0 || i >= arr.len() as i32 || j < 0 || j >= arr[i as usize].len() as i32 {
    return default;
  }

  let a = arr[i as usize][j as usize];
  if a == 2 || a == 3 {
    return 1;
  }
  return a;
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

pub struct BasicTileset {
  pub sprites: Vec<Image>,
}

impl BasicTileset {
  pub fn get_tiles_from_map(
    &self,
    map: &[[(u8, u8); levels::DECAL_WIDTH]; levels::DECAL_HEIGHT],
  ) -> Vec<Vec<(usize, usize)>> {
    let mut tiles = Vec::new();

    for i in 0..map.len() {
      let mut row = Vec::new();
      for j in 0..map[i].len() {
        if i == 0 || j == 0 || i == map.len() - 1 || j == map[i].len() - 1 {
          row.push((map[i][j].0 as usize, map[i][j].1 as usize));
          continue;
        }

        let left = map[i][j - 1].0;
        let right = map[i][j + 1].0;
        let up = map[i - 1][j].0;
        let down = map[i + 1][j].0;

        if (left == 5 && up != 0) || (left != 0 && up == 5) {
          row.push((14, map[i][j].1 as usize));
          continue;
        }
        if left == 6 && down == 6 {
          row.push((13, map[i][j].1 as usize));
          continue;
        }
        if (right == 5 && down != 0) || (right != 0 && down == 5) {
          row.push((12, map[i][j].1 as usize));
          continue;
        }
        if right == 6 && up == 6 {
          row.push((11, map[i][j].1 as usize));
          continue;
        }

        row.push((map[i][j].0 as usize, map[i][j].1 as usize));
      }

      tiles.push(row);
    }

    return tiles;
  }
}
