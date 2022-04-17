use prismal_app_core::traits::AppCore;
use prismal_utils::interior_mut::InteriorMut;
use prismal_utils::shared::*;
use prismal_window::prelude::Window;

#[cfg(target_arch = "wasm")]
type BoxedRenderCallback = (dyn FnMut(&mut wgpu::RenderPass));

#[cfg(not(target_arch = "wasm"))]
type BoxedRenderCallback = (dyn FnMut(&mut wgpu::RenderPass) + Send);

fn default_render_callback(_rp: &mut wgpu::RenderPass) {}

pub struct GfxState {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
    pub surface_config: RefMut<wgpu::SurfaceConfiguration>,
    callback: SyncRc<SyncRefMut<BoxedRenderCallback>>,
}
pub trait DoubleCaptures<'a, 'b: 'a> {}
impl<'a, 'b: 'a, T: ?Sized> DoubleCaptures<'a, 'b> for T {}

impl GfxState {
    pub async fn new<A: AppCore>(app: UnsyncRcMut<A>) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let (adapter, surface, size) = async {
            let app_ref = app.borrow_int_mut().unwrap();
            let window = app_ref.resources().get_unsync::<Window>().unwrap();
            let size = window.inner_size();
            let size = (size.width, size.height);
            let surface = unsafe { instance.create_surface(window) };
            let compatible_surface = Some(&surface);
            (
                instance
                    .request_adapter(&wgpu::RequestAdapterOptions {
                        power_preference: wgpu::PowerPreference::default(),
                        compatible_surface,
                        force_fallback_adapter: false,
                    })
                    .await
                    .unwrap(),
                surface,
                size,
            )
        }
        .await;

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

        Self {
            surface,
            device,
            queue,
            surface_config: RefMut::new(surface_config),
            callback: SyncRc::new(SyncRefMut::new(default_render_callback)),
        }
    }

    pub fn set_render_callback(
        &mut self,
        f: impl for<'r, 's> FnMut(&'r mut wgpu::RenderPass<'s>) + Send + 'static,
    ) {
        self.callback = SyncRc::new(SyncRefMut::new(f));
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
            {
                let mut cb = self.callback.lock();
                (cb)(&mut rp);
            }
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();
        Ok(())
    }
}
