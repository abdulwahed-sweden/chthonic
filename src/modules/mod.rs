//! Global modules registry.
//!
//! Each module must be registered here so the CLI can list and use it.

pub mod exploits;
pub mod auxiliary;

use crate::core::module_handler::ModuleHandler;

pub fn register_all_modules(handler: &mut ModuleHandler) {
    // Exploits
    handler.register_module(
        "exploit/test_exploit",
        Box::new(exploits::TestExploit),
    );

    // Auxiliary (scanners)
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
