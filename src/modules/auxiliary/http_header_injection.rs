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
                "X-Originating-IP".to_string(),
                "Host".to_string(),
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
        "Advanced HTTP header injection scanner testing for malicious header payloads and reflection vulnerabilities"
    }

    fn author(&self) -> &'static str {
        "Chthonic Underworld Team"
    }

    fn version(&self) -> &'static str {
        "1.1.0"
    }

    async fn run(&self, options: &[(String, String)]) -> Result<String, String> {
        let target_url = extract_option(options, "RHOSTS")
            .map(|s| s.as_str())
            .unwrap_or(&self.default_target);

        println!("{}", theme::info(&format!("Scanning: {}", target_url)));
        println!("{}", theme::info("Testing header injection vulnerabilities..."));
        println!("{}", theme::warning("This may take 30-60 seconds..."));
        println!("{}", theme::divider());

        let client = Client::builder()
            .timeout(Duration::from_secs(3))
            .user_agent("Chthonic/1.0 Security Scanner")
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        let mut vulnerabilities = Vec::new();
        let test_headers = self.default_headers.clone();
        let mut total_tests = 0;

        for (i, header) in test_headers.iter().enumerate() {
            println!("{}", theme::info(&format!("[{}/{}] Testing header: {}", i+1, test_headers.len(), header)));
            
            let test_values = vec![
                "127.0.0.1",
                "localhost", 
                "admin'--",
                "evil.com",
                "<script>alert(1)</script>",
            ];

            for test_value in test_values {
                total_tests += 1;
                print!(".");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                
                if let Ok(result) = test_header_injection(&client, target_url, header, test_value).await {
                    if result.vulnerable {
                        println!("\n{}", theme::error(&format!("🚨 VULNERABLE: {} = {}", header, test_value)));
                        println!("{}", theme::warning(&format!("Evidence: {}", result.evidence)));
                        vulnerabilities.push(format!("{}: {} ({})", result.header, result.payload, result.evidence));
                    }
                }
            }
            println!(" ✓");
        }

        println!("{}", theme::divider());
        println!("{}", theme::info(&format!("Completed {} tests on {} headers", total_tests, test_headers.len())));

        if vulnerabilities.is_empty() {
            println!("{}", theme::success("✅ No header injection vulnerabilities detected"));
            Ok("Scan completed successfully - No vulnerabilities found".to_string())
        } else {
            println!("{}", theme::error(&format!("🚨 SECURITY ALERT: {} vulnerabilities detected!", vulnerabilities.len())));
            for vuln in &vulnerabilities {
                println!("{}", theme::warning(&format!("  • {}", vuln)));
            }
            Ok(format!("🚨 Critical: {} header injection vulnerabilities found", vulnerabilities.len()))
        }
    }
}

// Helper struct for test results
#[derive(Debug)]
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
        Duration::from_secs(5),
        client.get(url).header(header, payload).send()
    ).await
    .map_err(|_| "Request timeout".to_string())?
    .map_err(|e| format!("Request failed: {}", e))?;

    // Clone headers before consuming response
    let response_headers = response.headers().clone();
    let status = response.status();
    
    // Read response body
    let body = response.text().await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    // Check for various types of header injection evidence
    let mut evidence = String::new();
    let mut vulnerable = false;

    // Check if payload is reflected in response body
    if body.contains(payload) {
        evidence = format!("Payload '{}' reflected in response body", payload);
        vulnerable = true;
    }

    // Check if payload is reflected in response headers
    for (name, value) in response_headers.iter() {
        if let Ok(value_str) = value.to_str() {
            if value_str.contains(payload) {
                evidence = format!("Payload '{}' reflected in response header '{}'", payload, name.as_str());
                vulnerable = true;
                break;
            }
        }
    }

    // Check for Host header injection (potential cache poisoning)
    if header.to_lowercase() == "host" && status.is_success() {
        evidence = format!("Host header injection accepted (potential cache poisoning)");
        vulnerable = true;
    }

    // Check for X-Forwarded-For IP spoofing acceptance
    if header.contains("Forwarded") || header.contains("Real-IP") || header.contains("Client-IP") {
        if status.is_success() && (payload == "127.0.0.1" || payload == "localhost") {
            // Additional check: see if the IP appears in response
            if body.contains(payload) {
                evidence = format!("IP spoofing via {} header successful", header);
                vulnerable = true;
            }
        }
    }

    // Check for script injection in User-Agent
    if header.to_lowercase() == "user-agent" && payload.contains("<script>") {
        if body.contains(payload) || body.contains("script") {
            evidence = format!("Potential XSS via User-Agent header");
            vulnerable = true;
        }
    }

    Ok(InjectionTestResult {
        header: header.to_string(),
        payload: payload.to_string(),
        evidence: if vulnerable { evidence } else { "No reflection detected".to_string() },
        vulnerable,
    })
}

// Helper function to extract options
fn extract_option<'a>(options: &'a [(String, String)], key: &str) -> Option<&'a String> {
    options.iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v)
}