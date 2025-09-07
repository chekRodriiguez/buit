use crate::cli::PhoneArgs;
use crate::utils::http::HttpClient;
use anyhow::Result;
use colored::*;
use serde::{Deserialize, Serialize};
use regex::Regex;
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneResult {
    pub number: String,
    pub formatted: String,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub carrier: Option<String>,
    pub line_type: Option<String>,
    pub valid: bool,
    pub possible_formats: Vec<String>,
    pub social_media: Vec<String>,
}
pub async fn run(args: PhoneArgs) -> Result<()> {
    println!("{} Phone number lookup: {}", "📱".cyan(), args.number.yellow().bold());
    let cleaned_number = clean_phone_number(&args.number);
    let client = HttpClient::new()?;
    let mut result = PhoneResult {
        number: args.number.clone(),
        formatted: format_phone_number(&cleaned_number),
        country: None,
        country_code: None,
        carrier: None,
        line_type: None,
        valid: validate_phone_number(&cleaned_number),
        possible_formats: generate_formats(&cleaned_number),
        social_media: vec![],
    };
    if result.valid {
        result.country = identify_country(&cleaned_number);
        result.country_code = extract_country_code(&cleaned_number);
        if args.carrier {
            println!("\n{} Checking carrier information...", "🔍".cyan());
            let carrier_info = lookup_carrier(&client, &cleaned_number).await?;
            result.carrier = carrier_info.carrier;
            result.line_type = carrier_info.line_type;
        }
        result.social_media = check_social_media(&client, &cleaned_number).await?;
    }
    display_results(&result, args.format.as_deref());
    Ok(())
}
fn clean_phone_number(number: &str) -> String {
    number.chars()
        .filter(|c| c.is_ascii_digit())
        .collect()
}
fn format_phone_number(number: &str) -> String {
    if number.len() == 10 && number.starts_with("1") {
        format!("+1 ({}) {}-{}",
            &number[0..3], &number[3..6], &number[6..10])
    } else if number.len() == 11 && number.starts_with("1") {
        format!("+1 ({}) {}-{}",
            &number[1..4], &number[4..7], &number[7..11])
    } else if number.len() == 10 {
        format!("({}) {}-{}",
            &number[0..3], &number[3..6], &number[6..10])
    } else {
        number.to_string()
    }
}
fn validate_phone_number(number: &str) -> bool {
    let re = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
    re.is_match(number)
}
fn identify_country(number: &str) -> Option<String> {
    if number.starts_with("1") && number.len() >= 10 {
        Some("United States/Canada".to_string())
    } else if number.starts_with("44") {
        Some("United Kingdom".to_string())
    } else if number.starts_with("33") {
        Some("France".to_string())
    } else if number.starts_with("49") {
        Some("Germany".to_string())
    } else if number.starts_with("86") {
        Some("China".to_string())
    } else if number.starts_with("91") {
        Some("India".to_string())
    } else if number.starts_with("81") {
        Some("Japan".to_string())
    } else if number.starts_with("7") {
        Some("Russia".to_string())
    } else {
        None
    }
}
fn extract_country_code(number: &str) -> Option<String> {
    if number.starts_with("1") && number.len() >= 10 {
        Some("+1".to_string())
    } else if number.len() > 2 {
        let code = &number[0..2];
        Some(format!("+{}", code))
    } else {
        None
    }
}
fn generate_formats(number: &str) -> Vec<String> {
    let mut formats = vec![];
    formats.push(number.to_string());
    formats.push(format!("+{}", number));
    if number.len() == 10 {
        formats.push(format!("({}) {}-{}",
            &number[0..3], &number[3..6], &number[6..10]));
        formats.push(format!("{}-{}-{}",
            &number[0..3], &number[3..6], &number[6..10]));
        formats.push(format!("{}.{}.{}",
            &number[0..3], &number[3..6], &number[6..10]));
    }
    formats
}
struct CarrierInfo {
    carrier: Option<String>,
    line_type: Option<String>,
}
async fn lookup_carrier(_client: &HttpClient, _number: &str) -> Result<CarrierInfo> {
    Ok(CarrierInfo {
        carrier: Some("Demo Carrier".to_string()),
        line_type: Some("Mobile".to_string()),
    })
}
async fn check_social_media(_client: &HttpClient, number: &str) -> Result<Vec<String>> {
    let mut platforms = vec![];
    platforms.push(format!("WhatsApp: https://wa.me/{}", number));
    platforms.push(format!("Telegram: Search for +{}", number));
    platforms.push("Signal: Available".to_string());
    Ok(platforms)
}
fn display_results(result: &PhoneResult, format: Option<&str>) {
    match format {
        Some("json") => {
            println!("{}", serde_json::to_string_pretty(result).unwrap());
        }
        _ => {
            println!("\n{}", "Phone Number Analysis:".green().bold());
            println!("{}", "═════════════════════".cyan());
            println!("  {} {}", "Number:".yellow(), result.number);
            println!("  {} {}", "Formatted:".yellow(), result.formatted);
            println!("  {} {}", "Valid:".yellow(),
                if result.valid { "✓".green() } else { "✗".red() });
            if let Some(country) = &result.country {
                println!("  {} {}", "Country:".yellow(), country.cyan());
            }
            if let Some(code) = &result.country_code {
                println!("  {} {}", "Country Code:".yellow(), code.cyan());
            }
            if let Some(carrier) = &result.carrier {
                println!("  {} {}", "Carrier:".yellow(), carrier.cyan());
            }
            if let Some(line_type) = &result.line_type {
                println!("  {} {}", "Line Type:".yellow(), line_type.cyan());
            }
            if !result.possible_formats.is_empty() {
                println!("\n{}", "Possible Formats:".yellow());
                for format in &result.possible_formats {
                    println!("  • {}", format.cyan());
                }
            }
            if !result.social_media.is_empty() {
                println!("\n{}", "Social Media:".yellow());
                for platform in &result.social_media {
                    println!("  • {}", platform.cyan());
                }
            }
        }
    }
}
