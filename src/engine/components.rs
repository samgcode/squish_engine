use std::f32::consts::PI;

use macroquad::prelude::*;

use super::inverse_lerp_f32;

pub struct PointMass {
  pub locked: bool,
  pub mass: f32,
  pub diameter: f32,
  pub position: Vec2,
  pub velocity: Vec2,
  pub net_force: Vec2,
}

pub struct Spring {
  strength: f32,
  length: f32,
  damping: f32,
  pub a: usize,
  pub b: usize,
}

impl PointMass {
  pub fn new(position: Vec2, mass: f32, locked: bool) -> Self {
    return Self {
      locked,
      mass,
      diameter: 2.0 * (mass / PI).sqrt(),
      position: position,
      velocity: Vec2::ZERO,
      net_force: Vec2::ZERO,
    };
  }

  pub fn update(&mut self, delta_time: f32) {
    if self.locked {
      return;
    }

    self.velocity += self.net_force / self.mass * delta_time;
    self.position += self.velocity * delta_time;

    self.net_force = Vec2::ZERO;
  }

  pub fn draw(&self) {
    draw_circle(self.position.x, self.position.y, 5.0, WHITE);
  }

  pub fn apply_force(&mut self, force: Vec2) {
    self.net_force += force;
  }

  pub fn apply_gravity(&mut self, force: Vec2) {
    self.net_force += force * self.mass;
  }
}

impl Spring {
  pub fn new(strength: f32, length: f32, damping: f32, a: usize, b: usize) -> Self {
    return Self {
      strength,
      length,
      damping,
      a,
      b,
    };
  }

  pub fn calculate_force(&self, point_a: &PointMass, point_b: &PointMass) -> Vec2 {
    let dist = (point_b.position - point_a.position).length();

    if dist == 0.0 {
      return Vec2::ZERO;
    }

    let dir = (point_b.position - point_a.position).normalize();
    let vel_diff = point_b.velocity - point_a.velocity;

    let damping_force = dir.dot(vel_diff) * self.damping;

    let force = self.strength * (dist - self.length) + damping_force;

    return force * dir;
  }

  pub fn draw(&self, point_a: &PointMass, point_b: &PointMass) {
    let diff = ((point_b.position - point_a.position).length() - self.length).abs();

    let v = inverse_lerp_f32(diff, 100.0, 0.0);
    let v = (v.max(-1.0) * 255.0) as u8;

    draw_line(
      point_a.position.x,
      point_a.position.y,
      point_b.position.x,
      point_b.position.y,
      1.0,
      Color::from_rgba(255, v, 0, 255),
    );
  }
}
