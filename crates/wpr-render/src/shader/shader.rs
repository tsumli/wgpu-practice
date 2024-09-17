use anyhow::Result;
use wgpu;

#[allow(unused)]
pub fn create_shader_module_descriptor<'a>(
    label: Option<&'a str>,
    path: &'a std::path::Path,
) -> Result<wgpu::ShaderModuleDescriptor<'a>> {
    let source = std::fs::read_to_string(path).unwrap();
    Ok(wgpu::ShaderModuleDescriptor {
        label: label,
        source: wgpu::ShaderSource::Wgsl(source.into()),
    })
}
