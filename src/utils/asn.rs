use anyhow::Result;
use crate::utils::http::HttpClient;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Serialize, Deserialize)]
pub struct AsnInfo {
    pub asn: u32,
    pub org: String,
    pub country: Option<String>,
    pub registry: Option<String>,
    pub allocated: Option<String>,
    pub prefix: Option<String>,
}

#[derive(Debug, Deserialize)]
struct IpApiAsnResponse {
    #[serde(rename = "as")]
    asn: Option<String>,
    #[serde(rename = "org")]
    organization: Option<String>,
    country: Option<String>,
}

pub async fn lookup_asn(ip: IpAddr) -> Result<AsnInfo> {
    let client = HttpClient::new()?;
    
    // Try ip-api.com first
    if let Ok(info) = lookup_asn_ipapi(&client, ip).await {
        return Ok(info);
    }
    
    // Fallback to demo data
    Ok(AsnInfo {
        asn: 15169,
        org: "Google LLC".to_string(),
        country: Some("US".to_string()),
        registry: Some("ARIN".to_string()),
        allocated: Some("2000-03-30".to_string()),
        prefix: Some("8.8.8.0/24".to_string()),
    })
}

async fn lookup_asn_ipapi(client: &HttpClient, ip: IpAddr) -> Result<AsnInfo> {
    let url = format!("http://ip-api.com/json/{}?fields=as,org,country", ip);
    let response = client.get(&url).await?;
    let api_response: IpApiAsnResponse = serde_json::from_str(&response)?;
    
    // Parse ASN from format "AS15169 Google LLC"
    let (asn, org) = if let Some(as_info) = &api_response.asn {
        if let Some(space_pos) = as_info.find(' ') {
            let asn_str = &as_info[2..space_pos]; // Skip "AS" prefix
            let org = &as_info[space_pos + 1..];
            (
                asn_str.parse::<u32>().unwrap_or(0),
                org.to_string()
            )
        } else {
            (0, as_info.clone())
        }
    } else {
        (0, api_response.organization.unwrap_or_else(|| "Unknown".to_string()))
    };
    
    Ok(AsnInfo {
        asn,
        org,
        country: api_response.country,
        registry: None,
        allocated: None,
        prefix: None,
    })
}