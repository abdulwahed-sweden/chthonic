//! Chthonic Framework Theme and Styling Module
//! 
//! Provides consistent, professional terminal styling for the Chthonic penetration testing framework.
//! Features color-coded output, ASCII art banners, and standardized message formatting.

use colored::*;

/// Displays the main Chthonic framework banner with ASCII art and stylized text
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

/// Formats success messages with green checkmark and bold text
pub fn success(text: &str) -> String {
    format!("{} {}", "✓".bright_green(), text.bright_green().bold())
}

/// Formats error messages with red X mark and bold text  
pub fn error(text: &str) -> String {
    format!("{} {}", "✗".bright_red(), text.bright_red().bold())
}

/// Formats informational messages with blue info icon and bold text
pub fn info(text: &str) -> String {
    format!("{} {}", "ℹ".bright_blue(), text.bright_blue().bold())
}

/// Formats warning messages with yellow warning icon and bold text
pub fn warning(text: &str) -> String {
    format!("{} {}", "⚠".bright_yellow(), text.bright_yellow().bold())
}

/// Formats module-related messages with wrench icon and magenta text
pub fn module(text: &str) -> String {
    format!("{} {}", "🛠".bright_magenta(), text.bright_magenta().bold())
}

/// Formats port numbers with cyan underlined text
pub fn port(text: &str) -> String {
    format!("{}", text.bright_cyan().bold().underline())
}

/// Formats command prompt with angled bracket and white text
pub fn prompt(text: &str) -> String {
    format!("{} {}", ">".bright_white().bold(), text.bright_white())
}

/// Formats target host information with target icon and red text
pub fn target(text: &str) -> String {
    format!("{} {}", "🎯".bright_red(), text.bright_red().bold())
}

/// Formats payload information with bomb icon and yellow text
pub fn payload(text: &str) -> String {
    format!("{} {}", "💣".bright_yellow(), text.bright_yellow().bold())
}

/// Formats session information with lock icon and green text
pub fn session(text: &str) -> String {
    format!("{} {}", "🔒".bright_green(), text.bright_green().bold())
}

/// Formats configuration information with gear icon and blue text
pub fn config(text: &str) -> String {
    format!("{} {}", "⚙️".bright_blue(), text.bright_blue().bold())
}

/// Creates a horizontal divider line for section separation
pub fn divider() -> String {
    "─".repeat(60).bright_black().to_string()
}

/// Custom status formatter with dynamic icon and color
pub fn status(icon: &str, text: &str, color: Color) -> String {
    format!("{} {}", icon.color(color).bold(), text.color(color).bold())
}