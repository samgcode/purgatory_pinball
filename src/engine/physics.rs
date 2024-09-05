use macroquad::prelude::*;

use crate::engine::objects::*;

pub fn ball_to_flipper(ball: &mut Ball, flipper: &Flipper) {
  let d = flipper.tip - flipper.anchor;
  let f = flipper.anchor - ball.pos;

  let a = d.dot(d);
  let b = 2.0 * f.dot(d);
  let c = f.dot(f) - ball.radius * ball.radius;

  let discriminant = b * b - 4.0 * a * c;
  if discriminant < 0.0 {
    return;
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
  }
}

pub fn ball_to_line(ball: &mut Ball, line: (Vec2, Vec2)) {
  let d = line.1 - line.0;
  let f = line.0 - ball.pos;

  let a = d.dot(d);
  let b = 2.0 * f.dot(d);
  let c = f.dot(f) - ball.radius * ball.radius;

  let discriminant = b * b - 4.0 * a * c;
  if discriminant < 0.0 {
    return;
  }

  let discriminant = discriminant.sqrt();

  let t1 = (-b - discriminant) / (2.0 * a);
  let t2 = (-b + discriminant) / (2.0 * a);

  if t1 >= 0.0 && t1 <= 1.0 || t2 >= 0.0 && t2 <= 1.0 {
    let length = line.0.distance(line.1);

    let dot = ((ball.pos.x - line.0.x) * (line.1.x - line.0.x)
      + (ball.pos.y - line.0.y) * (line.1.y - line.0.y))
      / (length * length);

    let mut closest = Vec2 {
      x: line.0.x + dot * (line.1.x - line.0.x),
      y: line.0.y + dot * (line.1.y - line.0.y),
    };

    let v = inverse_lerp_vec(closest, line.0, line.1);

    if v < 0.0 {
      closest = line.0;
    } else if v > 1.0 {
      closest = line.1;
    }

    let normal = (ball.pos - closest).normalize();
    let overlap = ball.radius - ball.pos.distance(closest);

    ball.pos += normal * overlap;

    let parallel_vel = ball.velocity.dot(normal) * normal;
    let perpendicular_vel = ball.velocity - parallel_vel;

    ball.velocity = perpendicular_vel + parallel_vel * 0.1;
  }
}

pub fn inverse_lerp_vec(v: Vec2, a: Vec2, b: Vec2) -> f32 {
  return (v.x - a.x) / (b.x - a.x);
}

pub fn ball_trigger_zone(ball: &Ball, zone: &mut TriggerZone) {
  let test = ball.pos.clamp(zone.bounds.0, zone.bounds.1);

  if ball.pos.distance(test) <= ball.radius {
    if zone.colliding {
      zone.state = CollisionState::Stay;
    } else {
      zone.state = CollisionState::Enter;
    }
    zone.colliding = true;
    return;
  }
  if zone.colliding {
    zone.state = CollisionState::Leave;
  } else {
    zone.state = CollisionState::None;
  }
  zone.colliding = false;
}
