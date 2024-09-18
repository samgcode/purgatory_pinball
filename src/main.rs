use macroquad::prelude::*;

use assets::*;
use engine::*;

mod assets;
mod drawing;
mod engine;

pub const WIDTH: f32 = 1920.0;
pub const HEIGHT: f32 = 1080.0;

#[macroquad::main("purgatory pinball")]
async fn main() {
  let mut game = Game::init().await;

  let mut fixed_dt: f32 = 0.0;

  let render_target = render_target_msaa(WIDTH as u32, HEIGHT as u32, 4);
  render_target.texture.set_filter(FilterMode::Linear);

  let mut render_target_cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, WIDTH, HEIGHT));
  render_target_cam.render_target = Some(render_target.clone());

  let tile_material = load_material(
    ShaderSource::Glsl {
      vertex: include_str!("vert.glsl"),
      fragment: include_str!("frag.glsl"),
    },
    MaterialParams {
      uniforms: vec![UniformDesc::new("t", UniformType::Float1)],
      ..Default::default()
    },
  )
  .unwrap();

  // tile_material.set_texture("tex", render_target.texture);
  let mut t: f32 = 0.0;

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

      game.redraw(scale);

      fixed_dt = 0.0;
    }

    game.update(dt);
    game.draw();

    set_default_camera();

    tile_material.set_uniform("t", t);
    t += 0.01;

    draw_texture_ex(
      &render_target.texture,
      (screen_width() - (WIDTH * scale)) * 0.5 + game.camera_pos.x,
      (screen_height() - (HEIGHT * scale)) * 0.5 + game.camera_pos.y,
      WHITE,
      DrawTextureParams {
        dest_size: Some(vec2(WIDTH * scale, HEIGHT * scale)),
        flip_y: true, // Must flip y otherwise 'render_target' will be upside down
        ..Default::default()
      },
    );
    gl_use_material(&tile_material);
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), RED);
    gl_use_default_material();

    game.draw_ui(scale);
    draw_text("[V0.42]", 0.0, 20.0, 30.0, WHITE);

    next_frame().await
  }
}
