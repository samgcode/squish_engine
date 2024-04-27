use std::f32::INFINITY;

use macroquad::prelude::*;

use crate::config::*;

use super::{cross_2d, PointMass};

pub struct SoftMesh {
  tex_coords: Vec<(f32, f32)>,
  indices: Vec<u16>,
  material: Material,
}

impl SoftMesh {
  pub fn generate(shape: Vec<Vec2>) -> Self {
    let tex_coords = generate_uv(&shape);

    let indices = triangulate(shape)
      .unwrap()
      .iter()
      .map(|i| *i as u16)
      .collect();

    let material = load_material(
      ShaderSource::Glsl {
        vertex: include_str!("../vert.glsl"),
        fragment: include_str!("../frag.glsl"),
      },
      MaterialParams {
        textures: vec!["tex".to_owned()],
        ..Default::default()
      },
    )
    .unwrap();

    return Self {
      tex_coords,
      indices,
      material,
    };
  }

  pub fn set_texture(&mut self, texture: Texture2D) {
    self.material.set_texture("tex", texture);
  }

  pub fn update_triangles(&mut self, points: &Vec<Vec2>) {
    if let Some(triangles) = triangulate(points.clone()) {
      self.indices = triangles.iter().map(|i| *i as u16).collect();
    }
  }

  pub fn draw(&self, points: &Vec<PointMass>) {
    if DRAW_TEXTURE {
      gl_use_material(&self.material);
      let mut vertices = Vec::new();

      for (i, v) in points.iter().enumerate() {
        vertices.push(Vertex::new(
          v.position.x,
          v.position.y,
          0.0,
          self.tex_coords[i].0,
          self.tex_coords[i].1,
          RED,
        ));
      }

      let context = unsafe { get_internal_gl() };
      context.quad_gl.draw_mode(DrawMode::Triangles);
      context.quad_gl.geometry(&vertices, &self.indices);
      gl_use_default_material();
    }

    if DRAW_TRIANGLES {
      for i in 0..self.indices.len() / 3 {
        let a = &points[self.indices[i * 3] as usize];
        let b = &points[self.indices[i * 3 + 1] as usize];
        let c = &points[self.indices[i * 3 + 2] as usize];
        draw_triangle_lines(a.position, b.position, c.position, 3.0, GREEN);
      }
    }
  }
}

fn triangulate(shape: Vec<Vec2>) -> Option<Vec<usize>> {
  let mut indices = (0..shape.len()).collect::<Vec<usize>>();
  let mut triangles = Vec::<usize>::new();

  let mut valid = true;

  while indices.len() > 3 {
    let mut is_ear = false;
    for v in 1..indices.len() {
      let vert = indices[v];
      let (prev, next) = get_adjacent(v, &indices);

      let vp = shape[vert] - shape[prev];
      let vn = shape[vert] - shape[next];

      if cross_2d(vp, vn) > 0.0 {
        continue; // vertex is convex
      }

      is_ear = true;

      for i in 0..shape.len() {
        if i == vert || i == prev || i == next {
          continue;
        }

        if point_in_triangle(shape[i], shape[prev], shape[vert], shape[next]) {
          is_ear = false;
          break;
        }
      }

      if is_ear {
        triangles.push(vert);
        triangles.push(prev);
        triangles.push(next);

        indices.remove(v);

        break;
      }
    }
    if !is_ear {
      valid = false;
      break;
    }
  }
  if valid {
    triangles.push(indices[0]);
    triangles.push(indices[1]);
    triangles.push(indices[2]);

    return Some(triangles);
  }

  return None;
}

fn get_adjacent(i: usize, list: &Vec<usize>) -> (usize, usize) {
  if i == 0 {
    return (*list.last().unwrap(), list[i + 1]);
  }
  if i == list.len() - 1 {
    return (list[i - 1], list[0]);
  }

  return (list[i - 1], list[i + 1]);
}

fn point_in_triangle(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
  let ab = b - a;
  let bc = c - b;
  let ca = a - c;

  let ap = p - a;
  let bp = p - b;
  let cp = p - c;

  if cross_2d(ab, ap) < 0.0 {
    return false;
  }
  if cross_2d(bc, bp) < 0.0 {
    return false;
  }
  if cross_2d(ca, cp) < 0.0 {
    return false;
  }

  return true;
}

fn generate_uv(shape: &Vec<Vec2>) -> Vec<(f32, f32)> {
  let mut min = Vec2::new(INFINITY, INFINITY);
  let mut max = Vec2::new(0.0, 0.0);

  for v in shape.clone() {
    min = min.min(v);
    max = max.max(v);
  }

  let range = max - min;
  let width = range.x;
  let height = range.y;

  let mut uv = Vec::<(f32, f32)>::new();

  for v in shape {
    let u = (v.x - min.x) / width;
    let v = (v.y - min.y) / height;

    uv.push((u, v));
  }

  return uv;
}
