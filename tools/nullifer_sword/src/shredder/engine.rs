use anyhow::Result;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::cli::Cli;
use crate::utils::*;
use super::{file, dir};

pub fn run(target: &str, passes: u8, delete: bool, cli: &Cli) -> Result<()> {
    info(&format!("Target received: {}", target));

    let path = Path::new(target);

    // ❌ NOT FOUND
    if !path.exists() {
        error("Target not found. Please check the path again.");
        return Ok(());
    }

    // 📄 FILE
    if path.is_file() {
        info("Detected file target");
        file::shred(path, passes, delete)?;
        return Ok(());
    }

    // 📁 DIRECTORY
    if path.is_dir() {
        info("Detected directory target");

        if !cli.recursive {
            warn("Directory provided without --recursive flag. Skipping.");
            return Ok(());
        }

        let entries: Vec<PathBuf> = WalkDir::new(path)
            .contents_first(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_path_buf())
            .collect();

        info(&format!("Total entries found: {}", entries.len()));

        entries.par_iter().for_each(|entry| {
            if entry.is_file() {
                info(&format!("Processing file: {}", entry.display()));

                if let Err(e) = file::shred(entry, passes, delete) {
                    error(&format!("Failed file {}: {}", entry.display(), e));
                }
            } else if entry.is_dir() && delete {
                if let Err(e) = dir::remove(entry) {
                    error(&format!("Failed dir {}: {}", entry.display(), e));
                }
            }
        });

        success("Directory processing completed");
        return Ok(());
    }

    warn("Unknown target type encountered");
    Ok(())
}
