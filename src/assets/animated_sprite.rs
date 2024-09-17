use macroquad::prelude::*;

pub struct AnimatedSprite {
  pub scale_factor: f32,
  pub sprites: Vec<Image>,
  pub animation: Vec<usize>,
  pub animation_length: usize,
}

impl AnimatedSprite {
  pub fn draw(&self, texture: &Texture2D, pos: Vec2, width: f32, sprite: usize) {
    let size = width * self.scale_factor;

    texture.update(&self.sprites[self.animation[sprite]]);

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
    let texture = Texture2D::from_image(&self.sprites[0]);
    texture.set_filter(FilterMode::Nearest);

    return texture;
  }
}
