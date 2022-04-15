use std::borrow::Cow;

use prismal_app_core::traits::AppCore;
use prismal_utils::{
    interior_mut::InteriorMut,
    shared::{RefMut, UnsyncRcMut},
};
use prismal_window::prelude::Window;

use crate::pipeline::{PipelineBuilder, PipelineBuilderError};

pub fn make_triangle_pipeline<D>(
    device: D,
    surface_format: wgpu::TextureFormat,
) -> Result<wgpu::RenderPipeline, PipelineBuilderError>
where
    D: std::ops::Deref<Target = wgpu::Device>,
{
    let wgsl = include_str!("../../assets/triangle.wgsl");

    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(wgsl)),
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });
    PipelineBuilder::default()
        .with_layout(&layout)
        .with_vertex_module(&shader, "vs_main")
        .with_fragment_module(&shader, "fs_main")
        .with_color_targets(&[wgpu::ColorTargetState {
            format: surface_format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        }])
        .build(device)
}

pub struct GfxState {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
    pub surface_config: RefMut<wgpu::SurfaceConfiguration>,

    triangle_pipeline: wgpu::RenderPipeline,
}

impl GfxState {
    pub async fn new<A: AppCore>(app: UnsyncRcMut<A>) -> Self {
        let app = app.borrow_int_mut().unwrap();
        let window = app.resources().get_unsync::<Window>().unwrap();
        let size = window.inner_size();
        let size = (size.width, size.height);

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                None,
            )
            .await
            .unwrap();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);
        let triangle_pipeline = make_triangle_pipeline(&device, surface_config.format).unwrap();

        Self {
            surface,
            device,
            queue,
            surface_config: RefMut::new(surface_config),
            triangle_pipeline,
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        let mut surface_config = self.surface_config.borrow_int_mut().unwrap();
        surface_config.width = width;
        surface_config.height = height;
        self.surface.configure(&self.device, &surface_config);
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            rp.set_pipeline(&self.triangle_pipeline);
            rp.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();
        Ok(())
    }
}
