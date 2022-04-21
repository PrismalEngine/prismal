use bytemuck::{Pod, Zeroable};
use educe::Educe;
use serde::{Deserialize, Serialize};

use prismal_math::vector::{Vec2, Vec3, Vec4};

use super::Vertex;

type FlatType = [f32; 15];

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[derive(Serialize, Deserialize)]
#[derive(Educe)]
#[educe(Default)]
#[serde(from = "FlatType")]
#[serde(into = "FlatType")]
#[repr(C)]
pub struct BasicVertex3d {
    #[educe(Default(expression = "[0.0; 3]"))]
    pub position: [f32; 3],

    #[educe(Default(expression = "[0.0; 3]"))]
    pub normal: [f32; 3],

    #[educe(Default(expression = "[0.0; 3]"))]
    pub tangent: [f32; 3],

    #[educe(Default(expression = "[0.0; 2]"))]
    pub tex_coords: [f32; 2],

    #[educe(Default(expression = "[0.0, 0.0, 0.0, 1.0]"))]
    pub color: [f32; 4],
}

impl From<FlatType> for BasicVertex3d {
    fn from(arr: FlatType) -> Self {
        Self {
            position: [arr[0], arr[1], arr[2]],
            normal: [arr[3], arr[4], arr[5]],
            tangent: [arr[6], arr[7], arr[8]],
            tex_coords: [arr[9], arr[10]],
            color: [arr[11], arr[12], arr[13], arr[14]],
        }
    }
}

impl From<BasicVertex3d> for FlatType {
    #[rustfmt::skip]
    fn from(arr: BasicVertex3d) -> Self {
        [
            arr.position[0], arr.position[1], arr.position[2],
            arr.normal[0], arr.normal[1], arr.normal[2],
            arr.tangent[0], arr.tangent[1], arr.tangent[2],
            arr.tex_coords[0], arr.tex_coords[1], 
            arr.color[0], arr.color[1], arr.color[2], arr.color[3],
        ]
    }
}

impl BasicVertex3d {
    pub const fn new(
        position: [f32; 3],
        normal: [f32; 3],
        tangent: [f32; 3],
        tex_coords: [f32; 2],
        color: [f32; 4],
    ) -> Self {
        Self {
            position,
            normal,
            tangent,
            tex_coords,
            color,
        }
    }
    pub fn from_vectors(
        position: Vec3,
        normal: Vec3,
        tangent: Vec3,
        tex_coords: Vec2,
        color: Vec4,
    ) -> Self {
        Self {
            position: position.to_array(),
            normal: normal.to_array(),
            tangent: tangent.to_array(),
            tex_coords: tex_coords.to_array(),
            color: color.to_array(),
        }
    }
}

impl Vertex for BasicVertex3d {
    fn vertex_desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        const ATTRIBS: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![
            0 => Float32x3, 1 => Float32x3, 2 => Float32x3, 3 => Float32x2, 4 => Float32x4,
        ];
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBS,
        }
    }
}
