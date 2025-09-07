use crate::cli::ShodanArgs;
use crate::utils::http::HttpClient;
use crate::config::Config;
use anyhow::Result;
use colored::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct ShodanResult {
    pub query: String,
    pub results: Vec<ShodanHost>,
    pub total_found: usize,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ShodanHost {
    pub ip: String,
    pub port: u16,
    pub service: String,
    pub banner: String,
    pub location: String,
    pub org: String,
    pub vulns: Vec<String>,
}
pub async fn run(args: ShodanArgs) -> Result<()> {
    println!("{} Shodan search: {}", "🔍".cyan(), args.query.yellow().bold());
    let config = Config::load()?;
    if config.get_api_key("shodan").is_none() {
        println!("{} No Shodan API key configured. Use: osint_toolkit config set-key shodan YOUR_API_KEY", "⚠".yellow());
        println!("{} Showing demo data instead...", "ℹ".cyan());
    }
    let client = HttpClient::new()?;
    let results = search_shodan(&client, &args.query, args.limit.unwrap_or(10), args.vulns).await?;
    display_results(&results);
    Ok(())
}
async fn search_shodan(client: &HttpClient, query: &str, limit: usize, include_vulns: bool) -> Result<ShodanResult> {
    let config = Config::load()?;
    
    if let Some(api_key) = config.get_api_key("shodan") {
        let url = format!("https://api.shodan.io/shodan/host/search?key={}&query={}&limit={}", 
            api_key, urlencoding::encode(query), limit);
        
        match client.get(&url).await {
            Ok(response) => {
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(&response) {
                    let mut hosts = vec![];
                    
                    if let Some(matches) = data.get("matches").and_then(|v| v.as_array()) {
                        for match_data in matches.iter().take(limit) {
                            let ip = match_data.get("ip_str")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown")
                                .to_string();
                            
                            let port = match_data.get("port")
                                .and_then(|v| v.as_u64())
                                .unwrap_or(80) as u16;
                            
                            let service = match_data.get("product")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown")
                                .to_string();
                            
                            let banner = match_data.get("data")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .lines()
                                .take(3)
                                .collect::<Vec<_>>()
                                .join("\n");
                            
                            let location = format!("{}, {}",
                                match_data.get("location").and_then(|l| l.get("city")).and_then(|v| v.as_str()).unwrap_or("Unknown"),
                                match_data.get("location").and_then(|l| l.get("country_name")).and_then(|v| v.as_str()).unwrap_or("Unknown")
                            );
                            
                            let org = match_data.get("org")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown")
                                .to_string();
                            
                            let mut vulns = vec![];
                            if include_vulns {
                                if let Some(vulns_data) = match_data.get("vulns").and_then(|v| v.as_object()) {
                                    vulns = vulns_data.keys().cloned().collect();
                                }
                            }
                            
                            hosts.push(ShodanHost {
                                ip,
                                port,
                                service,
                                banner,
                                location,
                                org,
                                vulns,
                            });
                        }
                    }
                    
                    let total = data.get("total")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0) as usize;
                    
                    return Ok(ShodanResult {
                        query: query.to_string(),
                        total_found: total,
                        results: hosts,
                    });
                }
            }
            Err(e) => {
                println!("{} Shodan API error: {}", "⚠".yellow(), e);
                println!("{} Falling back to demo data...", "ℹ".cyan());
            }
        }
    }
    
    println!("{} Using demo data due to API limitations", "ℹ".cyan());
    
    let mut hosts = vec![];
    for i in 0..limit.min(5) {
        let mut vulns = vec![];
        if include_vulns {
            vulns.extend_from_slice(&[
                "CVE-2021-44228".to_string(),
                "CVE-2022-0847".to_string(),
            ]);
        }
        hosts.push(ShodanHost {
            ip: format!("192.168.1.{}", i + 10),
            port: match i {
                0 => 80,
                1 => 443,
                2 => 22,
                3 => 21,
                _ => 8080,
            },
            service: match i {
                0 => "nginx/1.18.0".to_string(),
                1 => "Apache/2.4.41".to_string(),
                2 => "OpenSSH 8.2".to_string(),
                3 => "ProFTPD 1.3.6".to_string(),
                _ => "Unknown".to_string(),
            },
            banner: format!("HTTP/1.1 200 OK\r\nServer: {}\r\n", match i {
                0 => "nginx",
                1 => "Apache",
                _ => "Unknown"
            }),
            location: match i % 3 {
                0 => "United States".to_string(),
                1 => "Germany".to_string(),
                _ => "France".to_string(),
            },
            org: format!("Example Corp {}", i + 1),
            vulns,
        });
    }
    
    Ok(ShodanResult {
        query: query.to_string(),
        total_found: hosts.len(),
        results: hosts,
    })
}
fn display_results(result: &ShodanResult) {
    println!("\n{}", "Shodan Search Results:".green().bold());
    println!("{}", "═════════════════════".cyan());
    println!("  {} {}", "Query:".yellow(), result.query.cyan());
    println!("  {} {}", "Results Found:".yellow(), result.total_found.to_string().green());
    if result.results.is_empty() {
        println!("\n{} No results found", "✗".red());
        return;
    }
    for (i, host) in result.results.iter().enumerate() {
        println!("\n{}. {} {}",
            (i + 1).to_string().cyan(),
            "Host:".yellow(),
            host.ip.cyan().bold()
        );
        println!("   {} {}", "Port:".yellow(), host.port.to_string().green());
        println!("   {} {}", "Service:".yellow(), host.service.cyan());
        println!("   {} {}", "Location:".yellow(), host.location.cyan());
        println!("   {} {}", "Organization:".yellow(), host.org.cyan());
        if !host.vulns.is_empty() {
            println!("   {} {}", "Vulnerabilities:".red(), host.vulns.join(", ").red());
        }
        if !host.banner.is_empty() {
            println!("   {} {}", "Banner:".yellow(), host.banner.dimmed());
        }
    }
    println!("\n{}", "Security Notes:".yellow().bold());
    println!("• Always ensure you have permission before scanning");
    println!("• Consider firewall rules and rate limiting");
    println!("• Verify vulnerabilities with additional tools");
}
