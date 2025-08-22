//! Core framework components module
//! Contains essential systems for Chthonic operation

pub mod session_manager;
pub mod module_handler;
pub mod database;
pub mod event_bus;
pub mod logger;

/// Initializes all core systems
pub fn init_core() {
    println!("{}", "Initializing core systems...".to_string());
    database::init();
    event_bus::init();
    logger::init();
    println!("{}", "Core systems ready ✅".to_string());
}