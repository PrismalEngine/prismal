use thiserror::Error;

#[derive(Debug, Error)]
pub enum PipelineBuilderError {
    #[error("PipelineBuilder is missing required field: `{0}`")]
    MissingField(String),
}

#[derive(Debug, Clone)]
pub struct PipelineBuilder<'a> {
    label: Option<&'a str>,

    vertex_module: Option<(&'a wgpu::ShaderModule, &'a str)>,
    vertex_buffers: Option<&'a [wgpu::VertexBufferLayout<'a>]>,

    fragment_module: Option<(&'a wgpu::ShaderModule, &'a str)>,
    color_targets: Option<&'a [wgpu::ColorTargetState]>,

    topology: wgpu::PrimitiveTopology,
    cull_mode: Option<wgpu::Face>,
    front_face: wgpu::FrontFace,
    polygon_mode: wgpu::PolygonMode,

    layout: Option<&'a wgpu::PipelineLayout>,

    depth_enabled: bool,
}

impl<'a> Default for PipelineBuilder<'a> {
    fn default() -> Self {
        Self {
            label: None,
            layout: None,
            vertex_module: None,
            vertex_buffers: None,
            fragment_module: None,
            color_targets: None,
            topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode: Some(wgpu::Face::Back),
            front_face: wgpu::FrontFace::Ccw,
            polygon_mode: wgpu::PolygonMode::Fill,
            depth_enabled: false,
        }
    }
}

impl<'a> PipelineBuilder<'a> {
    pub fn with_label(self, value: Option<&'a str>) -> Self {
        Self {
            label: value,
            ..self
        }
    }
    pub fn with_vertex_module(self, shader: &'a wgpu::ShaderModule, entry: &'a str) -> Self {
        Self {
            vertex_module: Some((shader, entry)),
            ..self
        }
    }
    pub fn with_vertex_buffers(self, value: &'a [wgpu::VertexBufferLayout<'a>]) -> Self {
        Self {
            vertex_buffers: Some(value),
            ..self
        }
    }
    pub fn with_fragment_module(self, shader: &'a wgpu::ShaderModule, entry: &'a str) -> Self {
        Self {
            fragment_module: Some((shader, entry)),
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
        let vertex_module = self
            .vertex_module
            .ok_or_else(|| PipelineBuilderError::MissingField("vertex_module".into()))?;
        let vertex_buffers = self.vertex_buffers.unwrap_or(&[]);
        let vertex = wgpu::VertexState {
            module: vertex_module.0,
            entry_point: vertex_module.1,
            buffers: vertex_buffers,
        };
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
                format: wgpu::TextureFormat::Depth24Plus,
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

        let fragment_module = self
            .fragment_module
            .ok_or_else(|| PipelineBuilderError::MissingField("fragment_module".into()))?;
        let color_targets = self.color_targets.unwrap_or(&[]);

        let fragment = Some(wgpu::FragmentState {
            module: fragment_module.0,
            entry_point: fragment_module.1,
            targets: color_targets,
        });

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
