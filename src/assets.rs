use macroquad::prelude::*;

pub use bumper::BumperSprite;

mod bumper;
mod tileset;

macro_rules! include_image {
  ( $x:expr ) => {
    Image::from_file_with_format(include_bytes!($x), None).unwrap()
  };
}

macro_rules! include_bumper {
  ( $x:expr, $y:expr ) => {
    BumperSprite {
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
  pub bumper_blue: BumperSprite,
  pub bumper_white: BumperSprite,
  pub bumper_pink: BumperSprite,
  pub bumper_orange: BumperSprite,
  pub tileset: tileset::Tileset,
}

pub async fn load_assets() -> Assets {
  let bumper_blue = include_bumper!("mayaBumperBlue", "idle");
  let bumper_white = include_bumper!("mayaBumperWhite", "idle");
  let bumper_pink = include_bumper!("mayaBumperGreen", "green");
  let bumper_orange = include_bumper!("mayaBumperOrange", "orange");

  let tileset = include_tileset!();

  return Assets {
    bumper_blue,
    bumper_white,
    bumper_pink,
    bumper_orange,
    tileset,
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
