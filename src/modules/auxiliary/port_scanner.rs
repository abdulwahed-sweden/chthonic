//! Port scanner auxiliary module for Chthonic framework.
//! Provides asynchronous TCP port scanning capabilities.

use async_trait::async_trait;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::core::module_handler::Module;
use crate::utils::theme;
use crate::utils::helpers::{extract_option, parse_ports}; // استيراد كلاهما

/// Port scanner module for TCP port discovery
pub struct PortScanner {
    default_target: String,
    default_ports: Vec<u16>,
}

impl Default for PortScanner {
    fn default() -> Self {
        PortScanner {
            default_target: String::from("scanme.nmap.org"),
            default_ports: vec![21, 22, 80, 443, 8080],
        }
    }
}

#[async_trait]
impl Module for PortScanner {
    fn name(&self) -> &'static str {
        "auxiliary/port_scanner"
    }

    fn description(&self) -> &'static str {
        "Scans a target host for open TCP ports within a specified range using asynchronous I/O."
    }

    fn author(&self) -> &'static str {
        "Chthonic Team"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    async fn run(&self, options: &[(String, String)]) -> Result<String, String> {
        // Parse options with fallback to defaults
        let target_host = extract_option(options, "RHOSTS")
            .map(|s| s.as_str())
            .unwrap_or(&self.default_target);
        
        let ports_str = extract_option(options, "PORTS")
            .map(|s| s.as_str())
            .unwrap_or("");

        // Parse ports or use defaults
        let ports_to_scan = if ports_str.is_empty() {
            self.default_ports.clone()
        } else {
            parse_ports(ports_str)? // استخدام الدالة المستوردة
        };

        // Validate target host
        if target_host.is_empty() {
            return Err("Target host cannot be empty".to_string());
        }

        println!("{}", theme::info(&format!("Starting port scan on: {}", target_host)));
        let results = scan_ports(target_host, &ports_to_scan).await;
        
        if results.is_empty() {
            Ok("Scan completed. No open ports found.".to_string())
        } else {
            Ok(format!("Scan completed. Open ports: {:?}", results))
        }
    }
}

// Main port scanning logic
async fn scan_ports(target_host: &str, ports: &[u16]) -> Vec<u16> {
    let connection_timeout = Duration::from_secs(3);
    let mut open_ports = Vec::new();
    let mut tasks = Vec::new();

    // Create scanning tasks for each port
    for &port in ports {
        let target = target_host.to_string();
        tasks.push(tokio::spawn(async move {
            let address = format!("{}:{}", target, port);
            match timeout(connection_timeout, TcpStream::connect(&address)).await {
                Ok(Ok(_)) => {
                    println!("{}", theme::success(&format!("Port {} is OPEN", theme::port(&port.to_string()))));
                    Some(port)
                },
                _ => None,
            }
        }));
    }

    // Wait for all tasks to complete
    for task in tasks {
        if let Ok(Some(port)) = task.await {
            open_ports.push(port);
        }
    }

    open_ports
}