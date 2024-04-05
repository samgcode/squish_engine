use macroquad::prelude::*;
use std::f32::consts::TAU;

mod config;
mod engine;

use engine::*;

#[macroquad::main("Squish Engine")]
async fn main() {
  let mut shape_points = Vec::new();

  let n = 12;
  let r = 150.0;
  let mass = 15.0;
  for i in 0..n {
    shape_points.push((
      Vec2::new(
        r * (TAU * (i as f32 - 0.5) / n as f32).cos(),
        r * (TAU * (i as f32 - 0.5) / n as f32).sin(),
      ),
      mass,
    ));
  }

  let mut shape = Shape::new(
    shape_points,
    Vec2::new(200.0, 200.0),
    (2000.0, 50.0),
    (5000.0, 100.0),
    false,
  );

  loop {
    let delta_time = get_frame_time();

    let mouse_position: Vec2 = mouse_position().into();

    if is_mouse_button_pressed(MouseButton::Left) {}

    shape.update(delta_time);
    shape.draw();

    next_frame().await;
  }
}
