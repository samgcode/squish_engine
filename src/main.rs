use config::GRAVITY;
use macroquad::prelude::*;
use std::f32::consts::TAU;

mod config;
mod engine;

use engine::*;

#[macroquad::main("Squish Engine")]
async fn main() {
  let mut shape_points = Vec::new();

  let n = 6;
  let r = 75.0;
  let mass = 10.0;
  for i in 0..n {
    shape_points.push((
      Vec2::new(
        r * (TAU * (i as f32 - 0.5) / n as f32).cos(),
        r * (TAU * (i as f32 - 0.5) / n as f32).sin(),
      ),
      mass,
    ));
  }

  let platform_points = vec![
    (Vec2::new(72.0, 266.0), mass),
    (Vec2::new(116.0, 329.0), mass),
    (Vec2::new(160.0, 363.0), mass),
    (Vec2::new(201.0, 390.0), mass),
    (Vec2::new(268.0, 431.0), mass),
    (Vec2::new(318.0, 457.0), mass),
    (Vec2::new(377.0, 461.0), mass),
    (Vec2::new(470.0, 478.0), mass),
    (Vec2::new(520.0, 487.0), mass),
    (Vec2::new(603.0, 500.0), mass),
    (Vec2::new(609.0, 558.0), mass),
    (Vec2::new(408.0, 562.0), mass),
    (Vec2::new(236.0, 563.0), mass),
    (Vec2::new(63.0, 546.0), mass),
    (Vec2::new(43.0, 382.0), mass),
    (Vec2::new(40.0, 262.0), mass),
  ];

  let _skrungle_points = vec![
    (Vec2::new(164.0, 479.0), mass),
    (Vec2::new(166.0, 372.0), mass),
    (Vec2::new(166.0, 263.0), mass),
    (Vec2::new(172.0, 187.0), mass),
    (Vec2::new(179.0, 131.0), mass),
    (Vec2::new(239.0, 177.0), mass),
    (Vec2::new(295.0, 180.0), mass),
    (Vec2::new(352.0, 139.0), mass),
    (Vec2::new(356.0, 199.0), mass),
    (Vec2::new(355.0, 251.0), mass),
    (Vec2::new(366.0, 271.0), mass),
    (Vec2::new(400.0, 276.0), mass),
    (Vec2::new(490.0, 284.0), mass),
    (Vec2::new(567.0, 289.0), mass),
    (Vec2::new(620.0, 60.0), mass),
    (Vec2::new(632.0, 289.0), mass),
    (Vec2::new(636.0, 347.0), mass),
    (Vec2::new(636.0, 425.0), mass),
    (Vec2::new(635.0, 475.0), mass),
    (Vec2::new(622.0, 519.0), mass),
    (Vec2::new(606.0, 472.0), mass),
    (Vec2::new(594.0, 424.0), mass),
    (Vec2::new(570.0, 468.0), mass),
    (Vec2::new(554.0, 513.0), mass),
    (Vec2::new(542.0, 459.0), mass),
    (Vec2::new(536.0, 413.0), mass),
    (Vec2::new(475.0, 411.0), mass),
    (Vec2::new(406.0, 404.0), mass),
    (Vec2::new(346.0, 400.0), mass),
    (Vec2::new(336.0, 445.0), mass),
    (Vec2::new(324.0, 504.0), mass),
    (Vec2::new(310.0, 456.0), mass),
    (Vec2::new(299.0, 404.0), mass),
    (Vec2::new(246.0, 397.0), mass),
    (Vec2::new(223.0, 444.0), mass),
    (Vec2::new(190.0, 497.0), mass),
  ];

  let p_mass = 50.0;
  let mut point: PointMass = PointMass::new(Vec2::new(200.0, 200.0), p_mass, false);
  let mut shape = Shape::new(shape_points, (10000.0, 300.0), (3000.0, 0.0), false, 1.0);
  let mut platform = Shape::new(platform_points, (10000.0, 300.0), (3000.0, 0.0), true, 1.0);

  let mut drawing = false;
  let mut drawing_points = Vec::new();

  loop {
    let delta_time = get_frame_time();

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
      if is_key_pressed(KeyCode::A) {
        let mouse_pos: Vec2 = mouse_position().into();
        point = PointMass::new(mouse_pos, p_mass, false);
      }

      point.apply_gravity(GRAVITY * delta_time);

      // shape.update(delta_time);
      platform.update(delta_time);
      point.update(delta_time);

      if let Some(collision) = point_shape_collision(point.position, &platform) {
        resolve_point_line(&mut point, &mut platform, collision, delta_time);
      }

      // shape.draw();
      platform.draw();
      point.draw();
    }

    next_frame().await;
  }
}
