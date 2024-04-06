use macroquad::prelude::*;

#[allow(dead_code)]
pub fn lerp_f32(t: f32, a: f32, b: f32) -> f32 {
  return (t * (b - a)) + a;
}

#[allow(dead_code)]
pub fn lerp_vec(t: f32, a: Vec2, b: Vec2) -> Vec2 {
  return (t * (b - a)) + a;
}

#[allow(dead_code)]
pub fn inverse_lerp_f32(v: f32, a: f32, b: f32) -> f32 {
  return (v - a) / (b - a);
}

#[allow(dead_code)]
pub fn inverse_lerp_vec(v: Vec2, a: Vec2, b: Vec2) -> f32 {
  return (v.x - a.x) / (b.x - a.x);
}

#[allow(dead_code)]
pub fn cross(a: Vec2, b: Vec2) -> f32 {
  return a.x * b.y - a.y * b.x;
}
