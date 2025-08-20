// في src/cli/commands/use_module.rs
use crate::cli::CliState;
use std::collections::HashMap;

pub async fn use_module(state: &mut CliState, module_name: &str) {
    match state.module_handler.get_module(module_name) {
        Some(_) => {
            state.current_module = Some(module_name.to_string());
            // تأكد من وجود إدخال للإعدادات لهذه الوحدة
            state.module_options.entry(module_name.to_string())
                .or_insert_with(HashMap::new);
            println!("[+] Using module: {}", module_name);
        }
        None => {
            println!("[-] Module not found: {}", module_name);
        }
    }
}