use macroquad::prelude::*;

const SPRITE_SCALE_FACTOR: f32 = 2.0 * 64.0 / 20.0;

pub struct BallSprite {
  pub sprite: Image,
}

impl BallSprite {
  pub fn draw(&self, texture: &Texture2D, pos: Vec2, scale: f32) {
    let size = scale * SPRITE_SCALE_FACTOR;

    draw_texture_ex(
      texture,
      pos.x - size / 2.0 + 2.0,
      pos.y - size / 2.0 - 2.0,
      WHITE,
      DrawTextureParams {
        dest_size: Some(Vec2::new(size, size)),
        ..Default::default()
      },
    );
  }

  pub fn create_texture(&self) -> Texture2D {
    let texture = Texture2D::from_image(&self.sprite);
    texture.set_filter(FilterMode::Nearest);

    return texture;
  }
}
