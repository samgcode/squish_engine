use macroquad::prelude::*;

pub const GRAVITY: Vec2 = Vec2::new(0.0, 1000.0);
pub const DRAG_COEFFICIENT: f32 = 0.003;
pub const ZOOM: f32 = 2.0;

pub const DRAW_BOUNDING_BOX: bool = false;
pub const DRAW_SPRINGS: bool = false;
pub const DRAW_POINTS: bool = false;
pub const DRAW_OUTLINE: bool = false;
pub const DRAW_FRAME: bool = false;
pub const DRAW_TRIANGLES: bool = false;
pub const DRAW_TEXTURE: bool = true;
