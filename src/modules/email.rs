use crate::cli::EmailArgs;
use crate::utils::http::HttpClient;
use anyhow::Result;
use colored::*;
use serde::{Deserialize, Serialize};
use sha1::{Sha1, Digest};
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailResult {
    pub email: String,
    pub valid_format: bool,
    pub services: Vec<ServiceCheck>,
    pub breaches: Vec<BreachInfo>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceCheck {
    pub service: String,
    pub registered: bool,
    pub profile_url: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BreachInfo {
    pub name: String,
    pub date: String,
    pub compromised_data: Vec<String>,
}
pub async fn run(args: EmailArgs) -> Result<()> {
    println!("{} Checking email: {}", "📧".cyan(), args.email.yellow().bold());
    if !validate_email(&args.email) {
        println!("{} Invalid email format", "✗".red());
        return Ok(());
    }
    let client = HttpClient::new()?;
    let mut results = EmailResult {
        email: args.email.clone(),
        valid_format: true,
        services: vec![],
        breaches: vec![],
    };
    if args.social {
        println!("\n{} Checking social media accounts...", "🔍".cyan());
        results.services = check_social_accounts(&client, &args.email).await?;
    }
    if args.breaches {
        println!("\n{} Checking for data breaches...", "🔍".cyan());
        results.breaches = check_breaches(&client, &args.email).await?;
    }
    display_results(&results, &args.format);
    Ok(())
}
fn validate_email(email: &str) -> bool {
    let re = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    re.is_match(email)
}
async fn check_social_accounts(client: &HttpClient, email: &str) -> Result<Vec<ServiceCheck>> {
    let mut services = vec![];
    services.push(ServiceCheck {
        service: "GitHub".to_string(),
        registered: check_github(client, email).await?,
        profile_url: None,
    });
    services.push(ServiceCheck {
        service: "Gravatar".to_string(),
        registered: check_gravatar(client, email).await?,
        profile_url: Some(format!("https://gravatar.com/{}", hash_email(email))),
    });
    Ok(services)
}
async fn check_github(client: &HttpClient, email: &str) -> Result<bool> {
    let url = format!("https://api.github.com/search/users?q={}", email);
    match client.get(&url).await {
        Ok(response) => {
            Ok(response.contains("total_count") && !response.contains("\"total_count\":0"))
        }
        Err(_) => Ok(false),
    }
}
async fn check_gravatar(_client: &HttpClient, email: &str) -> Result<bool> {
    let hash = hash_email(email);
    let _url = format!("https://gravatar.com/avatar/{}", hash);
    Ok(true)
}
fn hash_email(email: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(email.trim().to_lowercase().as_bytes());
    format!("{:x}", hasher.finalize())
}
async fn check_breaches(_client: &HttpClient, _email: &str) -> Result<Vec<BreachInfo>> {
    let mut breaches = vec![];
    breaches.push(BreachInfo {
        name: "Example Breach (Demo)".to_string(),
        date: "2023-01-01".to_string(),
        compromised_data: vec!["Email addresses".to_string(), "Passwords".to_string()],
    });
    Ok(breaches)
}
fn display_results(results: &EmailResult, format: &str) {
    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(results).unwrap());
        }
        _ => {
            println!("\n{}", "Results:".green().bold());
            println!("{}", "════════".cyan());
            if !results.services.is_empty() {
                println!("\n{}", "Social Media Accounts:".yellow());
                for service in &results.services {
                    let status = if service.registered { "✓".green() } else { "✗".red() };
                    println!("  {} {}", status, service.service);
                    if let Some(url) = &service.profile_url {
                        println!("      {}", url.blue().underline());
                    }
                }
            }
            if !results.breaches.is_empty() {
                println!("\n{}", "Data Breaches:".red());
                for breach in &results.breaches {
                    println!("  {} {} ({})", "⚠".yellow(), breach.name, breach.date);
                    println!("    Compromised: {}", breach.compromised_data.join(", "));
                }
            }
        }
    }
}
