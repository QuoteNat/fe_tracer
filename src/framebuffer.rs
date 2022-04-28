use crate::vector;

pub struct Framebuffer {
    height: i32,
    width: i32,
    buffer: Vec<vector::Vec3D>,
}

