// src/cli/commands/run_module.rs
use crate::cli::CliState;

/// Command to execute the currently selected module
pub async fn run_module(state: &CliState) {
    match &state.current_module {
        Some(module_name) => {
            println!("[+] Running module: {}", module_name);
            if let Some(module) = state.module_handler.get_module(module_name) {
                match module.run().await {
                    Ok(result) => println!("[+] Result: {}", result),
                    Err(e) => println!("[-] Error: {}", e),
                }
            } else {
                println!("[-] Module not available: {}", module_name);
            }
        }
        None => {
            println!("[-] No module selected. Use 'use <module_name>' first.");
        }
    }
}