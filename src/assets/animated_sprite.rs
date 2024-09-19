use macroquad::prelude::*;

#[macro_export]
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

#[macro_export]
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
        include_image!(concat!("../assets/springreen/06.png")),
        include_image!(concat!("../assets/springreen/07.png")),
        include_image!(concat!("../assets/springreen/08.png")),
        include_image!(concat!("../assets/springreen/09.png")),
        include_image!(concat!("../assets/springreen/10.png")),
        include_image!(concat!("../assets/springreen/11.png")),
        include_image!(concat!("../assets/springreen/12.png")),
        include_image!(concat!("../assets/springreen/13.png")),
        include_image!(concat!("../assets/springreen/14.png")),
        include_image!(concat!("../assets/springreen/15.png")),
        include_image!(concat!("../assets/springreen/16.png")),
        include_image!(concat!("../assets/springreen/17.png")),
        include_image!(concat!("../assets/springreen/18.png")),
        include_image!(concat!("../assets/springreen/19.png")),
      ],
      animation: vec![
        0, 1, 0, 0, 0, 2, 6, 7, 7, 9, 18, 11, 13, 15, 16, 16, 15, 17, 11, 11, 10, 8, 8, 8, 18, 12,
        11, 11, 19, 17, 15, 17, 14, 11, 11, 11, 10, 10, 10, 12, 11, 11, 11, 13, 14, 11, 5, 5, 5, 5,
        4, 4, 4, 4, 3, 3, 3, 3,
      ],
      animation_length: 58,
    }
  };
}

pub struct AnimatedSprite {
  pub scale_factor: f32,
  pub sprites: Vec<Image>,
  pub animation: Vec<usize>,
  pub animation_length: usize,
}

impl AnimatedSprite {
  pub fn draw(&self, texture: &Texture2D, pos: Vec2, width: f32, sprite: usize, rotation: f32) {
    let size = width * self.scale_factor;

    texture.update(&self.sprites[self.animation[sprite]]);

    draw_texture_ex(
      texture,
      pos.x - size / 2.0,
      pos.y - size / 2.0,
      WHITE,
      DrawTextureParams {
        dest_size: Some(Vec2::new(size, size)),
        rotation,
        ..Default::default()
      },
    );
  }

  pub fn create_texture(&self) -> Texture2D {
    let texture = Texture2D::from_image(&self.sprites[0]);
    texture.set_filter(FilterMode::Nearest);

    return texture;
  }
}
