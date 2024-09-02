use macroquad::prelude::*;

pub use bumper::BumperSprite;

mod bumper;

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

pub struct Assets {
  pub bumper_blue: BumperSprite,
  pub bumper_white: BumperSprite,
  pub bumper_pink: BumperSprite,
  pub bumper_orange: BumperSprite,
}

pub async fn load_assets() -> Assets {
  let bumper_blue = include_bumper!("mayaBumperBlue", "idle");
  let bumper_white = include_bumper!("mayaBumperWhite", "idle");
  let bumper_pink = include_bumper!("mayaBumperGreen", "green");
  let bumper_orange = include_bumper!("mayaBumperOrange", "orange");

  return Assets {
    bumper_blue,
    bumper_white,
    bumper_pink,
    bumper_orange,
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
