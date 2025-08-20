// src/cli/mod.rs
pub mod commands;
use std::collections::HashMap;
use std::io::{self, Write};

use crate::core::module_handler::ModuleHandler;

/// Holds the current state of the CLI application
pub struct CliState {
    pub module_handler: ModuleHandler,
    pub current_module: Option<String>, // Name of the currently active module
    pub module_options: HashMap<String, HashMap<String, String>>, // ModuleName -> (OptionName -> Value)
}

/// Main function to run the CLI loop
pub async fn run(mut state: CliState) {
    println!("[+] Chthonic CLI initialized. Type 'help' for commands.");

    loop {
        // Print prompt based on active module
        let prompt = match &state.current_module {
            Some(module) => format!("chthonic ({}) > ", module),
            None => "chthonic > ".to_string(),
        };
        print!("{}", prompt);
        io::stdout().flush().unwrap(); // Force immediate prompt display

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Parse command and arguments
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts.get(0).unwrap_or(&"").to_lowercase();
        let args = &parts[1..];

        // Execute the command
        match command.as_str() {
            "list" => commands::list_modules(&state.module_handler),
            "use" => {
                if let Some(module_name) = args.get(0) {
                    commands::use_module(&mut state, module_name).await;
                } else {
                    println!("[-] Usage: use <module_name>");
                }
            }
            "run" => commands::run_module(&state).await,
            "set" => {
                if let Some(option_name) = args.get(0) {
                    if let Some(option_value) = args.get(1) {
                        commands::set_option(&mut state, option_name, option_value).await;
                    } else {
                        println!("[-] Usage: set <option> <value>");
                    }
                } else {
                    println!("[-] Usage: set <option> <value>");
                }
            }
            "show" => {
                commands::show_options(&state).await;
            }
            "exit" => {
                println!("[+] Exiting Chthonic. Goodbye! 🦀");
                break;
            }
            "help" => {
                println!("Available commands:");
                println!("  list            List all available modules");
                println!("  use <module>    Select a module to use");
                println!("  run             Run the currently selected module");
                println!("  set <opt> <val> Set a module option");
                println!("  show            Show current module options");
                println!("  exit            Exit the CLI");
            }
            "" => {} // Ignore empty input
            _ => println!("[-] Unknown command: '{}'. Type 'help'", command),
        }
    }
}