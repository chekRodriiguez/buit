use crate::cli::GeoipArgs;
use crate::utils::http::HttpClient;
use anyhow::Result;
use colored::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct GeoIpResult {
    pub ip: String,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub timezone: Option<String>,
    pub isp: Option<String>,
    pub organization: Option<String>,
    pub asn: Option<String>,
    pub threat_level: Option<String>,
}
#[derive(Debug, Deserialize)]
struct IpApiResponse {
    country: Option<String>,
    #[serde(rename = "countryCode")]
    country_code: Option<String>,
    #[serde(rename = "regionName")]
    region: Option<String>,
    city: Option<String>,
    lat: Option<f64>,
    lon: Option<f64>,
    timezone: Option<String>,
    isp: Option<String>,
    org: Option<String>,
    #[serde(rename = "as")]
    asn: Option<String>,
    status: Option<String>,
}
pub async fn run(args: GeoipArgs) -> Result<()> {
    println!("{} GeoIP lookup: {}", "🌍".cyan(), args.ip.yellow().bold());
    let client = HttpClient::new()?;
    let mut result = GeoIpResult {
        ip: args.ip.clone(),
        country: None,
        country_code: None,
        region: None,
        city: None,
        latitude: None,
        longitude: None,
        timezone: None,
        isp: None,
        organization: None,
        asn: None,
        threat_level: None,
    };
    let url = format!("http://ip-api.com/json/{}", args.ip);
    match client.get(&url).await {
        Ok(response) => {
            if let Ok(api_result) = serde_json::from_str::<IpApiResponse>(&response) {
                if api_result.status.as_deref() == Some("success") {
                    result.country = api_result.country;
                    result.country_code = api_result.country_code;
                    result.region = api_result.region;
                    result.city = api_result.city;
                    result.latitude = api_result.lat;
                    result.longitude = api_result.lon;
                    result.timezone = api_result.timezone;
                    if args.isp {
                        result.isp = api_result.isp;
                        result.organization = api_result.org;
                        result.asn = api_result.asn;
                    }
                }
            }
        }
        Err(e) => {
            println!("{} Error fetching data: {}", "⚠".yellow(), e);
        }
    }
    result.threat_level = assess_threat_level(&args.ip);
    display_results(&result);
    Ok(())
}
fn assess_threat_level(ip: &str) -> Option<String> {
    let suspicious_ranges = [
        "10.", "192.168.", "172.16.",
        "127.",
        "0.", "255.",
    ];
    for range in &suspicious_ranges {
        if ip.starts_with(range) {
            return Some("Private/Invalid".to_string());
        }
    }
    Some("Unknown".to_string())
}
fn display_results(result: &GeoIpResult) {
    println!("\n{}", "GeoIP Results:".green().bold());
    println!("{}", "══════════════".cyan());
    println!("  {} {}", "IP Address:".yellow(), result.ip.cyan());
    if let Some(country) = &result.country {
        println!("  {} {}", "Country:".yellow(), country.cyan());
    }
    if let Some(code) = &result.country_code {
        println!("  {} {}", "Country Code:".yellow(), code.cyan());
    }
    if let Some(region) = &result.region {
        println!("  {} {}", "Region:".yellow(), region.cyan());
    }
    if let Some(city) = &result.city {
        println!("  {} {}", "City:".yellow(), city.cyan());
    }
    if let (Some(lat), Some(lon)) = (result.latitude, result.longitude) {
        println!("  {} {}, {}", "Coordinates:".yellow(), lat, lon);
        println!("  {} https://maps.google.com/?q={},{}",
            "Map:".yellow(), lat, lon);
    }
    if let Some(timezone) = &result.timezone {
        println!("  {} {}", "Timezone:".yellow(), timezone.cyan());
    }
    if let Some(isp) = &result.isp {
        println!("\n{}", "ISP Information:".yellow());
        println!("  ISP: {}", isp.cyan());
    }
    if let Some(org) = &result.organization {
        println!("  Organization: {}", org.cyan());
    }
    if let Some(asn) = &result.asn {
        println!("  ASN: {}", asn.cyan());
    }
    if let Some(threat) = &result.threat_level {
        println!("\n{}", "Security Assessment:".yellow());
        let color = match threat.as_str() {
            "Private/Invalid" => "yellow",
            "High" => "red",
            "Medium" => "yellow",
            _ => "green",
        };
        println!("  Threat Level: {}",
            match color {
                "red" => threat.red(),
                "yellow" => threat.yellow(),
                _ => threat.green(),
            });
    }
}
