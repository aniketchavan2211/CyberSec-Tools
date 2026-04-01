mod cli;
mod banner;
mod utils;
mod shredder;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();
    banner::banner();

    let cli = cli::Cli::parse();

    match &cli.command {
        Some(cmd) => shredder::dispatch_subcommand(cmd.clone(), &cli)?,
        None => shredder::dispatch_legacy(cli)?,
    }

    Ok(())
}
