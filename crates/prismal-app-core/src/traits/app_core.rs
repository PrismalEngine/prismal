use crate::info::AppInfo;
use crate::resources::AppResources;

pub trait AppCore {
    fn start(&mut self);

    fn preload_asset_paths(&self) -> Vec<String> {
        vec![]
    }

    fn info<'i>(&self) -> AppInfo<'i>;

    fn resources(&self) -> &AppResources;
    fn resources_mut(&mut self) -> &mut AppResources;
}
