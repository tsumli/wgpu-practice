use anyhow::{ensure, Ok, Result};
use gltf;
use std::path::Path;

pub fn load_gltf(path: &Path) -> Result<gltf::Gltf> {
    log::info!("Loading glTF file: {:?}", path);
    ensure!(path.exists(), "File not found: {:?}", path);

    let gltf = {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        gltf::Gltf::from_reader(reader)?
    };

    for buffer in gltf.buffers() {
        let buffer = match buffer.source() {
            gltf::buffer::Source::Uri(uri) => {
                log::info!("Loading buffer from URI: {:?}", uri);
                let parent_dir = path.parent().unwrap_or(Path::new(""));
                gltf::buffer::Data::from_source(buffer.source(), Some(parent_dir))?
            }
            gltf::buffer::Source::Bin => gltf::buffer::Data::from_source(buffer.source(), None)?,
        };
        dbg!(buffer);
    }

    // let file = std::fs::File::open(path)?;
    // let reader = std::io::BufReader::new(file);
    // gltf::Glb::from_reader(reader)?;
    Ok(gltf)
}

mod tests {
    use super::*;
    use wpr_utils::get_project_root;

    #[test]
    fn test_load_gltf() -> Result<()> {
        let proj_root = get_project_root()?;
        let asset_root = proj_root.join("asset");
        let path = asset_root.join("glTF-Sample-Assets/Models/Box/glTF/Box.gltf");
        let gltf = load_gltf(&path)?;
        println!("{:#?}", gltf);
        Ok(())
    }
}
