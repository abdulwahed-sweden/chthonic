//! 🔥 Chthonic CLI Theme & Styling Utilities
//! Dark, professional, and intimidating theme for the ultimate penetration testing framework
#[allow(dead_code)]

use colored::*;

/// Prints the main ASCII banner with elite hacker aesthetic
pub fn print_banner() {
    let banner = r#"
   ╔═╗┬ ┬┌┬┐┌┬┐┌─┐┬  ┌─┐┬ ┬
   ║ ╦│ │ │  │ ├┤ │  ├─┤└┬┘
   ╚═╝└─┘ ┴  ┴ └─┘┴─┘┴ ┴ ┴ 
    ██████╗██╗  ██╗████████╗██╗  ██╗ ██████╗ ███╗   ██╗██╗ ██████╗
   ██╔════╝██║  ██║╚══██╔══╝██║  ██║██╔════╝ ████╗  ██║██║██╔═══██╗
   ██║     ███████║   ██║   ███████║██║  ███╗██╔██╗ ██║██║██║   ██║
   ██║     ██╔══██║   ██║   ██╔══██║██║   ██║██║╚██╗██║██║██║   ██║
   ╚██████╗██║  ██║   ██║   ██║  ██║╚██████╔╝██║ ╚████║██║╚██████╔╝
    ╚═════╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝ ╚═════╝
    "#;

    println!("{}", banner.bright_red().bold());
    println!("{}", "        ↳ UNDERWORLD PENETRATION FRAMEWORK 🦀☠️\n".bright_black().italic());
}

/// Elite success styling
pub fn success(text: &str) -> String {
    format!("{} {}", "✓".bright_green(), text.bright_green().bold())
}

/// Critical error styling
pub fn error(text: &str) -> String {
    format!("{} {}", "✗".bright_red(), text.bright_red().bold())
}

/// Tactical info styling
pub fn info(text: &str) -> String {
    format!("{} {}", "ℹ".bright_blue(), text.bright_blue().bold())
}

/// Security warning styling
pub fn warning(text: &str) -> String {
    format!("{} {}", "⚠".bright_yellow(), text.bright_yellow().bold())
}

/// Module execution styling
pub fn module(text: &str) -> String {
    format!("{} {}", "🛠".bright_magenta(), text.bright_magenta().bold())
}

/// Network port styling
pub fn port(text: &str) -> String {
    format!("{}", text.bright_cyan().bold().underline())
}

/// Command prompt styling
pub fn prompt(text: &str) -> String {
    format!("{} {}", ">".bright_white().bold(), text.bright_white())
}

/// Target host styling
pub fn target(text: &str) -> String {
    format!("{} {}", "🎯".bright_red(), text.bright_red().bold())
}

/// Payload styling
pub fn payload(text: &str) -> String {
    format!("{} {}", "💣".bright_yellow(), text.bright_yellow().bold())
}

/// Session styling
pub fn session(text: &str) -> String {
    format!("{} {}", "🔒".bright_green(), text.bright_green().bold())
}

/// Configuration styling
pub fn config(text: &str) -> String {
    format!("{} {}", "⚙️".bright_blue(), text.bright_blue().bold())
}

/// Creates a horizontal divider for output sections
pub fn divider() -> String {
    "─".repeat(60).bright_black().to_string()
}