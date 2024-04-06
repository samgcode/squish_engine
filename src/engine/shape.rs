use std::f32::INFINITY;

use macroquad::prelude::*;

use crate::config::*;

use super::*;

pub struct Shape {
  pub bounding_box: (Vec2, Vec2),
  lock_frame: bool,
  position: Vec2,
  rotation: f32,
  frame: Vec<Vec2>,
  pub points: Vec<PointMass>,
  frame_points: Vec<PointMass>,
  np: usize,
  springs: Vec<Spring>,
  frame_springs: Vec<Spring>,
}

impl Shape {
  pub fn new(
    input_points: Vec<(Vec2, f32)>,
    body_strength: (f32, f32),
    frame_strength: (f32, f32),
    lock_frame: bool,
    scale: f32,
  ) -> Self {
    let mut min = Vec2::new(INFINITY, INFINITY);
    let mut max = Vec2::ZERO;
    let mut total_position = Vec2::ZERO;

    let mut points = Vec::new();
    let mut frame_points = Vec::new();
    let mut frame = Vec::new();

    let mut springs = Vec::new();
    let mut frame_springs = Vec::new();

    let np = input_points.len();

    input_points.iter().for_each(|point| {
      min = min.min(point.0);
      max = max.max(point.0);

      total_position += point.0;
    });

    let position = total_position / np as f32;

    input_points.iter().for_each(|point| {
      let r = point.0 - position;
      points.push(PointMass::new(r * scale + position, point.1, false));
      frame_points.push(PointMass::new(r * scale, 0.0, false));
      frame.push(r * scale);
    });

    add_springs(&mut springs, 1, body_strength, &points);
    add_springs(&mut springs, 2, body_strength, &points);

    for i in 0..np {
      frame_springs.push(Spring::new(frame_strength.0, 0.0, frame_strength.1, i, i));
    }

    // points[np - 1].locked = true;
    // points[0].apply_force(Vec2::new(10000.0, 10000.0));

    return Self {
      bounding_box: (min, max),
      lock_frame,
      position,
      rotation: 0.0,
      frame,
      points,
      frame_points,
      np,
      springs,
      frame_springs,
    };
  }

  pub fn update(&mut self, delta_time: f32) {
    if is_mouse_button_down(MouseButton::Left) && !self.lock_frame {
      self.points[self.np - 1].position = mouse_position().into();
      self.points[self.np - 1].velocity = Vec2::ZERO;
    }

    for point in self.points.iter_mut() {
      point.apply_gravity(GRAVITY * delta_time);
      point.update(delta_time);
    }

    for spring in self.springs.iter() {
      let force = spring.calculate_force(&self.points[spring.a], &self.points[spring.b]);

      self.points[spring.a].apply_force(force * delta_time);
      self.points[spring.b].apply_force(-force * delta_time);
    }

    for i in 0..self.np {
      let point = &mut self.points[i];

      if point.velocity.length() != 0.0 {
        let drag = DRAG_COEFFICIENT
          * point.diameter
          * point.velocity.length()
          * point.velocity.length()
          * -point.velocity.normalize();
        point.apply_force(drag * delta_time);
      }
    }

    let mut min = Vec2::new(INFINITY, INFINITY);
    let mut max = Vec2::ZERO;

    let mut total_position = Vec2::ZERO;

    for point in self.points.iter() {
      min = min.min(point.position);
      max = max.max(point.position);

      total_position += point.position;
    }

    self.bounding_box = (min, max);

    if !self.lock_frame {
      self.position = total_position / self.np as f32;

      let mut a = 0.0;
      let mut b = 0.0;
      for (i, v) in self.points.iter().enumerate() {
        let r = v.position - self.position;
        a += r.dot(self.frame[i]);
        b += cross(r, self.frame[i]);
      }
      let angle = -(b.atan2(a));

      self.rotation = angle;
    }

    let angle_c = self.rotation.cos();
    let angle_s = self.rotation.sin();

    for i in 0..self.np {
      let frame_pos = self.frame[i];
      self.frame_points[i].position = Vec2::new(
        angle_c * frame_pos.x - angle_s * frame_pos.y + self.position.x,
        angle_s * frame_pos.x + angle_c * frame_pos.y + self.position.y,
      );
    }

    for spring in self.frame_springs.iter() {
      let a = &mut self.points[spring.a];
      let force = spring.calculate_force(a, &self.frame_points[spring.b]);
      a.velocity += force / a.mass * delta_time;
    }
  }

  pub fn draw(&self) {
    self.springs.iter().for_each(|spring| {
      spring.draw(&self.points[spring.a], &self.points[spring.b]);
    });

    self.frame_springs.iter().for_each(|spring| {
      spring.draw(&self.points[spring.a], &self.frame_points[spring.b]);
    });

    // self.points.iter().for_each(|point| {
    //   point.draw();
    // });

    draw_line_vec(
      self.points[self.np - 1].position,
      self.points[0].position,
      2.0,
      WHITE,
    );
    for i in 1..self.np {
      draw_line_vec(
        self.points[i - 1].position,
        self.points[i].position,
        2.0,
        WHITE,
      );
    }

    // self.frame_points.iter().for_each(|point| {
    //   draw_circle_vec(point.position, 4.0, GRAY);
    // });
    // draw_circle_vec(self.position, 5.0, RED);

    draw_rectangle_lines(
      self.bounding_box.0.x,
      self.bounding_box.0.y,
      self.bounding_box.1.x - self.bounding_box.0.x,
      self.bounding_box.1.y - self.bounding_box.0.y,
      2.0,
      WHITE,
    );
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
