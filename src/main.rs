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

  let render_target = render_target_msaa(WIDTH as u32, HEIGHT as u32, 4);
  let shader_target = render_target_msaa(WIDTH as u32, HEIGHT as u32, 4);
  render_target.texture.set_filter(FilterMode::Linear);
  shader_target.texture.set_filter(FilterMode::Linear);

  let mut render_target_cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, WIDTH, HEIGHT));
  let mut shader_target_cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, WIDTH, HEIGHT));
  render_target_cam.render_target = Some(render_target.clone());
  shader_target_cam.render_target = Some(shader_target.clone());

  let tile_material = load_material(
    ShaderSource::Glsl {
      vertex: include_str!("vert.glsl"),
      fragment: include_str!("frag.glsl"),
    },
    MaterialParams {
      uniforms: vec![UniformDesc::new("t", UniformType::Float1)],
      textures: vec!["tex".to_string()],
      ..Default::default()
    },
  )
  .unwrap();

  let mut t: f32 = 0.0;
  let fixed_dt = 1.0 / 60.0;

  loop {
    let scale: f32 = f32::min(screen_width() / WIDTH, screen_height() / HEIGHT);

    if fixed_dt >= 1.0 / 61.0 {
      game.fixed_update(fixed_dt / 4.0);
      game.fixed_update(fixed_dt / 4.0);
      game.fixed_update(fixed_dt / 4.0);
      game.fixed_update(fixed_dt / 4.0);

      game.redraw(scale);
    }

    game.update();

    let display_size = vec2(WIDTH * scale, HEIGHT * scale);
    let display_x = (screen_width() - (WIDTH * scale)) * 0.5;
    let display_y = (screen_height() - (HEIGHT * scale)) * 0.5;

    {
      set_camera(&render_target_cam);
      clear_background(BLACK);

      game.draw();

      set_default_camera();

      draw_texture_ex(
        &render_target.texture,
        display_x + game.camera_pos.x,
        display_y + game.camera_pos.y,
        WHITE,
        DrawTextureParams {
          dest_size: Some(display_size),
          flip_y: true, // Must flip y otherwise 'render_target' will be upside down
          ..Default::default()
        },
      );
    }

    {
      set_camera(&shader_target_cam);
      clear_background(Color::new(0.0, 0.0, 0.0, 0.0));

      game.draw_shaded();

      set_default_camera();

      tile_material.set_uniform("t", t);
      tile_material.set_texture("tex", shader_target.texture.clone());
      t += 0.0003;

      gl_use_material(&tile_material);
      draw_rectangle(
        display_x + game.camera_pos.x,
        display_y + game.camera_pos.y,
        display_size.x,
        display_size.y,
        RED,
      );
      gl_use_default_material();
    }

    game.draw_ui(scale);
    draw_text("[V0.50]", 0.0, 50.0 * scale * 0.8, 50.0 * scale, WHITE);

    next_frame().await
  }
}
