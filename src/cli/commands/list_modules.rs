// src/cli/commands/list_modules.rs
use crate::core::module_handler::ModuleHandler;

/// Command to list all available modules
pub fn list_modules(module_handler: &ModuleHandler) {
    module_handler.list_modules();
}