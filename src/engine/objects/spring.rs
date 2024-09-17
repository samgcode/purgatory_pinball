use macroquad::prelude::*;

use super::TriggerZone;

const WIDTH: f32 = 48.0;
const HEIGHT: f32 = 12.0;
const DEFAULT_STRENGTH: f32 = 500.0;

#[allow(unused)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

use Direction::*;

pub struct Spring {
  pub pos: Vec2,
  pub direction: Direction,
  pub normal: Vec2,
  pub strength: f32,
  pub collider: TriggerZone, // texture: Texture2D,
}

impl Spring {
  pub fn new(pos: Vec2, direction: Direction, strength: Option<f32>) -> Self {
    if let Direction::Right | Direction::Left = direction {}

    let collider = match direction {
      Up | Down => TriggerZone::new(
        pos - Vec2::new(WIDTH / 2.0, HEIGHT / 2.0),
        Vec2::new(WIDTH, HEIGHT),
      ),
      Right | Left => TriggerZone::new(
        pos - Vec2::new(HEIGHT / 2.0, WIDTH / 2.0),
        Vec2::new(HEIGHT, WIDTH),
      ),
    };

    let normal = match direction {
      Up => Vec2::new(0.0, -1.0),
      Down => Vec2::new(0.0, 1.0),
      Left => Vec2::new(-1.0, 0.0),
      Right => Vec2::new(1.0, 0.0),
    };

    Self {
      pos,
      direction, // texture: Texture2D::empty(),
      normal,
      strength: strength.unwrap_or(DEFAULT_STRENGTH),
      collider,
    }
  }

  pub fn draw(&self) {
    let (x, y, w, h) = match self.direction {
      Direction::Up | Direction::Down => (
        self.pos.x - WIDTH / 2.0,
        self.pos.y - HEIGHT / 2.0,
        WIDTH,
        HEIGHT,
      ),
      Direction::Left | Direction::Right => (
        self.pos.x - HEIGHT / 2.0,
        self.pos.y - WIDTH / 2.0,
        HEIGHT,
        WIDTH,
      ),
    };

    draw_rectangle(x, y, w, h, GREEN);
    // self.collider.draw();
  }
}
