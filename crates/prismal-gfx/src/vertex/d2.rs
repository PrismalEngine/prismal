use bytemuck::{Pod, Zeroable};
use educe::Educe;
use serde::{Deserialize, Serialize};

use prismal_math::vector::{Vec2, Vec4};

use super::Vertex;

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[derive(Serialize, Deserialize)]
#[derive(Educe)]
#[educe(Default)]
#[repr(C)]
pub struct BasicVertex2d {
    #[educe(Default(expression = "[0.0; 2]"))]
    pub position: [f32; 2],

    #[educe(Default(expression = "[0.0; 2]"))]
    pub tex_coords: [f32; 2],

    #[educe(Default(expression = "[0.0, 0.0, 0.0, 1.0]"))]
    pub color: [f32; 4],
}

impl BasicVertex2d {
    pub const fn new(position: [f32; 2], tex_coords: [f32; 2], color: [f32; 4]) -> Self {
        Self {
            position,
            tex_coords,
            color,
        }
    }
    pub fn from_vectors(position: Vec2, tex_coords: Vec2, color: Vec4) -> Self {
        Self {
            position: position.to_array(),
            tex_coords: tex_coords.to_array(),
            color: color.to_array(),
        }
    }
}

impl Vertex for BasicVertex2d {
    fn vertex_desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        const ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
            0 => Float32x2, 1 => Float32x2, 2 => Float32x4,
        ];
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBS,
        }
    }
}
