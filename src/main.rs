use config::GRAVITY;
use macroquad::prelude::*;
use std::f32::consts::TAU;

mod config;
mod engine;

use engine::*;

#[macroquad::main("Squish Engine")]
async fn main() {
  let mut shape_points = Vec::new();

  let n = 4;
  let r = 50.0;
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
    (Vec2::new(96.0, 53.0), mass),
    (Vec2::new(151.0, 53.0), mass),
    (Vec2::new(198.0, 63.0), mass),
    (Vec2::new(209.0, 104.0), mass),
    (Vec2::new(209.0, 149.0), mass),
    (Vec2::new(204.0, 195.0), mass),
    (Vec2::new(208.0, 216.0), mass),
    (Vec2::new(245.0, 219.0), mass),
    (Vec2::new(300.0, 228.0), mass),
    (Vec2::new(351.0, 256.0), mass),
    (Vec2::new(340.0, 282.0), mass),
    (Vec2::new(288.0, 288.0), mass),
    (Vec2::new(237.0, 312.0), mass),
    (Vec2::new(207.0, 344.0), mass),
    (Vec2::new(206.0, 384.0), mass),
    (Vec2::new(215.0, 408.0), mass),
    (Vec2::new(233.0, 440.0), mass),
    (Vec2::new(269.0, 455.0), mass),
    (Vec2::new(313.0, 462.0), mass),
    (Vec2::new(448.0, 473.0), mass),
    (Vec2::new(503.0, 469.0), mass),
    (Vec2::new(546.0, 448.0), mass),
    (Vec2::new(555.0, 413.0), mass),
    (Vec2::new(521.0, 386.0), mass),
    (Vec2::new(459.0, 382.0), mass),
    (Vec2::new(426.0, 361.0), mass),
    (Vec2::new(474.0, 331.0), mass),
    (Vec2::new(540.0, 328.0), mass),
    (Vec2::new(570.0, 305.0), mass),
    (Vec2::new(573.0, 248.0), mass),
    (Vec2::new(529.0, 217.0), mass),
    (Vec2::new(483.0, 189.0), mass),
    (Vec2::new(449.0, 158.0), mass),
    (Vec2::new(435.0, 118.0), mass),
    (Vec2::new(444.0, 80.0), mass),
    (Vec2::new(505.0, 47.0), mass),
    (Vec2::new(570.0, 41.0), mass),
    (Vec2::new(642.0, 77.0), mass),
    (Vec2::new(676.0, 156.0), mass),
    (Vec2::new(675.0, 240.0), mass),
    (Vec2::new(663.0, 353.0), mass),
    (Vec2::new(639.0, 425.0), mass),
    (Vec2::new(608.0, 499.0), mass),
    (Vec2::new(523.0, 551.0), mass),
    (Vec2::new(401.0, 571.0), mass),
    (Vec2::new(288.0, 563.0), mass),
    (Vec2::new(192.0, 534.0), mass),
    (Vec2::new(126.0, 471.0), mass),
    (Vec2::new(91.0, 382.0), mass),
    (Vec2::new(63.0, 270.0), mass),
    (Vec2::new(45.0, 153.0), mass),
    (Vec2::new(56.0, 82.0), mass),
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
  let mut point = PointMass::new(Vec2::new(200.0, 200.0), p_mass, false);
  let mut shape = Shape::new(shape_points, (5000.0, 150.0), (15000.0, 0.0), false, 0.5);
  let mut platform = Shape::new(
    platform_points,
    (20000.0, 300.0),
    (7000.0, 100.0),
    true,
    1.0,
  );

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
      if is_key_down(KeyCode::A) {
        let mouse_pos: Vec2 = mouse_position().into();
        point = PointMass::new(mouse_pos, p_mass, false);
      }

      point.apply_gravity(GRAVITY * delta_time);

      shape.update(delta_time);
      platform.update(delta_time);
      point.update(delta_time);

      shape_shape_collision(&mut shape, &mut platform, delta_time);
      if let Some(collision) = point_shape_collision(point.position, &platform) {
        resolve_point_line(&mut point, &mut platform, collision, delta_time);
      }

      next_frame().await;
      shape.draw();
      platform.draw();
      point.draw();
    }
  }
}
