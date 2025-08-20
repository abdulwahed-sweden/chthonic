// src/cli/commands/mod.rs

// Sub-modules for each command
pub mod use_module;
pub mod list_modules;
pub mod run_module;
pub mod set_option;
pub mod show_options;

// Re-export all command functions for easy access
pub use use_module::*;
pub use list_modules::*;
pub use run_module::*;
pub use set_option::*;
pub use show_options::*;