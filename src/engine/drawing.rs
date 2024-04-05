use macroquad::prelude::*;

#[allow(dead_code)]
pub fn draw_line_vec(a: Vec2, b: Vec2, width: f32, color: Color) {
  draw_line(a.x, a.y, b.x, b.y, width, color);
}

#[allow(dead_code)]
pub fn draw_circle_vec(point: Vec2, r: f32, color: Color) {
  draw_circle(point.x, point.y, r, color);
}
