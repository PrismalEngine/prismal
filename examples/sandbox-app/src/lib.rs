mod sandbox_ecs;
use sandbox_ecs::SysPrintFps;

use prismal::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

struct SandboxApp {
    resources: Box<AppResources>,
}

#[rustfmt::skip]
const VERTICES: &[BasicVertex2d] = &[
    BasicVertex2d::new([0.0,   0.5], [0.5, 0.0],[1.0, 0.0, 0.0, 1.0]),
    BasicVertex2d::new([-0.5, -0.5], [0.0, 1.0],[1.0, 1.0, 0.0, 1.0]),
    BasicVertex2d::new([0.5,  -0.5], [1.0, 1.0],[0.0, 0.0, 1.0, 1.0]),
];

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
                .push_vertex_buffer_layout::<BasicVertex2d>()
                .push_color_target(wgpu::ColorTargetState {
                    blend: Some(wgpu::BlendState::REPLACE),
                    format: surface_config.format,
                    write_mask: wgpu::ColorWrites::ALL,
                })
                .build(&gfx_state.device)
                .unwrap(),
        );

        let vertex_buffer = Rc::new(SimpleBuffer::from_bytes(
            &gfx_state.device,
            bytemuck::cast_slice(VERTICES),
            wgpu::BufferUsages::VERTEX,
        ));
        gfx_state.set_render_callback(move |rp| {
            let pipeline = pipeline.clone();
            let pipeline = Rc::as_ptr(&pipeline);

            let vertex_buffer = vertex_buffer.clone();
            let vertex_buffer = Rc::as_ptr(&vertex_buffer);

            rp.set_pipeline(unsafe { &*pipeline });
            rp.set_vertex_buffer(0, unsafe { (*vertex_buffer).buffer.slice(..) });
            rp.draw(0..(VERTICES.len() as u32), 0..1);
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
        })
    }
}

impl AppEcs for SandboxApp {
    fn ecs_initializers() -> Vec<Box<dyn EcsInitializer>> {
        struct SandboxEcsInitializer;
        impl EcsInitializer for SandboxEcsInitializer {
            fn setup_tick_dispatcher<'a, 'b>(
                &self,
                builder: DispatcherBuilder<'a, 'b>,
            ) -> DispatcherBuilder<'a, 'b> {
                builder.with(SysPrintFps, "app_sys_print_fps", &[])
            }

            fn setup_world(&self, _world: &mut World) {}
        }

        vec![Box::new(SandboxEcsInitializer)]
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    entry::<SandboxApp>().await;
}
