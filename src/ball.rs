use macroquad::prelude::*;

pub struct Ball {
  pub pos: Vec2,
  pub velocity: Vec2,
  pub radius: f32,
}

impl Ball {
  pub fn new(pos: Vec2, velocity: Vec2) -> Self {
    Self {
      pos,
      velocity,
      radius: 20.0,
    }
  }

  pub fn fixed_update(&mut self, gravity: Vec2, dt: f32) {
    self.pos += self.velocity * dt;
    self.velocity += gravity * dt;
  }

  pub fn draw(&self) {
    draw_circle(
      self.pos.x,
      self.pos.y,
      self.radius,
      Color::new(0.5, 0.5, 0.5, 1.0),
    );
  }
}
