use macroquad::prelude::*;

const SPRITE_SCALE_FACTOR: f32 = 16.0 / 12.0;

pub struct SpringSprite {
  pub sprites: Vec<Image>,
  pub animation: Vec<usize>,
  pub animation_length: usize,
}

impl SpringSprite {
  pub fn draw(&self, texture: &Texture2D, pos: Vec2, width: f32, sprite: usize) {
    let size = width * SPRITE_SCALE_FACTOR;

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
