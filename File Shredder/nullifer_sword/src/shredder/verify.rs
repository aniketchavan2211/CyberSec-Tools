use anyhow::Result;
use std::path::Path;
use crate::utils::*;

pub fn run(target: &str) -> Result<()> {
    let path = Path::new(target);

    if !path.exists() {
        success("File not found → destroyed");
    } else {
        error("File still exists → not fully removed");
    }

    Ok(())
}
