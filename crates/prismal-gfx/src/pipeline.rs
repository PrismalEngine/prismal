use std::borrow::Cow;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PipelineBuilderError {
    #[error("PipelineBuilder is missing required field: `{0}`")]
    MissingField(String),
}

#[derive(Debug, Clone)]
pub struct RenderPipelineBuilder<'a> {
    label: Option<&'a str>,

    shader_source: Option<&'a str>,
    vertex_entry: &'a str,
    fragment_entry: Option<&'a str>,

    vertex_buffers: Vec<wgpu::VertexBufferLayout<'a>>,
    color_targets: Vec<wgpu::ColorTargetState>,

    topology: wgpu::PrimitiveTopology,
    cull_mode: Option<wgpu::Face>,
    front_face: wgpu::FrontFace,
    polygon_mode: wgpu::PolygonMode,

    layout: Option<&'a wgpu::PipelineLayout>,

    depth_enabled: bool,
}

impl<'a> Default for RenderPipelineBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> RenderPipelineBuilder<'a> {
    pub fn new() -> Self {
        Self {
            label: None,
            layout: None,
            shader_source: None,
            vertex_entry: "vs_main",
            fragment_entry: Some("fs_main"),
            vertex_buffers: Vec::new(),
            color_targets: Vec::new(),
            topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode: Some(wgpu::Face::Back),
            front_face: wgpu::FrontFace::Ccw,
            polygon_mode: wgpu::PolygonMode::Fill,
            depth_enabled: false,
        }
    }

    pub fn with_label(self, label: Option<&'a str>) -> Self {
        Self { label, ..self }
    }

    pub fn with_shader_source(self, shader_source: &'a str) -> Self {
        Self {
            shader_source: Some(shader_source),
            ..self
        }
    }
    pub fn with_vertex_entry_fn(self, entry_name: &'a str) -> Self {
        Self {
            vertex_entry: entry_name,
            ..self
        }
    }
    pub fn with_fragment_entry_fn(self, entry_name: Option<&'a str>) -> Self {
        Self {
            fragment_entry: entry_name,
            ..self
        }
    }
    pub fn push_vertex_buffer_layout(
        mut self,
        buffer_layout: wgpu::VertexBufferLayout<'a>,
    ) -> Self {
        self.vertex_buffers.push(buffer_layout);
        self
    }

    pub fn push_color_target(mut self, target: wgpu::ColorTargetState) -> Self {
        self.color_targets.push(target);
        self
    }
    pub fn with_topology(self, topology: wgpu::PrimitiveTopology) -> Self {
        Self { topology, ..self }
    }
    pub fn with_cull_mode(self, cull_mode: Option<wgpu::Face>) -> Self {
        Self { cull_mode, ..self }
    }
    pub fn with_front_face(self, front_face: wgpu::FrontFace) -> Self {
        Self { front_face, ..self }
    }
    pub fn with_polygon_mode(self, polygon_mode: wgpu::PolygonMode) -> Self {
        Self {
            polygon_mode,
            ..self
        }
    }
    pub fn with_depth_enabled(self, depth_enabled: bool) -> Self {
        Self {
            depth_enabled,
            ..self
        }
    }
    pub fn with_layout(self, layout: &'a wgpu::PipelineLayout) -> Self {
        Self {
            layout: Some(layout),
            ..self
        }
    }

    pub fn build<D>(self, device: D) -> Result<wgpu::RenderPipeline, PipelineBuilderError>
    where
        D: std::ops::Deref<Target = wgpu::Device>,
    {
        let label = self.label;
        let layout = Some(
            self.layout
                .ok_or_else(|| PipelineBuilderError::MissingField("layout".into()))?,
        );

        let shader_source = self
            .shader_source
            .ok_or_else(|| PipelineBuilderError::MissingField("shader_source".into()))?;

        let shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_source)),
        });

        let vertex_buffers = &self.vertex_buffers[..];
        let vertex = wgpu::VertexState {
            module: &shader_module,
            entry_point: self.vertex_entry,
            buffers: vertex_buffers,
        };

        let fragment = self
            .fragment_entry
            .map(|fragment_entry| wgpu::FragmentState {
                module: &shader_module,
                entry_point: fragment_entry,
                targets: &self.color_targets[..],
            });

        let primitive = wgpu::PrimitiveState {
            topology: self.topology,
            strip_index_format: None,
            front_face: self.front_face,
            cull_mode: self.cull_mode,
            unclipped_depth: false,
            polygon_mode: self.polygon_mode,
            conservative: false,
        };
        let depth_stencil = if self.depth_enabled {
            Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            })
        } else {
            None
        };

        let multisample = wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };

        Ok(
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label,
                layout,
                vertex,
                primitive,
                depth_stencil,
                multisample,
                fragment,
                multiview: None,
            }),
        )
    }
}
