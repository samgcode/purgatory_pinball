use macroquad::prelude::*;

use animated_sprite::AnimatedSprite;
use ball::BallSprite;

mod animated_sprite;
mod ball;
mod tileset;

macro_rules! include_image {
  ( $x:expr ) => {
    Image::from_file_with_format(include_bytes!($x), None).unwrap()
  };
}

macro_rules! include_bumper {
  ( $x:expr, $y:expr ) => {
    AnimatedSprite {
      scale_factor: 2.0 * 64.0 / 20.0,
      sprites: vec![
        include_image!(concat!("../assets/bumpers/", $x, "/", $y, "00.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/", $y, "34.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/", $y, "35.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/", $y, "36.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/", $y, "37.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/", $y, "38.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/", $y, "39.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/", $y, "40.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/", $y, "41.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/", $y, "42.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/", $y, "43.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/", $y, "44.png")),
      ],
      animation: vec![
        0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8,
        9, 9, 9, 9, 9, 9, 9, 9, 10, 10, 10, 10, 11, 11, 11,
      ],
      animation_length: 45,
    }
  };
}

macro_rules! include_spring {
  ( ) => {
    AnimatedSprite {
      scale_factor: 16.0 / 12.0,
      sprites: vec![
        include_image!(concat!("../assets/springreen/00.png")),
        include_image!(concat!("../assets/springreen/01.png")),
        include_image!(concat!("../assets/springreen/02.png")),
        include_image!(concat!("../assets/springreen/03.png")),
        include_image!(concat!("../assets/springreen/04.png")),
        include_image!(concat!("../assets/springreen/05.png")),
      ],
      animation: vec![
        0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 3, 3, 3, 3, 2, 2, 2, 2, 1, 1, 1, 1, 2,
        2, 2, 2, 2, 2, 2, 2, 4, 4, 4, 4, 5, 5, 5, 5, 5,
      ],
      animation_length: 46,
    }
  };
}

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

pub struct Assets {
  pub bumper_blue: AnimatedSprite,
  pub bumper_white: AnimatedSprite,
  pub bumper_pink: AnimatedSprite,
  pub bumper_orange: AnimatedSprite,
  pub tileset: tileset::Tileset,
  pub ball: ball::BallSprite,
  pub spring: AnimatedSprite,
}

pub async fn load_assets() -> Assets {
  let bumper_blue = include_bumper!("mayaBumperBlue", "idle");
  let bumper_white = include_bumper!("mayaBumperWhite", "idle");
  let bumper_pink = include_bumper!("mayaBumperGreen", "green");
  let bumper_orange = include_bumper!("mayaBumperOrange", "orange");

  let tileset = include_tileset!();

  let ball = BallSprite {
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
