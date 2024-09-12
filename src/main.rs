use macroquad::prelude::*;

use assets::*;
use engine::*;

mod assets;
mod drawing;
mod engine;

const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = 1080.0;

#[macroquad::main("purgatory pinball")]
async fn main() {
  let mut game = Game::init().await;

  let mut fixed_dt: f32 = 0.0;

  let render_target = render_target_msaa(WIDTH as u32, HEIGHT as u32, 4);
  render_target.texture.set_filter(FilterMode::Linear);

  let mut render_target_cam = Camera2D::from_display_rect(Rect::new(0., 0., WIDTH, HEIGHT));
  render_target_cam.render_target = Some(render_target.clone());

  loop {
    let dt = get_frame_time();
    fixed_dt += dt;

    let scale: f32 = f32::min(screen_width() / WIDTH, screen_height() / HEIGHT);

    set_camera(&render_target_cam);

    clear_background(BLACK);

    if fixed_dt >= 1.0 / 61.0 {
      game.fixed_update(fixed_dt / 4.0);
      game.fixed_update(fixed_dt / 4.0);
      game.fixed_update(fixed_dt / 4.0);
      game.fixed_update(fixed_dt / 4.0);

      game.redraw();

      fixed_dt = 0.0;
    }

    game.update(dt);
    game.draw();

    draw_text("[V0.29]", 0.0, 20.0, 30.0, WHITE);

    set_default_camera();

    draw_texture_ex(
      &render_target.texture,
      (screen_width() - (WIDTH * scale)) * 0.5,
      (screen_height() - (HEIGHT * scale)) * 0.5,
      WHITE,
      DrawTextureParams {
        dest_size: Some(vec2(WIDTH * scale, HEIGHT * scale)),
        flip_y: true, // Must flip y otherwise 'render_target' will be upside down
        ..Default::default()
      },
    );

    next_frame().await
  }
}
