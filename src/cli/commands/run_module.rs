// في src/cli/commands/run_module.rs
use crate::cli::CliState;

pub async fn run_module(state: &CliState) {
    match &state.current_module {
        Some(module_name) => {
            println!("[+] Running module: {}", module_name);
            if let Some(module) = state.module_handler.get_module(module_name) {
                // هنا يجب تمرير الإعدادات للوحدة!
                // هذه تحتاج تعديل Module trait ليقبل parameters
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