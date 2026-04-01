use colored::*;
use std::env;

/// Get terminal width safely
fn term_width() -> usize {
    env::var("COLUMNS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(100)
}

/// Handle emoji width (⚔️ = 2 width)
fn display_width(s: &str) -> usize {
    let mut width = 0;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '⚔' {
            if let Some('\u{fe0f}') = chars.peek() {
                chars.next();
            }
            width += 2;
        } else {
            width += 1;
        }
    }

    width
}

/// Center text safely
fn center(text: &str, width: usize) -> String {
    let len = display_width(text);

    if len >= width {
        return text.to_string();
    }

    let pad = (width - len) / 2;
    format!("{}{}", " ".repeat(pad), text)
}

/// Print banner (no border)
fn print_box(lines: &[String], width: usize) {
    println!();

    for line in lines {
        println!("{}", center(line, width).bright_red());
    }

    println!();
}

pub fn banner() {
    let width = term_width().clamp(80, 140);

    // 🔥 Cargo auto metadata
    let version = env!("CARGO_PKG_VERSION");
    let author = option_env!("CARGO_PKG_AUTHORS").unwrap_or("Unknown");
    let name = env!("CARGO_PKG_NAME");
    let repo = option_env!("CARGO_PKG_REPOSITORY").unwrap_or("N/A");

    let content = vec![
        "███╗   ██╗██╗   ██╗██╗     ██╗███████╗███████╗██████╗ ",
        "████╗  ██║██║   ██║██║     ██║██╔════╝██╔════╝██╔══██╗",
        "██╔██╗ ██║██║   ██║██║     ██║█████╗  █████╗  ██████╔╝",
        "██║╚██╗██║██║   ██║██║     ██║██╔══╝  ██╔══╝  ██╔══██╗",
        "██║ ╚████║╚██████╔╝███████╗██║██║     ███████╗██║  ██║",
        "╚═╝  ╚═══╝ ╚═════╝ ╚══════╝╚═╝╚═╝     ╚══════╝╚═╝  ╚═╝",
        "",
        "⚔️  NULLIFER SWORD  ⚔️",
        "",
        "[ Offensive Security Tool | Secure Deletion Engine ]",
    ];

    let lines: Vec<String> = content.iter().map(|l| l.to_string()).collect();
    print_box(&lines, width);

    // ===== METADATA =====
    println!("{}", center("═══════════════════════════════════════════════", width).bright_black());

    println!("{}", center(&format!("📦 Package : {}", name), width).bright_magenta());
    println!("{}", center(&format!("⚙️ Version : v{}", version), width).bright_yellow());
    println!("{}", center(&format!("👤 Author  : {}", author), width).bright_cyan());
    println!("{}", center("🛡️ Team    : Th3 Cyb3r Kn!ght's", width).bright_blue());
    println!("{}", center(&format!("🌐 Repo    : {}", repo), width).bright_black());

    println!("{}", center("═══════════════════════════════════════════════", width).bright_black());

    println!();
}
