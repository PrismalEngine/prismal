use bytemuck::{Pod, Zeroable};

pub trait Vertex: Copy + Pod + Zeroable {
    fn vertex_desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}
