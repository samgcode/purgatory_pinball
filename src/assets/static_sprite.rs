use macroquad::prelude::*;

pub struct StaticSprite {
  pub scale_factor: f32,
  pub sprite: Image,
}

impl StaticSprite {
  pub fn draw(&self, texture: &Texture2D, pos: Vec2, scale: f32) {
    let size = scale * self.scale_factor;

    draw_texture_ex(
      texture,
      pos.x - size / 2.0,
      pos.y - size / 2.0,
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
