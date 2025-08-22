//! Auxiliary (scanner/support) modules registry.
//!
//! Add new auxiliary modules here and re-export them for easy access from
//! the global modules registry.

pub mod port_scanner;
pub mod http_header_injection;
pub mod sql_injection;

pub use port_scanner::PortScanner;
pub use http_header_injection::HttpHeaderInjection;
pub use sql_injection::SQLInjectionScanner;
