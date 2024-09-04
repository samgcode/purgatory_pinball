use macroquad::prelude::*;

pub enum CollisionState {
  Enter,
  Stay,
  Leave,
  None,
}

pub struct TriggerZone {
  pub bounds: (Vec2, Vec2),
  pub colliding: bool,
  pub state: CollisionState,
}

impl TriggerZone {
  pub fn new(pos: Vec2, size: Vec2) -> Self {
    Self {
      bounds: (pos, pos + size),
      colliding: false,
      state: CollisionState::None,
    }
  }

  pub fn draw(&self) {
    draw_rectangle_lines(
      self.bounds.0.x,
      self.bounds.0.y,
      self.bounds.1.x - self.bounds.0.x,
      self.bounds.1.y - self.bounds.0.y,
      2.0,
      RED,
    )
  }
}
