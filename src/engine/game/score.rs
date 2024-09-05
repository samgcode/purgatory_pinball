use macroquad::prelude::*;

#[derive(Clone)]
pub enum ScoreType {
  Points(i32),
  Bonus(i32),
  Multiplier(f32),
  SetMulti(f32),
}

pub struct ScoreSystem {
  pub score: i32,
  pub highscore: i32,
  multiplier: f32,
  pub lives: i32,
}

impl ScoreSystem {
  pub fn new() -> Self {
    Self {
      score: 0,
      highscore: 0,
      multiplier: 1.0,
      lives: 3,
    }
  }

  pub fn die(&mut self) {
    self.lives -= 1;
  }

  pub fn reset(&mut self) {
    if self.score > self.highscore {
      self.highscore = self.score;
    }

    self.score = 0;
    self.multiplier = 1.0;
    self.lives = 3;
  }

  pub fn apply_score(&mut self, score: ScoreType) {
    match score {
      ScoreType::Points(points) => {
        self.score += (points as f32 * self.multiplier) as i32;
        self.multiplier = 1.0;
      }
      ScoreType::Bonus(points) => {
        self.score += (points as f32 * self.multiplier) as i32;
      }
      ScoreType::Multiplier(multiplier) => {
        self.multiplier *= multiplier;
      }
      ScoreType::SetMulti(multiplier) => {
        self.multiplier = multiplier;
      }
    }
  }

  pub fn draw(&self) {
    let score = format!("Score: {}", self.score);
    let highscore = format!("Highscore: {}", self.highscore);
    let lives = format!("Lives: {}", self.lives);
    let multiplier = format!("Mult: {:.2}", self.multiplier);

    draw_text(&score, 200.0, 20.0, 30.0, WHITE);
    draw_text(&multiplier, 500.0, 20.0, 30.0, WHITE);
    draw_text(&lives, 700.0, 20.0, 30.0, WHITE);
    draw_text(&highscore, 900.0, 20.0, 30.0, WHITE);
  }
}
