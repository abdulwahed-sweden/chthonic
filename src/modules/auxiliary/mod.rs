// src/modules/auxiliary/mod.rs
pub mod port_scanner;
pub mod http_header_injection;  // أضف هذا

pub use port_scanner::PortScanner;
pub use http_header_injection::HttpHeaderInjection;  // أضف هذا