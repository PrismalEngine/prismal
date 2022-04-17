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
    fragment_entry: &'a str,

    vertex_buffers: Option<&'a [wgpu::VertexBufferLayout<'a>]>,
    color_targets: Option<&'a [wgpu::ColorTargetState]>,

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
            fragment_entry: "fs_main",
            vertex_buffers: None,
            color_targets: None,
            topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode: Some(wgpu::Face::Back),
            front_face: wgpu::FrontFace::Ccw,
            polygon_mode: wgpu::PolygonMode::Fill,
            depth_enabled: false,
        }
    }

    pub fn with_label(self, value: Option<&'a str>) -> Self {
        Self {
            label: value,
            ..self
        }
    }
    pub fn with_vertex_entry(self, value: &'a str) -> Self {
        Self {
            vertex_entry: value,
            ..self
        }
    }
    pub fn with_fragment_entry(self, value: &'a str) -> Self {
        Self {
            fragment_entry: value,
            ..self
        }
    }
    pub fn with_vertex_buffers(self, value: &'a [wgpu::VertexBufferLayout<'a>]) -> Self {
        Self {
            vertex_buffers: Some(value),
            ..self
        }
    }
    pub fn with_color_targets(self, value: &'a [wgpu::ColorTargetState]) -> Self {
        Self {
            color_targets: Some(value),
            ..self
        }
    }
    pub fn with_topology(self, value: wgpu::PrimitiveTopology) -> Self {
        Self {
            topology: value,
            ..self
        }
    }
    pub fn with_cull_mode(self, value: wgpu::Face) -> Self {
        Self {
            cull_mode: Some(value),
            ..self
        }
    }
    pub fn with_front_face(self, value: wgpu::FrontFace) -> Self {
        Self {
            front_face: value,
            ..self
        }
    }
    pub fn with_polygon_mode(self, value: wgpu::PolygonMode) -> Self {
        Self {
            polygon_mode: value,
            ..self
        }
    }
    pub fn with_depth_enabled(self, value: bool) -> Self {
        Self {
            depth_enabled: value,
            ..self
        }
    }
    pub fn with_layout(self, value: &'a wgpu::PipelineLayout) -> Self {
        Self {
            layout: Some(value),
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
        let vertex_entry = self.vertex_entry;
        let fragment_entry = self.fragment_entry;

        let shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_source)),
        });

        let vertex_buffers = self.vertex_buffers.unwrap_or(&[]);
        let vertex = wgpu::VertexState {
            module: &shader_module,
            entry_point: vertex_entry,
            buffers: vertex_buffers,
        };

        let color_targets = self.color_targets.unwrap_or(&[]);
        let fragment = Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: fragment_entry,
            targets: color_targets,
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
