use prismal::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

struct SandboxApp {
    resources: Box<AppResources>,
}

impl AppCore for SandboxApp {
    fn start(&mut self) {}

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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    entry::<SandboxApp>().await;
}
