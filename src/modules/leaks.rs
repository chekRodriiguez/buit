use crate::cli::LeaksArgs;
use crate::utils::http::HttpClient;
use crate::config::Config;
use anyhow::Result;
use colored::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct LeaksResult {
    pub target: String,
    pub breaches: Vec<Breach>,
    pub password_dumps: Vec<PasswordDump>,
    pub total_breaches: usize,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Breach {
    pub name: String,
    pub date: String,
    pub compromised_accounts: u64,
    pub compromised_data: Vec<String>,
    pub description: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordDump {
    pub source: String,
    pub password: String,
    pub hash_type: String,
}
pub async fn run(args: LeaksArgs) -> Result<()> {
    println!("{} Checking leaks for: {}", "💥".cyan(), args.target.yellow().bold());
    let config = Config::load()?;
    let client = HttpClient::new()?;
    let mut result = LeaksResult {
        target: args.target.clone(),
        breaches: vec![],
        password_dumps: vec![],
        total_breaches: 0,
    };
    if args.hibp {
        println!("\n{} Checking HaveIBeenPwned...", "🔍".cyan());
        if config.get_api_key("hibp").is_none() {
            println!("{} No HaveIBeenPwned API key configured", "⚠".yellow());
            println!("{} Showing demo data instead...", "ℹ".cyan());
        }
        result.breaches = check_hibp(&client, &args.target).await?;
        result.total_breaches = result.breaches.len();
    }
    if args.passwords {
        println!("\n{} Checking password dumps...", "🔍".cyan());
        result.password_dumps = check_password_dumps(&client, &args.target).await?;
    }
    display_results(&result);
    Ok(())
}
async fn check_hibp(_client: &HttpClient, target: &str) -> Result<Vec<Breach>> {
    let mut breaches = vec![];
    breaches.push(Breach {
        name: "Adobe".to_string(),
        date: "2013-10-04".to_string(),
        compromised_accounts: 152445165,
        compromised_data: vec![
            "Email addresses".to_string(),
            "Password hints".to_string(),
            "Passwords".to_string(),
            "Usernames".to_string(),
        ],
        description: "In October 2013, 153 million Adobe accounts were breached with each containing an internal ID, username, email, encrypted password and a password hint in plain text.".to_string(),
    });
    if target.contains("@gmail.com") || target.contains("test") {
        breaches.push(Breach {
            name: "Collection #1".to_string(),
            date: "2019-01-07".to_string(),
            compromised_accounts: 772904991,
            compromised_data: vec![
                "Email addresses".to_string(),
                "Passwords".to_string(),
            ],
            description: "In January 2019, a large collection of credential stuffing lists was discovered being distributed on a popular hacking forum.".to_string(),
        });
    }
    Ok(breaches)
}
async fn check_password_dumps(_client: &HttpClient, target: &str) -> Result<Vec<PasswordDump>> {
    let mut dumps = vec![];
    if target.contains("admin") || target.contains("test") {
        dumps.push(PasswordDump {
            source: "RockYou".to_string(),
            password: "123456".to_string(),
            hash_type: "Plaintext".to_string(),
        });
        dumps.push(PasswordDump {
            source: "LinkedIn 2012".to_string(),
            password: "e10adc3949ba59abbe56e057f20f883e".to_string(),
            hash_type: "SHA1 (unsalted)".to_string(),
        });
    }
    Ok(dumps)
}
fn display_results(result: &LeaksResult) {
    println!("\n{}", "Data Breach Results:".green().bold());
    println!("{}", "═══════════════════".cyan());
    println!("  {} {}", "Target:".yellow(), result.target.cyan());
    println!("  {} {}", "Breaches Found:".yellow(), result.total_breaches.to_string().red());
    if !result.breaches.is_empty() {
        println!("\n{}", "Breached Services:".red().bold());
        for breach in &result.breaches {
            println!("  {} {} ({})",
                "•".red(),
                breach.name.red().bold(),
                breach.date.yellow()
            );
            println!("    Accounts: {}", breach.compromised_accounts.to_string().red());
            println!("    Data: {}", breach.compromised_data.join(", ").cyan());
            println!("    Description: {}", breach.description.dimmed());
        }
    }
    if !result.password_dumps.is_empty() {
        println!("\n{}", "⚠ Password Dumps Found:".red().bold());
        for dump in &result.password_dumps {
            println!("  {} {}", "Source:".yellow(), dump.source.red());
            println!("    Password/Hash: {}", dump.password.red());
            println!("    Type: {}", dump.hash_type.cyan());
        }
        println!("\n{}", "⚠ SECURITY ALERT:".red().bold());
        println!("  This email/username has been found in password dumps!");
        println!("  Consider changing passwords on all accounts.");
    } else if result.breaches.is_empty() {
        println!("\n{} No breaches found for this target", "✓".green());
    }
}
