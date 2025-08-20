// src/cli/commands/show_options.rs
use crate::cli::CliState;

pub async fn show_options(state: &CliState) {
    if let Some(current_module) = &state.current_module {
        if let Some(options) = state.module_options.get(current_module) {
            if options.is_empty() {
                println!("[-] No options set for {}", current_module);
            } else {
                println!("[+] Module options ({}):", current_module);
                for (key, value) in options {
                    println!("   {} => {}", key, value);
                }
            }
        } else {
            println!("[-] No options found for {}", current_module);
        }
    } else {
        println!("[-] No module selected. Use 'use' first.");
    }
}