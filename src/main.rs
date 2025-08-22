// src/main.rs
mod cli;
mod core;
mod modules;
mod utils;

use crate::core::module_handler::ModuleHandler;
use crate::utils::theme;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Initialize theme and display banner (مرة واحدة فقط)
    theme::print_banner();
    
    // Initialize all core systems
    crate::core::init_core();
    
    // Initialize core components
    let _manager = core::session_manager::SessionManager::new();
    let mut module_handler = ModuleHandler::new();
    modules::register_all_modules(&mut module_handler);

    // Create CLI state and start the interactive loop
    let cli_state = cli::CliState {
        module_handler,
        current_module: None,
        module_options: HashMap::new(),
    };

    cli::run(cli_state).await;
}