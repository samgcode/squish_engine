use config::GRAVITY;
use macroquad::prelude::*;
use std::f32::consts::TAU;

mod config;
mod engine;
mod object_data;

use engine::*;

#[macroquad::main("Squish Engine")]
async fn main() {
  let mut shape_points = Vec::new();

  let n = 10;
  let r = 50.0;
  let mass = 1.0;
  for i in 0..n {
    shape_points.push((
      Vec2::new(
        r * (TAU * (i as f32 - 0.5) / n as f32).cos(),
        r * (TAU * (i as f32 - 0.5) / n as f32).sin(),
      ),
      mass,
    ));
  }

  let platform_points: Vec<(Vec2, f32)> = object_data::PLATFORM_POINTS.into();
  let shape_points: Vec<(Vec2, f32)> = object_data::_SKRUNGLE_POINTS.into();

  let p_mass = 1.0;
  let mut point = PointMass::new(Vec2::new(200.0, 200.0), p_mass, false);
  let mut shape = Shape::new(shape_points, (500.0, 30.0), (1500.0, 0.0), false, 0.15);
  let mut platform = Shape::new(platform_points, (800.0, 30.0), (1000.0, 10.0), true, 1.0);

  // let texture =
  //   Texture2D::from_file_with_format(include_bytes!("../texture.png"), Some(ImageFormat::Png));
  let texture = load_texture("src/texture.png").await.unwrap();
  shape.set_texture(texture.clone());

  let mut drawing = false;
  let mut drawing_points = Vec::new();

  let mut direction = Vec2::ZERO;

  loop {
    // let delta_time = get_frame_time();
    let delta_time = 0.007;

    if is_key_pressed(KeyCode::D) {
      drawing = !drawing;
    }

    if drawing {
      if is_key_pressed(KeyCode::Backspace) {
        drawing_points.pop();
      }
      if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_position: Vec2 = mouse_position().into();
        drawing_points.push(Vec2::new(mouse_position.x, mouse_position.y));

        println!("let shape_points = vec![");
        for point in drawing_points.iter() {
          println!("\t(Vec2::new({}.0, {}.0), mass),", point.x, point.y);
        }
        println!("];");
      }

      for point in drawing_points.iter() {
        draw_circle_vec(*point, 4.0, WHITE);
      }
    } else {
      if is_key_down(KeyCode::A) {
        let mouse_pos: Vec2 = mouse_position().into();
        point = PointMass::new(mouse_pos, p_mass, false);
      }

      let mut new_dir = Vec2::ZERO;
      if is_key_down(KeyCode::Up) {
        new_dir.y = -1.0;
      } else if is_key_down(KeyCode::Down) {
        new_dir.y = 1.0;
      }
      if is_key_down(KeyCode::Right) {
        new_dir.x = 1.0;
      } else if is_key_down(KeyCode::Left) {
        new_dir.x = -1.0;
      }

      if new_dir.length() > 0.0 {
        direction = new_dir;
      }

      draw_line_vec(shape.position, shape.position + direction * 25.0, 3.0, BLUE);
      if is_key_pressed(KeyCode::X) {
        shape.set_velocity(Vec2::ZERO);
        shape.apply_force(direction * 1500.0);
      }

      point.apply_gravity(GRAVITY * delta_time);

      shape.update(delta_time);
      platform.update(delta_time);
      point.update(delta_time);

      shape_shape_collision(&mut shape, &mut platform);
      if let Some(collision) = point_shape_collision(point.position, &platform) {
        resolve_point_line(&mut point, &mut platform, collision);
      }

      next_frame().await;
      shape.draw();
      platform.draw();
      point.draw();
    }
  }
}
