// src/cli/commands/show_options.rs
use crate::cli::CliState;

pub async fn show_options(state: &CliState) {
    if let Some(current_module) = &state.current_module {
        println!("[+] Options for module: {}", current_module);
        // سيتم تنفيذ العرض الفعلي هنا لاحقاً
        println!("[-] Option display not implemented yet");
    } else {
        println!("[-] No module selected. Use 'use' first.");
    }
}