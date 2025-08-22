// src/modules/mod.rs
pub mod exploits;
pub mod auxiliary;

use crate::core::module_handler::ModuleHandler;

pub fn register_all_modules(handler: &mut ModuleHandler) {
    // Register exploits
    handler.register_module(
        "exploit/test_exploit",
        Box::new(exploits::TestExploit),
    );

    // Register auxiliary modules
    handler.register_module(
        "auxiliary/port_scanner",
        Box::new(auxiliary::PortScanner::default()),
    );

    handler.register_module(
        "auxiliary/http_header_injection",
        Box::new(auxiliary::HttpHeaderInjection::default()),
    );

    handler.register_module(
        "auxiliary/sql_injection",
        Box::new(auxiliary::SQLInjectionScanner::default()),
    );
}
