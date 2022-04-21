use anyhow::*;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::env;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    // This tells cargo to rerun this script if something in /res/ changes.
    println!("cargo:rerun-if-changed=assets/*");

    let out_dir = get_output_path()?;
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let paths_to_copy = vec!["assets/"];
    copy_items(&paths_to_copy, out_dir, &copy_options)?;
    Ok(())
}
fn get_target_dir() -> Result<PathBuf> {
    let out_dir = PathBuf::from(
        env::var("OUT_DIR")
            .context("`OUT_DIR` environment variable is required, but was not set")?,
    );
    for dir in out_dir.ancestors() {
        let fname = dir.file_name();
        if let Some(fname) = fname {
            if fname == "target" {
                return Ok(dir.to_path_buf());
            }
        }
    }
    Err(anyhow::anyhow!(
        "Failed to find `target` directory in `OUT_DIR` value"
    ))
}
fn get_output_path() -> Result<PathBuf> {
    let target_dir = get_target_dir()?;
    let build_type = env::var("PROFILE")
        .context("`PROFILE` environment variable is required, but was not set")?;
    let path = Path::new(&target_dir).join(build_type);
    Ok(path)
}
