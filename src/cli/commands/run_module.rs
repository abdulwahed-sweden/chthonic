// src/cli/commands/run_module.rs
use crate::cli::CliState;
use crate::utils::theme;  // غير إلى utils

pub async fn run_module(state: &CliState) {
    match &state.current_module {
        Some(module_name) => {
            println!("{}", theme::info(&format!("Running module: {}", module_name)));
            if let Some(module) = state.module_handler.get_module(module_name) {
                let options = state.module_options
                    .get(module_name)
                    .map(|opts| opts.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<Vec<_>>())
                    .unwrap_or_default();

                match module.run(&options).await {
                    Ok(result) => println!("{}", theme::success(&format!("Result: {}", result))),
                    Err(e) => println!("{}", theme::error(&format!("Error: {}", e))),
                }
            } else {
                println!("{}", theme::error("Module not available"));
            }
        }
        None => {
            println!("{}", theme::error("No module selected. Use 'use <module_name>' first."));
        }
    }
}