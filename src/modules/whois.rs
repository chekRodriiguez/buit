use crate::cli::WhoisArgs;
use crate::utils::http::HttpClient;
use anyhow::Result;
use colored::*;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
#[derive(Debug, Serialize, Deserialize)]
pub struct WhoisResult {
    pub target: String,
    pub target_type: String,
    pub registrar: Option<String>,
    pub creation_date: Option<String>,
    pub expiration_date: Option<String>,
    pub name_servers: Vec<String>,
    pub organization: Option<String>,
    pub country: Option<String>,
    pub emails: Vec<String>,
    pub raw_data: String,
    pub parsed: bool,
}
pub async fn run(args: WhoisArgs) -> Result<()> {
    println!("{} WHOIS lookup: {}", "🔍".cyan(), args.target.yellow().bold());
    let client = HttpClient::new()?;
    let target_type = if args.target.parse::<IpAddr>().is_ok() {
        "IP"
    } else {
        "Domain"
    };
    let mut result = WhoisResult {
        target: args.target.clone(),
        target_type: target_type.to_string(),
        registrar: None,
        creation_date: None,
        expiration_date: None,
        name_servers: vec![],
        organization: None,
        country: None,
        emails: vec![],
        raw_data: String::new(),
        parsed: false,
    };
    let mut success = false;
    println!("  {} Trying local WHOIS command...", "🔍".cyan());
    if let Ok(output) = std::process::Command::new("whois")
        .arg(&args.target)
        .output()
    {
        if !output.stdout.is_empty() {
            result.raw_data = String::from_utf8_lossy(&output.stdout).to_string();
            success = true;
            if args.parse {
                let data = result.raw_data.clone();
                parse_whois_text(&mut result, &data);
            }
            println!("  {} Local WHOIS command successful", "✓".green());
        } else if !output.stderr.is_empty() {
            let error = String::from_utf8_lossy(&output.stderr);
            println!("  {} Local WHOIS command failed: {}", "⚠".yellow(), error.trim());
        }
    } else {
        println!("  {} Local WHOIS command not available", "⚠".yellow());
    }
    if !success {
        println!("  {} Trying web services...", "🔍".cyan());
        let whois_services = if target_type == "IP" {
            vec![
                format!("https://ipapi.co/{}/json", args.target),
            ]
        } else {
            vec![
                format!("http://whois.domaintools.com/{}", args.target),
            ]
        };
        for (index, url) in whois_services.iter().enumerate() {
            println!("  {} Trying web service {}...", "🔍".cyan(), index + 1);
            match client.get(url).await {
                Ok(response) => {
                    if !response.is_empty()
                        && !response.contains("error")
                        && !response.contains("API Key")
                        && !response.contains("captcha")
                        && !response.contains("Security Check") {
                        result.raw_data = response.clone();
                        success = true;
                        if args.parse {
                            if target_type == "IP" && url.contains("ipapi.co") {
                                parse_ipapi_data(&mut result, &response);
                            } else {
                                parse_whois_text(&mut result, &response);
                            }
                        }
                        println!("  {} Web service successful", "✓".green());
                        break;
                    }
                }
                Err(_) => {
                    println!("  {} Web service {} failed", "⚠".yellow(), index + 1);
                    continue;
                }
            }
        }
    }
    if !success {
        println!("  {} All methods failed, using demo data", "ℹ".blue());
        result.raw_data = generate_sample_whois(&args.target, target_type);
        if args.parse {
            parse_sample_whois(&mut result, target_type);
        }
    }
    display_results(&result, args.parse);
    Ok(())
}
fn generate_sample_whois(target: &str, target_type: &str) -> String {
    match target_type {
        "Domain" => format!(r#"
Domain Name: {}
Registry Domain ID: 123456789_DOMAIN_COM-VRSN
Registrar WHOIS Server: whois.registrar.com
Registrar URL: http:
Updated Date: 2024-01-15T10:00:00Z
Creation Date: 2020-01-15T10:00:00Z
Registry Expiry Date: 2025-01-15T10:00:00Z
Registrar: Example Registrar, Inc.
Registrar IANA ID: 12345
Registrar Abuse Contact Email: abuse@registrar.com
Registrar Abuse Contact Phone: +1.5551234567
Domain Status: clientTransferProhibited
Registry Registrant ID: REDACTED
Registrant Name: REDACTED FOR PRIVACY
Registrant Organization: Privacy Service
Registrant Street: REDACTED FOR PRIVACY
Registrant City: REDACTED FOR PRIVACY
Registrant State/Province: REDACTED FOR PRIVACY
Registrant Postal Code: REDACTED FOR PRIVACY
Registrant Country: US
Registrant Phone: REDACTED FOR PRIVACY
Registrant Email: REDACTED FOR PRIVACY
Name Server: ns1.example.com
Name Server: ns2.example.com
DNSSEC: unsigned
"#, target),
        "IP" => format!(r#"
NetRange:       108.89.0.0 - 108.89.255.255
CIDR:           108.89.0.0/16
NetName:        AT-T-ENTERPRISES
NetHandle:      NET-108-89-0-0-1
Parent:         NET108 (NET-108-0-0-0-0)
NetType:        Direct Allocation
OriginAS:       AS7018
Organization:   AT&T Enterprises, LLC (ATTS-37)
RegDate:        2010-01-15
Updated:        2020-03-15
Ref:            https:
OrgName:        AT&T Enterprises, LLC
OrgId:          ATTS-37
Address:        1120 20th Street NW
Address:        Suite 800 South
City:           Washington
StateProv:      DC
PostalCode:     20036
Country:        US
RegDate:        2005-01-15
Updated:        2023-03-15
"#),
        _ => "No WHOIS data available".to_string(),
    }
}
fn parse_sample_whois(result: &mut WhoisResult, target_type: &str) {
    match target_type {
        "Domain" => {
            result.registrar = Some("Example Registrar, Inc.".to_string());
            result.creation_date = Some("2020-01-15".to_string());
            result.expiration_date = Some("2025-01-15".to_string());
            result.name_servers = vec!["ns1.example.com".to_string(), "ns2.example.com".to_string()];
            result.organization = Some("Privacy Service".to_string());
            result.country = Some("US".to_string());
            result.emails = vec!["abuse@registrar.com".to_string()];
        }
        "IP" => {
            result.organization = Some("AT&T Enterprises, LLC".to_string());
            result.country = Some("US".to_string());
            result.emails = vec![];
        }
        _ => {}
    }
    result.parsed = true;
}
fn parse_ipapi_data(result: &mut WhoisResult, data: &str) {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
        result.organization = json.get("org").and_then(|v| v.as_str()).map(|s| s.to_string());
        result.country = json.get("country_name").and_then(|v| v.as_str()).map(|s| s.to_string());
        if let Ok(formatted) = serde_json::to_string_pretty(&json) {
            result.raw_data = formatted;
        }
        result.parsed = true;
    }
}
fn parse_whois_text(result: &mut WhoisResult, data: &str) {
    let lines: Vec<&str> = data.lines().collect();
    for line in lines {
        let line = line.trim();
        if line.to_lowercase().contains("registrar:") {
            if let Some(value) = extract_value_after_colon(line) {
                result.registrar = Some(value);
            }
        }
        if line.to_lowercase().contains("creation date:") ||
           line.to_lowercase().contains("created:") ||
           line.to_lowercase().contains("registered:") {
            if let Some(value) = extract_value_after_colon(line) {
                result.creation_date = Some(value);
            }
        }
        if line.to_lowercase().contains("expiry date:") ||
           line.to_lowercase().contains("expiration date:") ||
           line.to_lowercase().contains("expires:") {
            if let Some(value) = extract_value_after_colon(line) {
                result.expiration_date = Some(value);
            }
        }
        if line.to_lowercase().contains("name server:") ||
           line.to_lowercase().contains("nameserver:") {
            if let Some(value) = extract_value_after_colon(line) {
                result.name_servers.push(value);
            }
        }
        if line.to_lowercase().contains("organization:") ||
           line.to_lowercase().contains("org:") ||
           line.to_lowercase().contains("orgname:") {
            if let Some(value) = extract_value_after_colon(line) {
                result.organization = Some(value);
            }
        }
        if line.to_lowercase().contains("country:") {
            if let Some(value) = extract_value_after_colon(line) {
                result.country = Some(value);
            }
        }
        if line.contains("@") && (
            line.to_lowercase().contains("email:") ||
            line.to_lowercase().contains("abuse") ||
            line.to_lowercase().contains("contact")
        ) {
            if let Some(email) = extract_email_from_line(line) {
                if !result.emails.contains(&email) {
                    result.emails.push(email);
                }
            }
        }
    }
    result.parsed = true;
}
fn extract_value_after_colon(line: &str) -> Option<String> {
    if let Some(pos) = line.find(':') {
        let value = line[pos + 1..].trim();
        if !value.is_empty() && value != "REDACTED FOR PRIVACY" {
            return Some(value.to_string());
        }
    }
    None
}
fn extract_email_from_line(line: &str) -> Option<String> {
    let words: Vec<&str> = line.split_whitespace().collect();
    for word in words {
        if word.contains("@") && word.contains(".") {
            let email = word.trim_end_matches(&[',', '.', ')', ']'][..]);
            if email.matches('@').count() == 1 {
                return Some(email.to_string());
            }
        }
    }
    None
}
fn display_results(result: &WhoisResult, parse: bool) {
    println!("\n{}", "WHOIS Results:".green().bold());
    println!("{}", "══════════════".cyan());
    println!("  {} {}", "Target:".yellow(), result.target.cyan());
    println!("  {} {}", "Type:".yellow(), result.target_type.cyan());
    if parse && result.parsed {
        if let Some(registrar) = &result.registrar {
            println!("  {} {}", "Registrar:".yellow(), registrar.cyan());
        }
        if let Some(creation) = &result.creation_date {
            println!("  {} {}", "Created:".yellow(), creation.cyan());
        }
        if let Some(expiration) = &result.expiration_date {
            println!("  {} {}", "Expires:".yellow(), expiration.cyan());
        }
        if let Some(org) = &result.organization {
            println!("  {} {}", "Organization:".yellow(), org.cyan());
        }
        if let Some(country) = &result.country {
            println!("  {} {}", "Country:".yellow(), country.cyan());
        }
        if !result.name_servers.is_empty() {
            println!("\n{}", "Name Servers:".yellow());
            for ns in &result.name_servers {
                println!("  • {}", ns.cyan());
            }
        }
        if !result.emails.is_empty() {
            println!("\n{}", "Contact Emails:".yellow());
            for email in &result.emails {
                println!("  • {}", email.cyan());
            }
        }
    } else {
        println!("\n{}", "Raw WHOIS Data:".yellow());
        println!("{}", "═══════════════".cyan());
        println!("{}", result.raw_data);
    }
}
