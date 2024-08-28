use macroquad::prelude::*;

use crate::bumper::*;

pub struct Ball {
  pos: Vec2,
  velocity: Vec2,
  radius: f32,
}

impl Ball {
  pub fn new(pos: Vec2, velocity: Vec2) -> Self {
    Self {
      pos,
      velocity,
      radius: 20.0,
    }
  }

  pub fn update(&mut self, gravity: Vec2, dt: f32) {
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

  pub fn update_collision(&mut self, bumper: &Bumper) {
    let distance = (self.pos - bumper.pos).length();
    let overlap = self.radius + bumper.radius - distance;

    if overlap > 0.0 {
      let normal: Vec2 = (self.pos - bumper.pos).normalize();
      self.pos += normal * overlap;
      self.velocity = normal * bumper.strength;
    }
  }
}
