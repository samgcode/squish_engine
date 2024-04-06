use std::f32::INFINITY;

use macroquad::prelude::*;

use super::{math::*, Shape, *};

const EPS: f32 = 0.00001;
const ELASTICITY: f32 = 0.8;
const FRICTION_COEFFICIENT: f32 = 0.2;

pub struct Collision {
  d: f32,
  normal: Vec2,
  t: f32,
  point: Vec2,
  line: (usize, usize),
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
  let l_mass = point_a.mass + point_b.mass;

  let p_mass_p = point.mass / (l_mass + point.mass);
  let l_mass_p = 1.0 - p_mass_p;

  let dist = collision.d * collision.normal;

  let p_dist = l_mass_p * dist;
  let a_dist = p_mass_p * (1.0 - collision.t) * 2.0 * -dist;
  let b_dist = p_mass_p * collision.t * 2.0 * -dist;

  let p = 2.0 * (point.velocity.dot(collision.normal) - l_vel.dot(collision.normal))
    / (point.mass + l_mass);

  let p_vel_f = (point.velocity - p * l_mass * collision.normal) * ELASTICITY;
  let l_vel_f = (l_vel + p * point.mass * collision.normal) * ELASTICITY;

  let perpendicular_n = point_a.position - collision.point;
  let friction_force = if perpendicular_n.length() != 0.0 {
    let perpendicular_n = perpendicular_n.normalize();
    perpendicular_n * perpendicular_n.dot(p_vel_f) * FRICTION_COEFFICIENT
  } else {
    Vec2::ZERO
  };

  let p_vel_f = p_vel_f - friction_force;

  point.add_position(p_dist);
  point.set_velocity(p_vel_f);
  point.update(delta_time);

  let particle = &mut shape.points[collision.line.0];
  particle.add_position(a_dist);
  particle.set_velocity((1.0 - collision.t) * l_vel_f);
  particle.update(delta_time);

  let particle = &mut shape.points[collision.line.1];
  particle.add_position(b_dist);
  particle.set_velocity(collision.t * l_vel_f);
  particle.update(delta_time);
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

  let mut prev_a = np - 1;

  for i in 0..np {
    let a = shape.points[prev_a].position;
    let b = shape.points[i].position;

    let (close_point, d) = closest_point_on_line(point, (b, a));

    if d < closest_d {
      closest_d = d;
      closest_point = close_point;
      closest_line = (b, a, i, prev_a);
    }

    prev_a = i;
  }

  let line_t = inverse_lerp_vec(closest_point, closest_line.0, closest_line.1);

  let n = if closest_d != 0.0 {
    (closest_point - point) / closest_d
  } else {
    Vec2::ZERO
  };

  return Some(Collision {
    d: closest_d,
    normal: n,
    t: line_t,
    point: closest_point,
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

pub fn closest_point_on_line(point: Vec2, line: (Vec2, Vec2)) -> (Vec2, f32) {
  let dv = Vec2::new(line.1.x - line.0.x, line.1.y - line.0.y).normalize();
  let (dx, dy) = (dv.x, dv.y);

  let c = dy * line.0.x - dx * line.0.y;
  let d = (dy * point.x - dx * point.y - c) / (dy * dy + dx * dx).sqrt();

  let close_point = Vec2::new(point.x + -dy * d, point.y + dx * d);

  let max_x = line.0.x.max(line.1.x);
  let max_y = line.0.y.max(line.1.y);
  let min_x = line.0.x.min(line.1.x);
  let min_y = line.0.y.min(line.1.y);

  if close_point.x > max_x
    || close_point.y > max_y
    || close_point.x < min_x
    || close_point.y < min_y
  {
    let dist_0 = point.distance(line.0);
    let dist_1 = point.distance(line.1);
    if dist_0 < dist_1 {
      return (line.0, dist_0);
    }
    return (line.1, dist_1);
  }

  return (close_point, d);
}
