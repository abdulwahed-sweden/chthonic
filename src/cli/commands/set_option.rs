// src/cli/commands/set_option.rs
use crate::cli::CliState;

pub async fn set_option(state: &mut CliState, option_name: &str, option_value: &str) {
    if let Some(current_module) = &state.current_module {
        state.module_options
            .entry(current_module.clone())
            .or_insert_with(std::collections::HashMap::new)
            .insert(option_name.to_string(), option_value.to_string());
        
        println!("[+] {} => {}", option_name, option_value);
    } else {
        println!("[-] No module selected. Use 'use' first.");
    }
}