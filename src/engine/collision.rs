use std::f32::INFINITY;

use macroquad::prelude::*;

use super::{math::*, Shape, *};

const EPS: f32 = 0.00001;
const ELASTICITY: f32 = 0.8;
const DAMPING: f32 = 0.8;
const FRICTION_COEFFICIENT: f32 = 50.0;

pub struct Collision {
  d: f32,
  normal: Vec2,
  t: f32,
  line: (usize, usize),
}

pub fn shape_shape_collision(shape_a: &mut Shape, shape_b: &mut Shape, delta_time: f32) {
  for point in shape_a.points.iter_mut() {
    if let Some(collision) = point_shape_collision(point.position, shape_b) {
      resolve_point_line(point, shape_b, collision, delta_time);
    }
  }
}

pub fn resolve_point_line(
  point: &mut PointMass,
  shape: &mut Shape,
  collision: Collision,
  delta_time: f32,
) {
  let point_a = &shape.points[collision.line.0];
  let point_b = &shape.points[collision.line.1];

  let l_vel = point_a.velocity + point_b.velocity;
  let p_vel = point.velocity;

  let dist = collision.d * collision.normal;

  let perpendicular_n = (point_b.position - point_a.position).normalize();

  let v_reflected = p_vel - 2.0 * (p_vel.dot(collision.normal)) * collision.normal;
  let l_reflected = l_vel - 2.0 * (l_vel.dot(collision.normal)) * collision.normal;

  let friction_force =
    perpendicular_n * perpendicular_n.dot(v_reflected) * FRICTION_COEFFICIENT * delta_time;

  point.add_position(dist * 0.5);
  point.velocity = v_reflected * ELASTICITY + l_vel * DAMPING - friction_force;

  let particle = &mut shape.points[collision.line.0];
  particle.add_position((1.0 - collision.t) * -dist);
  particle.velocity = (1.0 - collision.t) * (l_reflected * ELASTICITY + p_vel * 0.5 * DAMPING);

  let particle = &mut shape.points[collision.line.1];
  particle.add_position(collision.t * -dist);
  particle.velocity = collision.t * (l_reflected * ELASTICITY + p_vel * 0.5 * DAMPING);
}

pub fn point_shape_collision(point: Vec2, shape: &Shape) -> Option<Collision> {
  let (min, max) = shape.bounding_box;
  if point.x < min.x || point.y < min.y || point.x > max.x || point.y > max.y {
    return None;
  }

  let ray_end = Vec2::new(max.x + 5.0, point.y);
  let np = shape.points.len();
  let mut intersections = 0;

  let mut prev_a = np - 1;

  for i in 0..np {
    let a = shape.points[prev_a].position;
    let b = shape.points[i].position;

    if (a.x < point.x || a.x > ray_end.x) && (b.x < point.x || b.x > ray_end.x) {
      prev_a = i;
      continue;
    }

    if (a.x < point.x || a.x > ray_end.x) || (b.x < point.x || b.x > ray_end.x) {
      prev_a = i;
      if let Some(_) = get_intersection_point((point, ray_end), (a, b)) {
        intersections += 1;
      }
      continue;
    }

    if (a.y >= point.y && b.y <= point.y) || (a.y <= point.y && b.y >= point.y) {
      intersections += 1;
    }

    prev_a = i;
  }

  if intersections % 2 == 0 {
    return None;
  }

  let mut closest_d = INFINITY;
  let mut closest_point = Vec2::ZERO;
  let mut closest_line = (Vec2::ZERO, Vec2::ZERO, 0, 0);
  let mut closest_n = Vec2::ZERO;

  let mut prev_a = np - 1;

  for i in 0..np {
    let a = shape.points[prev_a].position;
    let b = shape.points[i].position;

    let (close_point, d, n) = closest_point_on_line(point, (b, a));

    // draw_circle_vec(close_point, 5.0, BLUE);
    if d < closest_d {
      closest_d = d;
      closest_point = close_point;
      closest_line = (b, a, i, prev_a);
      closest_n = n;
    }

    prev_a = i;
  }
  // draw_circle_vec(closest_point, 5.0, RED);

  let line_t = inverse_lerp_vec(closest_point, closest_line.0, closest_line.1);

  return Some(Collision {
    d: closest_d,
    normal: closest_n,
    t: line_t,
    line: (closest_line.2, closest_line.3),
  });
}

fn get_intersection_point(ray: (Vec2, Vec2), line: (Vec2, Vec2)) -> Option<Vec2> {
  let (a, b) = line;
  let (c, d) = ray;

  let denominator = (d.x - c.x) * (b.y - a.y) - (b.x - a.x) * (d.y - c.y);

  let r = ((b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y)) / denominator;
  if (r + EPS) < 0.0 {
    return None;
  }

  let s = ((a.x - c.x) * (d.y - c.y) - (d.x - c.x) * (a.y - c.y)) / denominator;
  if (s + EPS) < 0.0 || (s - EPS) > 1.0 {
    return None;
  }

  return Some(Vec2 {
    x: s * (b.x - a.x) + a.x,
    y: s * (b.y - a.y) + a.y,
  });
}

pub fn closest_point_on_line(point: Vec2, line: (Vec2, Vec2)) -> (Vec2, f32, Vec2) {
  let dv = (line.1 - line.0).normalize();
  let (dx, dy) = (dv.x, dv.y);

  let n = Vec2::new(-dy, dx);
  let m = dy / dx;
  let b = line.0.y - m * line.0.x;

  let d = (m * point.x - point.y + b) / (n.y - m * n.x);
  let close_point = d * n + point;

  let t = inverse_lerp_vec(close_point, line.0, line.1);

  if t <= 1.0 && t >= 0.0 {
    return (close_point, d.abs(), n);
  }
  if t > 1.0 {
    return (line.1, point.distance(line.1), n);
  }
  return (line.0, point.distance(line.0), n);
}
