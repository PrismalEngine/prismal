use prismal_app_core::traits::AppCore;
use prismal_utils::interior_mut::InteriorMut;
use prismal_utils::shared::UnsyncRcMut;

use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

#[allow(clippy::let_and_return)]
pub fn initialize_window<A: AppCore>(app: UnsyncRcMut<A>, event_loop: &EventLoop<()>) -> Window {
    let app = app.borrow_int_mut().unwrap();

    let info = app.info();
    let window = WindowBuilder::new()
        .with_title(info.label)
        .with_inner_size(winit::dpi::PhysicalSize::new(800u32, 600u32))
        .build(event_loop)
        .unwrap();
    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("prismal-app")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }
    window
}
