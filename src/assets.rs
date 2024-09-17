use macroquad::prelude::*;

use animated_sprite::AnimatedSprite;
use static_sprite::StaticSprite;

use crate::include_bumper;
use crate::include_spring;
use crate::include_tileset;

mod animated_sprite;
mod static_sprite;
mod tileset;

macro_rules! include_image {
  ( $x:expr ) => {
    Image::from_file_with_format(include_bytes!($x), None).unwrap()
  };
}

pub struct Assets {
  pub bumper_blue: AnimatedSprite,
  pub bumper_white: AnimatedSprite,
  pub bumper_pink: AnimatedSprite,
  pub bumper_orange: AnimatedSprite,
  pub tileset: tileset::Tileset,
  pub ball: StaticSprite,
  pub spring: AnimatedSprite,
}

pub async fn load_assets() -> Assets {
  let bumper_blue = include_bumper!("mayaBumperBlue", "idle");
  let bumper_white = include_bumper!("mayaBumperWhite", "idle");
  let bumper_pink = include_bumper!("mayaBumperGreen", "green");
  let bumper_orange = include_bumper!("mayaBumperOrange", "orange");

  let tileset = include_tileset!();

  let ball = StaticSprite {
    scale_factor: 2.0 * 64.0 / 20.0,
    sprite: include_image!("../assets/ball.png"),
  };

  let spring = include_spring!();

  return Assets {
    bumper_blue,
    bumper_white,
    bumper_pink,
    bumper_orange,
    tileset,
    ball,
    spring,
  };
}

impl Assets {
  pub fn create_bumper_texture(&self) -> Texture2D {
    let texture = Texture2D::from_image(&include_image!(
      "../assets/bumpers/mayaBumperBlue/idle00.png"
    ));
    texture.set_filter(FilterMode::Nearest);

    return texture;
  }
}
