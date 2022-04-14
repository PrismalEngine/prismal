#[derive(Debug, Clone)]
pub struct AppInfo<'info> {
    pub label: &'info str,
    pub publisher_label: &'info str,
    pub version: &'info str,
}
