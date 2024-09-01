use macroquad::prelude::*;

pub use bumper::BumperSprite;

mod bumper;

macro_rules! include_image {
  ( $x:expr ) => {
    Image::from_file_with_format(include_bytes!($x), None).unwrap()
  };
}
macro_rules! include_bumper {
  ( $x:expr ) => {
    BumperSprite {
      animation: vec![
        include_image!(concat!("../assets/bumpers/", $x, "/idle00.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/idle34.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/idle35.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/idle36.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/idle37.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/idle38.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/idle39.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/idle40.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/idle41.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/idle42.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/idle43.png")),
        include_image!(concat!("../assets/bumpers/", $x, "/idle44.png")),
      ],
    }
  };
}

pub struct Assets {
  pub bumper_blue: BumperSprite,
}

pub async fn load_assets() -> Assets {
  let bumper_blue = include_bumper!("mayaBumperWhite");

  return Assets { bumper_blue };
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
