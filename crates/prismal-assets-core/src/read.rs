#[cfg(target_arch = "wasm32")]
fn format_url(file_name: &str) -> reqwest::Url {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let base = reqwest::Url::parse(&format!(
        "{}/{}",
        location.origin().unwrap(),
        option_env!("ASSETS_PATH").unwrap_or("assets")
    ))
    .unwrap();
    base.join(file_name).unwrap()
}

#[cfg(target_arch = "wasm32")]
pub async fn read_asset_file_bytes(path: String) -> Result<Vec<u8>, anyhow::Error> {
    let url = format_url(&path);
    Ok(reqwest::get(url).await?.bytes().await?.to_vec())
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn read_asset_file_bytes(path: String) -> Result<Vec<u8>, anyhow::Error> {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let path = exe_dir.join(path);
    std::fs::read(path).map_err(anyhow::Error::new)
}
