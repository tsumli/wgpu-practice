use anyhow::Result;
use wpr_utils::get_project_root;

pub fn main() -> Result<()> {
    let project_root = get_project_root()?;
    let asset_root = project_root.join("asset");
    if !asset_root.exists() {
        std::fs::create_dir_all(&asset_root)?;
    }

    {
        let remote_path = "https://github.com/KhronosGroup/glTF-Sample-Assets";
        let local_path = asset_root.join("glTF-Sample-Assets");
        if !local_path.exists() {
            git2::Repository::clone_recurse(remote_path, local_path)?;
        }
    }

    Ok(())
}
