use prismal::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

struct SandboxApp {
    resources: Box<AppResources>,
    pipeline: Option<Rc<wgpu::RenderPipeline>>,
}

impl AppCore for SandboxApp {
    fn start(&mut self) {
        let gfx_state = self.resources.get_mut::<GfxState>().unwrap();
        let pipeline_layout =
            gfx_state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });
        let surface_config = gfx_state.surface_config.borrow_int_mut().unwrap().clone();
        let pipeline = Rc::new(
            RenderPipelineBuilder::new()
                .with_layout(&pipeline_layout)
                .with_shader_source(include_str!("assets/triangle.wgsl"))
                .push_color_target(wgpu::ColorTargetState {
                    blend: Some(wgpu::BlendState::REPLACE),
                    format: surface_config.format,
                    write_mask: wgpu::ColorWrites::ALL,
                })
                .build(&gfx_state.device)
                .unwrap(),
        );
        self.pipeline = Some(pipeline.clone());
        gfx_state.set_render_callback(move |rp| {
            let pipeline = pipeline.clone();
            let pipeline = Rc::as_ptr(&pipeline);
            rp.set_pipeline(unsafe { &*pipeline });
            rp.draw(0..6, 0..1);
        });
    }

    fn info<'i>(&self) -> AppInfo<'i> {
        AppInfo {
            label: "Sandbox App",
            publisher_label: "Prismal Engine",
            version: "0.1.0",
        }
    }

    fn resources(&self) -> &AppResources {
        self.resources.as_ref()
    }

    fn resources_mut(&mut self) -> &mut AppResources {
        self.resources.as_mut()
    }
}

impl AppFactory for SandboxApp {
    fn make_app() -> UnsyncRcMut<Self> {
        unsync_rc_mut(Self {
            resources: AppResources::new(),
            pipeline: None,
        })
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    entry::<SandboxApp>().await;
}
