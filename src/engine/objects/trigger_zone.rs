use macroquad::prelude::*;

pub enum State {
  Enter,
  Stay,
  Leave,
  None,
}

pub struct TriggerZone {
  pub pos: Vec2,
  pub size: Vec2,
  pub state: State,
}

impl TriggerZone {
  fn new(pos: Vec2, size: Vec2) -> Self {
    Self {
      pos,
      size,
      state: State::None,
    }
  }
}
