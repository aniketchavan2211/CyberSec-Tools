use colored::*;
use std::env;

/// Get terminal width safely
fn term_width() -> usize {
    env::var("COLUMNS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(100)
}

/// Rough display width (handles ⚔️ emoji as double-width)
fn display_width(s: &str) -> usize {
    let mut width = 0;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '⚔' {
            // skip variation selector if present
            if let Some('\u{fe0f}') = chars.peek() {
                chars.next();
            }
            width += 2; // emoji width
        } else {
            width += 1;
        }
    }

    width
}

/// Center text using display width (fixes misalignment)
fn center(text: &str, width: usize) -> String {
    let len = display_width(text);

    if len >= width {
        return text.to_string();
    }

    let pad = (width - len) / 2;
    format!("{}{}", " ".repeat(pad), text)
}

/// Print box
fn print_box(lines: &[String], width: usize) {
    let inner = width - 2;

    println!("{}", format!("╔{}╗", "═".repeat(inner)).bright_red());

    for _ in 0..2 {
        println!("{}", format!("║{}║", " ".repeat(inner)).bright_red());
    }

    for line in lines {
        let centered = center(line, inner);
        let mut padded = centered.clone();

        while display_width(&padded) < inner {
            padded.push(' ');
        }

        println!("{}", format!("║{}║", padded).bright_red());
    }

    for _ in 0..2 {
        println!("{}", format!("║{}║", " ".repeat(inner)).bright_red());
    }

    println!("{}", format!("╚{}╝", "═".repeat(inner)).bright_red());
}

pub fn banner() {
    let width = term_width().clamp(80, 140);
    let version = env!("CARGO_PKG_VERSION");

    let content = vec![
        "███╗   ██╗██╗   ██╗██╗     ██╗███████╗███████╗██████╗",
        "████╗  ██║██║   ██║██║     ██║██╔════╝██╔════╝██╔══██╗",
        "██╔██╗ ██║██║   ██║██║     ██║█████╗  █████╗  ██████╔╝",
        "██║╚██╗██║██║   ██║██║     ██║██╔══╝  ██╔══╝  ██╔══██╗",
        "██║ ╚████║╚██████╔╝███████╗██║██║     ███████╗██║  ██║",
        "╚═╝  ╚═══╝ ╚═════╝ ╚══════╝╚═╝╚═╝     ╚══════╝╚═╝  ╚═╝",
        "",
        "⚔️  Nullifer Sword  ⚔️", // ← NO manual spacing needed anymore
    ];

    let lines: Vec<String> = content.iter().map(|l| l.to_string()).collect();

    print_box(&lines, width);

    println!();

    println!("{}", center("👤 Author : Th3 R0gu3 Kn!ght", width).bright_cyan());
    println!("{}", center("🛡️ Team   : Th3 Cyb3r Kn!ght's", width).bright_blue());
    println!("{}", center("🌐 GitHub : github.com/aniketchavan2211", width).bright_black());
    println!("{}", center(&format!("⚙️ Version : v{}", version), width).bright_yellow());

    println!();
}
