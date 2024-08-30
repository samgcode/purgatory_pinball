use macroquad::prelude::*;

use crate::Ball;
use crate::Flipper;

pub fn update_collision(ball: &mut Ball, flipper: &mut Flipper) {
  ball_to_flipper(ball, flipper);
}

fn ball_to_flipper(ball: &mut Ball, flipper: &Flipper) -> bool {
  let d = flipper.tip - flipper.anchor;
  let f = flipper.anchor - ball.pos;

  let a = d.dot(d);
  let b = 2.0 * f.dot(d);
  let c = f.dot(f) - ball.radius * ball.radius;

  let discriminant = b * b - 4.0 * a * c;
  if discriminant < 0.0 {
    return false;
  }

  let discriminant = discriminant.sqrt();

  let t1 = (-b - discriminant) / (2.0 * a);
  let t2 = (-b + discriminant) / (2.0 * a);

  if t1 >= 0.0 && t1 <= 1.0 || t2 >= 0.0 && t2 <= 1.0 {
    let dot = ((ball.pos.x - flipper.anchor.x) * (flipper.tip.x - flipper.anchor.x)
      + (ball.pos.y - flipper.anchor.y) * (flipper.tip.y - flipper.anchor.y))
      / (flipper.length * flipper.length);

    let mut closest = Vec2 {
      x: flipper.anchor.x + dot * (flipper.tip.x - flipper.anchor.x),
      y: flipper.anchor.y + dot * (flipper.tip.y - flipper.anchor.y),
    };

    let v = inverse_lerp_vec(closest, flipper.anchor, flipper.tip);

    if v < 0.0 {
      closest = flipper.anchor;
    } else if v > 1.0 {
      closest = flipper.tip;
    }

    let normal = (ball.pos - closest).normalize();
    let overlap = ball.radius - ball.pos.distance(closest);

    ball.pos += normal * overlap;

    let parallel_vel = ball.velocity.dot(normal) * normal;
    let perpendicular_vel = ball.velocity - parallel_vel;

    let r = flipper.anchor.distance(closest);
    let v = -(flipper.vel * r * r * std::f32::consts::PI) / 360.0 * 0.016;

    ball.velocity = perpendicular_vel + normal * v;

    return true;
  }

  return false;
}

//   fn ball_to_line() {
//   let dot = ((ball.pos.x - flipper.anchor.x) * (flipper.tip.x - flipper.anchor.x)
//     + (ball.pos.y - flipper.anchor.y) * (flipper.tip.y - flipper.anchor.y))
//     / (flipper.length * flipper.length);

//   let closest = Vec2 {
//     x: flipper.anchor.x + dot * (flipper.tip.x - flipper.anchor.x),
//     y: flipper.anchor.y + dot * (flipper.tip.y - flipper.anchor.y),
//   };

//   draw_circle(closest.x, closest.y, 5.0, RED);

//   let v = inverse_lerp_vec(closest, flipper.anchor, flipper.tip);

//   if v < 0.0 || v > 1.0 {
//     return None;
//   }

//   if closest.distance(ball.pos) < ball.radius {
//     return Some(closest);
//   }
//   return None;
// }

pub fn inverse_lerp_vec(v: Vec2, a: Vec2, b: Vec2) -> f32 {
  return (v.x - a.x) / (b.x - a.x);
}
