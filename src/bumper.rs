use macroquad::prelude::*;

pub struct Bumper {
  pub pos: Vec2,
  color: Color,
  pub strength: f32,
  pub radius: f32,
}

impl Bumper {
  pub fn new(pos: Vec2, color: Color, strength: f32) -> Self {
    Self {
      pos,
      color,
      strength,
      radius: 50.0,
    }
  }

  pub fn draw(&self) {
    draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
  }
}
