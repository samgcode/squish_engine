use std::f32::INFINITY;

use macroquad::prelude::*;

use crate::config::GRAVITY;

use super::*;

pub struct Shape {
  bounding_box: (Vec2, Vec2),
  lock_frame: bool,
  position: Vec2,
  rotation: f32,
  frame: Vec<Vec2>,
  points: Vec<PointMass>,
  springs: Vec<Spring>,
}

impl Shape {
  pub fn new(
    input_points: Vec<(Vec2, f32)>,
    position: Vec2,
    body_springs: (f32, f32),
    frame_springs: (f32, f32),
    lock_frame: bool,
  ) -> Self {
    let mut min = Vec2::new(INFINITY, INFINITY);
    let mut max = Vec2::ZERO;

    let mut points = Vec::new();
    let mut frame = Vec::new();
    let mut springs = Vec::new();

    input_points.iter().for_each(|point| {
      min = min.min(point.0);
      max = max.max(point.0);

      points.push(PointMass::new(point.0 + position, point.1, false));
      frame.push(point.0);
    });

    springs.push(Spring::new(
      body_springs.0,
      (points[points.len() - 1].position - points[0].position).length(),
      body_springs.1,
      points.len() - 1,
      0,
    ));
    for i in 1..points.len() {
      springs.push(Spring::new(
        body_springs.0,
        (points[i - 1].position - points[i].position).length(),
        body_springs.1,
        i,
        i - 1,
      ));
    }

    points[0].locked = true;

    return Self {
      bounding_box: (min, max),
      lock_frame,
      position,
      rotation: 0.0,
      frame,
      points,
      springs,
    };
  }

  pub fn update(&mut self, delta_time: f32) {
    self.points[0].position = mouse_position().into();

    for spring in self.springs.iter() {
      let force = spring.calculate_force(&self.points[spring.a], &self.points[spring.b]);
      self.points[spring.a].apply_force(force);
      self.points[spring.b].apply_force(-force);
    }

    self.points.iter_mut().for_each(|point| {
      point.apply_gravity(GRAVITY);

      point.update(delta_time);
    });
  }

  pub fn draw(&self) {
    self.points.iter().for_each(|point| {
      point.draw();
    });
  }
}
