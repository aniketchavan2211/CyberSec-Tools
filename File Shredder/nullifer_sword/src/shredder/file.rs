use std::path::Path;
use std::fs::{remove_file, OpenOptions};
use std::io::{Write, Seek, SeekFrom};

use anyhow::Result;
use rand::RngCore;
use zeroize::Zeroize;
use indicatif::{ProgressBar, ProgressStyle};

use crate::utils::{info, error, success};

pub fn shred(path: &Path, passes: u8, delete: bool) -> Result<()> {
    info(&format!("Shredding file: {}", path.display()));

    let mut file = match OpenOptions::new().write(true).open(path) {
        Ok(f) => f,
        Err(e) => {
            error(&format!("Cannot open file {}: {}", path.display(), e));
            return Ok(());
        }
    };

    let size = file.metadata()?.len();
    info(&format!("File size: {} bytes", size));

    // 🔥 Progress bar
    let pb = ProgressBar::new(size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes}")
            .unwrap(),
    );
    pb.set_message(path.display().to_string());

    // 🔐 Overwrite buffer
    let mut buffer = vec![0u8; 65536];
    let mut rng = rand::thread_rng();

    for _ in 0..passes {
        file.seek(SeekFrom::Start(0))?;

        let mut written = 0;
        while written < size {
            let chunk = std::cmp::min(buffer.len() as u64, size - written);

            rng.fill_bytes(&mut buffer[..chunk as usize]);
            file.write_all(&buffer[..chunk as usize])?;

            written += chunk;
            pb.inc(chunk);
        }
    }

    pb.finish_and_clear();

    // 🧠 Zero memory
    buffer.zeroize();

    // truncate file
    file.set_len(0)?;

    drop(file);

    if delete {
        remove_file(path)?;
        success(&format!("File deleted: {}", path.display()));
    } else {
        success(&format!("File wiped: {}", path.display()));
    }

    Ok(())
}
