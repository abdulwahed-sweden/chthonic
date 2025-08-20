//! Port scanner auxiliary module for Chthonic framework.
//! Provides asynchronous TCP port scanning capabilities.

use async_trait::async_trait;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::core::module_handler::Module;

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
            parse_ports(ports_str)?
        };

        // Validate target host
        if target_host.is_empty() {
            return Err("Target host cannot be empty".to_string());
        }

        println!("[+] Starting port scan on: {}", target_host);
        let results = scan_ports(target_host, &ports_to_scan).await;
        
        if results.is_empty() {
            Ok("Scan completed. No open ports found.".to_string())
        } else {
            Ok(format!("Scan completed. Open ports: {:?}", results))
        }
    }
}

// Helper function to extract option values
fn extract_option<'a>(options: &'a [(String, String)], key: &str) -> Option<&'a String> {
    options.iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v)
}

// Helper function to parse port strings (e.g., "80,443,22" or "1-100")
fn parse_ports(ports_str: &str) -> Result<Vec<u16>, String> {
    let mut ports = Vec::new();
    
    for part in ports_str.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        
        if part.contains('-') {
            // Handle port ranges (e.g., "1-100")
            let range_parts: Vec<&str> = part.split('-').collect();
            if range_parts.len() != 2 {
                return Err(format!("Invalid port range format: {}", part));
            }
            
            let start = range_parts[0].parse::<u16>()
                .map_err(|e| format!("Invalid start port: {}", e))?;
            let end = range_parts[1].parse::<u16>()
                .map_err(|e| format!("Invalid end port: {}", e))?;
            
            if start > end {
                return Err(format!("Invalid range: start port {} > end port {}", start, end));
            }
            
            for port in start..=end {
                ports.push(port);
            }
        } else {
            // Handle single port
            let port = part.parse::<u16>()
                .map_err(|e| format!("Invalid port: {}", e))?;
            ports.push(port);
        }
    }
    
    if ports.is_empty() {
        return Err("No valid ports specified".to_string());
    }
    
    Ok(ports)
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
                    println!("[+] Port {} is OPEN", port);
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