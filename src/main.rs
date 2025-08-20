// src/main.rs
// ... (بقية الـ imports)
mod cli;
mod core;
mod modules;

use crate::core::module_handler::ModuleHandler;
use std::collections::HashMap; // أضف هذا الاستيراد

#[tokio::main]
async fn main() {
    println!("[+] Chthonic Rising from the Underworld... 🦀☠️");

    // Initialize core components
    let _manager = core::session_manager::SessionManager::new();
    let mut module_handler = ModuleHandler::new();
    modules::register_all_modules(&mut module_handler);

    // Create CLI state and start the interactive loop
    let cli_state = cli::CliState {
        module_handler,
        current_module: None,
        module_options: HashMap::new(), // أضف هذا الحقل
    };

    cli::run(cli_state).await;
}