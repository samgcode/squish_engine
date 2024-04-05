use macroquad::prelude::*;

pub struct PointMass {
  pub locked: bool,
  pub mass: f32,
  pub position: Vec2,
  pub velocity: Vec2,
  net_force: Vec2,
}

impl PointMass {
  pub fn new(position: Vec2, mass: f32, locked: bool) -> Self {
    return Self {
      locked,
      mass,
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
