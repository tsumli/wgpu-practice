use anyhow::{Context, Result};
use git2;

pub fn get_project_root() -> Result<std::path::PathBuf> {
    let repo = git2::Repository::open_from_env()?;
    Ok(repo
        .path()
        .parent()
        .with_context(|| "failed to get parent")?
        .into())
}
