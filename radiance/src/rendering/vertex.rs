use crate::math::*;

pub struct Vertex {
    position: Vec3,
    color: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3, color: Vec3) -> Self {
        Vertex { position, color }
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn color(&self) -> Vec3 {
        self.color
    }

    pub fn position_offset() -> usize {
        offset_of!(Vertex, position)
    }

    pub fn color_offset() -> usize {
        offset_of!(Vertex, color)
    }
}
