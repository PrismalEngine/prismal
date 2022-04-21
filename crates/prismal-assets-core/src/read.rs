use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadAssetFileError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(target_arch = "wasm32")]
pub fn read_asset_file_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, ReadAssetFileError> {
    todo!("Get assets for WASM via HTTP")
}

#[cfg(not(target_arch = "wasm32"))]
pub fn read_asset_file_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, ReadAssetFileError> {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let path = exe_dir.join(path);
    std::fs::read(path).map_err(ReadAssetFileError::Io)
}
