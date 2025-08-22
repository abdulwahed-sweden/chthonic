//! HTTP Header Injection Scanner Module
//! Detects insecure header processing in web applications

use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;
use tokio::time::timeout;

use crate::core::module_handler::Module;
use crate::utils::theme;

/// HTTP Header Injection scanner
pub struct HttpHeaderInjection {
    default_target: String,
    default_headers: Vec<String>,
}

impl Default for HttpHeaderInjection {
    fn default() -> Self {
        HttpHeaderInjection {
            default_target: String::from("https://httpbin.org"),
            default_headers: vec![
                "X-Forwarded-For".to_string(),
                "User-Agent".to_string(), 
                "Referer".to_string(),
                "X-Real-IP".to_string(),
            ],
        }
    }
}

#[async_trait]
impl Module for HttpHeaderInjection {
    fn name(&self) -> &'static str {
        "auxiliary/http_header_injection"
    }

    fn description(&self) -> &'static str {
        "Scans for HTTP header injection vulnerabilities by sending malicious header payloads"
    }

    fn author(&self) -> &'static str {
        "Chthonic Team"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    async fn run(&self, options: &[(String, String)]) -> Result<String, String> {
        let target_url = extract_option(options, "RHOSTS")
            .map(|s| s.as_str())
            .unwrap_or(&self.default_target);

        let test_headers = self.default_headers.clone();

        println!("{}", theme::info(&format!("Scanning: {}", target_url)));
        println!("{}", theme::info("Testing header injection vulnerabilities..."));

        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        let mut vulnerabilities = Vec::new();

        for header in test_headers {
            let test_values = vec![
                "127.0.0.1",
                "localhost",
                "admin'--",
                "\"; sleep(5); --",
                "<?php system('id'); ?>",
            ];

            for test_value in test_values {
                if let Ok(result) = test_header_injection(&client, target_url, &header, test_value).await {
                    if result.vulnerable {
                        vulnerabilities.push(format!("{}: {} = {}", result.header, result.payload, result.evidence));
                        println!("{}", theme::error(&format!("VULNERABLE: {} = {}", header, test_value)));
                    }
                }
            }
        }

        if vulnerabilities.is_empty() {
            Ok("No header injection vulnerabilities found.".to_string())
        } else {
            Ok(format!("Found {} vulnerabilities:\n{}", 
                vulnerabilities.len(), 
                vulnerabilities.join("\n")))
        }
    }
}

// Helper struct for test results
struct InjectionTestResult {
    header: String,
    payload: String,
    evidence: String,
    vulnerable: bool,
}

// Test a specific header for injection vulnerabilities
async fn test_header_injection(
    client: &Client,
    url: &str,
    header: &str,
    payload: &str,
) -> Result<InjectionTestResult, String> {
    let response = timeout(
        Duration::from_secs(10),
        client.get(url).header(header, payload).send()
    ).await
    .map_err(|e| format!("Timeout: {}", e))?
    .map_err(|e| format!("Request failed: {}", e))?;

    // احفظ الـ headers قبل استدعاء text()
    let response_headers = response.headers().clone();
    let body = response.text().await.map_err(|e| format!("Failed to read response: {}", e))?;

    // Check for evidence of injection in response
    let vulnerable = body.contains(payload) || 
                    response_headers.get(header)
                        .map(|h| h.to_str().unwrap_or("").contains(payload))
                        .unwrap_or(false);

    Ok(InjectionTestResult {
        header: header.to_string(),
        payload: payload.to_string(),
        evidence: if vulnerable { "Payload reflected in response".to_string() } else { "No reflection".to_string() },
        vulnerable,
    })
}

// Helper function to extract options
fn extract_option<'a>(options: &'a [(String, String)], key: &str) -> Option<&'a String> {
    options.iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v)
}