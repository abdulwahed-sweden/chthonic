// src/cli/commands/use_module.rs
use crate::cli::CliState;

/// Command to select a module for use
pub async fn use_module(state: &mut CliState, module_name: &str) {
    match state.module_handler.get_module(module_name) {
        Some(_) => {
            state.current_module = Some(module_name.to_string());
            println!("[+] Using module: {}", module_name);
        }
        None => {
            println!("[-] Module not found: {}", module_name);
        }
    }
}