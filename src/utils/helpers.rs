//! Common helper functions shared across modules
//! Provides utility functions for module operations

/// Extract option value by key from options vector
/// 
/// # Arguments
/// * `options` - Vector of key-value tuples
/// * `key` - Key to search for
/// 
/// # Returns
/// Option containing reference to the value if found
pub fn extract_option<'a>(options: &'a [(String, String)], key: &str) -> Option<&'a String> {
    options.iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v)
}

/// Parse port range string into vector of ports
/// 
/// # Arguments
/// * `ports_str` - String containing ports (e.g., "80,443,1000-1005")
/// 
/// # Returns
/// Result with vector of ports or error message
pub fn parse_ports(ports_str: &str) -> Result<Vec<u16>, String> {
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