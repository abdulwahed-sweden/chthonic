//! Common helper functions shared across the framework.
//! - Option extractors (string/bool/usize)
//! - Robust port list parsing (e.g., "80,443,1000-1005")

use std::collections::BTreeSet;

/// Extract an option value by key (case-insensitive) from `(String, String)` pairs.
pub fn extract_option<'a>(options: &'a [(String, String)], key: &str) -> Option<&'a String> {
    let needle = key.trim().to_ascii_lowercase();
    options
        .iter()
        .find(|(k, _)| k.trim().eq_ignore_ascii_case(&needle))
        .map(|(_, v)| v)
}

/// Parse a boolean option.
/// Accepts: true/false/1/0/yes/no/y/n (case-insensitive).
pub fn extract_bool_option(options: &[(String, String)], key: &str) -> Option<bool> {
    extract_option(options, key).and_then(|raw| match raw.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "y" => Some(true),
        "false" | "0" | "no" | "n" => Some(false),
        _ => None,
    })
}

/// Parse an unsigned integer option (`usize`).
pub fn extract_usize_option(options: &[(String, String)], key: &str) -> Option<usize> {
    extract_option(options, key).and_then(|raw| raw.trim().parse::<usize>().ok())
}

/// Parse a port list string into a sorted, de-duplicated vector of valid ports.
/// Supports single ports and ranges (e.g., "80,443,1000-1005").
pub fn parse_ports(ports_str: &str) -> Result<Vec<u16>, String> {
    if ports_str.trim().is_empty() {
        return Err("No ports specified".to_string());
    }

    let mut ports: BTreeSet<u16> = BTreeSet::new();

    for raw in ports_str.split(',') {
        let part = raw.trim();
        if part.is_empty() {
            continue;
        }

        if let Some(idx) = part.find('-') {
            let (a, b) = part.split_at(idx);
            let b = &b[1..];
            let start = a.trim().parse::<u32>().map_err(|_| format!("Invalid start port: '{part}'"))?;
            let end = b.trim().parse::<u32>().map_err(|_| format!("Invalid end port: '{part}'"))?;
            if start == 0 || end == 0 || start > 65535 || end > 65535 {
                return Err(format!("Port range out of bounds (1-65535): '{part}'"));
            }
            if start > end {
                return Err(format!("Invalid range (start > end): '{part}'"));
            }
            for p in start..=end {
                ports.insert(p as u16);
            }
        } else {
            let p = part.parse::<u32>().map_err(|_| format!("Invalid port: '{part}'"))?;
            if p == 0 || p > 65535 {
                return Err(format!("Port out of bounds (1-65535): '{part}'"));
            }
            ports.insert(p as u16);
        }
    }

    if ports.is_empty() {
        return Err("No valid ports parsed".to_string());
    }

    Ok(ports.into_iter().collect())
}
