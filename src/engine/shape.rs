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
  np: usize,
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

    let np = input_points.len();

    input_points.iter().for_each(|point| {
      min = min.min(point.0);
      max = max.max(point.0);

      points.push(PointMass::new(point.0 + position, point.1, false));
      frame.push(point.0);
    });

    add_springs(&mut springs, 1, body_springs, &points);
    add_springs(&mut springs, 2, body_springs, &points);

    points[np - 1].locked = true;

    return Self {
      bounding_box: (min, max),
      lock_frame,
      position,
      rotation: 0.0,
      frame,
      points,
      np,
      springs,
    };
  }

  pub fn update(&mut self, delta_time: f32) {
    self.points[self.np - 1].position = mouse_position().into();

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
    self.springs.iter().for_each(|spring| {
      spring.draw(&self.points[spring.a], &self.points[spring.b]);
    });

    self.points.iter().for_each(|point| {
      point.draw();
    });
  }
}

fn add_springs(
  springs: &mut Vec<Spring>,
  spacing: usize,
  values: (f32, f32),
  points: &Vec<PointMass>,
) {
  let np = points.len();

  for i in 1..spacing + 1 {
    springs.push(Spring::new(
      values.0,
      points[np - i]
        .position
        .distance(points[spacing - i].position),
      values.1,
      np - i,
      spacing - i,
    ));
  }

  for i in spacing..np {
    springs.push(Spring::new(
      values.0,
      points[i - spacing].position.distance(points[i].position),
      values.1,
      i,
      i - spacing,
    ));
  }
}
