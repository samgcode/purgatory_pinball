use macroquad::prelude::*;

const SPRITE_SCALE_FACTOR: f32 = 2.0 * 64.0 / 20.0;

pub struct BumperSprite {
  pub animation: Vec<Image>,
}

impl BumperSprite {
  pub fn draw(&self, texture: &Texture2D, pos: Vec2, scale: f32, frame: usize) {
    let size = scale * SPRITE_SCALE_FACTOR;

    texture.update(&self.animation[frame]);

    // let texture = Texture2D::from_image(&self.animation[frame]);
    // texture.set_filter(FilterMode::Nearest);
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
}
