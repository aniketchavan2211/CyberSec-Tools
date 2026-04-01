pub mod engine;
pub mod file;
pub mod dir;
pub mod verify;

use anyhow::Result;
use crate::cli::{Cli, Commands};

pub fn dispatch_subcommand(cmd: Commands, cli: &Cli) -> Result<()> {
    match cmd {
        Commands::Shred { target, passes } => engine::run(&target, passes, true, cli),
        Commands::Wipe { target, passes } => engine::run(&target, passes, false, cli),
        Commands::Verify { target } => verify::run(&target),
    }
}

pub fn dispatch_legacy(cli: Cli) -> Result<()> {
    let target = match &cli.target {
        Some(t) => t.clone(),
        None => {
            println!("Use --help");
            return Ok(());
        }
    };

    engine::run(&target, cli.passes, true, &cli)
}
