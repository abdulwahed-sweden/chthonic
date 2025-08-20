// src/cli/commands/set_option.rs
use crate::cli::CliState;

pub async fn set_option(state: &mut CliState, option_name: &str, option_value: &str) {
    if let Some(current_module) = &state.current_module {
        println!("[+] Set option: {} = {} for module: {}", option_name, option_value, current_module);
        // سيتم التنفيذ الفعلي هنا لاحقاً
    } else {
        println!("[-] No module selected. Use 'use' first.");
    }
}