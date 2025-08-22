// src/modules/auxiliary/mod.rs
pub mod port_scanner;
pub mod http_header_injection;
pub mod sql_injection;  // أضف هذا السطر

pub use port_scanner::PortScanner;
pub use http_header_injection::HttpHeaderInjection;
pub use sql_injection::SQLInjectionScanner;  // أضف هذا