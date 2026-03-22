use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(name = "nullifer")]
#[command(about = "NULLIFER ⚔️ Secure File Shredder")]
pub struct Cli {
    pub target: Option<String>,

    #[arg(short, long)]
    pub recursive: bool,

    #[arg(short, long, default_value_t = 3)]
    pub passes: u8,

    #[arg(long)]
    pub no_encrypt: bool,

    #[arg(long)]
    pub no_rename: bool,

    #[arg(long)]
    pub parallel: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Shred {
        target: String,
        #[arg(short, long, default_value_t = 3)]
        passes: u8,
    },
    Wipe {
        target: String,
        #[arg(short, long, default_value_t = 1)]
        passes: u8,
    },
    Verify {
        target: String,
    },
}
