use colored::*;
use log::{info as log_info, warn as log_warn, error as log_error};

pub fn info(msg: &str) {
    log_info!("{}", msg);
    println!("{} {}", "[INFO]".cyan().bold(), msg);
}

pub fn warn(msg: &str) {
    log_warn!("{}", msg);
    println!("{} {}", "[WARN]".yellow().bold(), msg);
}

pub fn error(msg: &str) {
    log_error!("{}", msg);
    eprintln!("{} {}", "[ERROR]".red().bold(), msg);
}

pub fn success(msg: &str) {
    println!("{} {}", "[OK]".green().bold(), msg);
}
