use anyhow::Result;
use std::fs::remove_dir;
use std::path::Path;

use crate::utils::*;

pub fn remove(path: &Path) -> Result<()> {
    remove_dir(path)?;
    success(&format!("Removed dir {}", path.display()));
    Ok(())
}
