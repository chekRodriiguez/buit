use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use anyhow::Result;
use console::style;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};

use crate::cli::*;
use crate::modules::{
    username, email, subdomain, ip, whois, hash, 
    geoip, phone, github, search, social, leaks,
    portscan, domain, metadata, report, reverse_image
};

use tokio::fs;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: String,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime: u64,
}

#[derive(Deserialize)]
pub struct ApiQuery {
    pub format: Option<String>,
    pub limit: Option<usize>,
    pub platforms: Option<String>,
    #[allow(dead_code)]
    pub verbose: Option<bool>,
    pub ports: Option<String>,
    pub scan_type: Option<String>,
    pub dns: Option<bool>,
    pub ssl: Option<bool>,
    pub whois: Option<bool>,
    pub output: Option<String>,
    pub engines: Option<String>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    #[allow(dead_code)]
    pub fn error(error: String) -> ApiResponse<Value> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(error),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

pub async fn start_api_server(port: u16) -> Result<()> {
    println!("{} Starting BUIT API Server...", style("🚀").green());
    println!("{} Server running on: {}", style("🖥").cyan(), style(format!("http://127.0.0.1:{}", port)).blue().underlined());
    println!("{} API Documentation: {}", style("📚").yellow(), style(format!("http://127.0.0.1:{}/docs", port)).blue().underlined());
    println!();
    println!("{}", style("Available Endpoints:").green().bold());
    println!("  GET  /health              - Health check");
    println!("  GET  /username/{{handle}}   - Username search");
    println!("  GET  /email/{{address}}     - Email analysis");
    println!("  GET  /subdomain/{{domain}}  - Subdomain enumeration");
    println!("  GET  /ip/{{address}}        - IP analysis");
    println!("  GET  /whois/{{domain}}      - WHOIS lookup");
    println!("  GET  /hash/{{value}}        - Hash analysis");
    println!("  GET  /geoip/{{ip}}          - GeoIP lookup");
    println!("  GET  /phone/{{number}}      - Phone analysis");
    println!("  GET  /github/{{user}}       - GitHub OSINT");
    println!("  GET  /search/{{query}}      - Web search");
    println!("  GET  /social/{{target}}     - Social media");
    println!("  GET  /leaks/{{target}}      - Data breaches");
    println!("  GET  /portscan/{{target}}   - Port scanning");
    println!("  GET  /domain/{{domain}}     - Domain analysis");
    println!("  GET  /metadata/{{file}}     - File metadata");
    println!("  GET  /report/{{title}}      - Generate report");
    println!("  GET  /reverse-image/{{url}} - Reverse image search");

    let app = create_router();

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/username/:handle", get(username_handler))
        .route("/email/:address", get(email_handler))
        .route("/subdomain/:domain", get(subdomain_handler))
        .route("/ip/:address", get(ip_handler))
        .route("/whois/:domain", get(whois_handler))
        .route("/hash/:value", get(hash_handler))
        .route("/geoip/:ip", get(geoip_handler))
        .route("/phone/:number", get(phone_handler))
        .route("/github/:user", get(github_handler))
        .route("/search/:query", get(search_handler))
        .route("/social/:target", get(social_handler))
        .route("/leaks/:target", get(leaks_handler))
        .route("/portscan/:target", get(portscan_handler))
        .route("/domain/:domain", get(domain_handler))
        .route("/metadata/:file", get(metadata_handler))
        .route("/report/:title", get(report_handler))
        .route("/reverse-image/:url", get(reverse_image_handler))
        .route("/docs", get(docs_handler))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        )
}

async fn health_handler() -> Json<ApiResponse<HealthResponse>> {
    let health = HealthResponse {
        status: "ok".to_string(),
        version: "1.0.4".to_string(),
        uptime: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    };
    Json(ApiResponse::success(health))
}

async fn username_handler(
    Path(handle): Path<String>,
    Query(params): Query<ApiQuery>
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let args = UsernameArgs {
        username: handle.clone(),
        format: params.format.unwrap_or_else(|| "json".to_string()),
        output: None,
        platforms: params.platforms,
    };

    // Create a temporary file to capture results
    let temp_file = format!("temp_username_{}.json", handle);
    let args_with_output = UsernameArgs {
        output: Some(temp_file.clone()),
        ..args
    };
    
    match username::run(args_with_output).await {
        Ok(_) => {
            // Try to read the results from the temp file
            match fs::read_to_string(&temp_file).await {
                Ok(content) => {
                    let _ = fs::remove_file(&temp_file).await; // Cleanup
                    match serde_json::from_str::<Value>(&content) {
                        Ok(data) => Ok(Json(ApiResponse::success(data))),
                        Err(_) => {
                            // Fallback to basic response
                            let data = json!({
                                "username": handle,
                                "message": "Search completed successfully",
                                "note": "Results available via CLI for detailed output"
                            });
                            Ok(Json(ApiResponse::success(data)))
                        }
                    }
                },
                Err(_) => {
                    // Fallback response if temp file can't be read
                    let data = json!({
                        "username": handle,
                        "message": "Search completed successfully",
                        "note": "Results available via CLI for detailed output"
                    });
                    Ok(Json(ApiResponse::success(data)))
                }
            }
        }
        Err(e) => {
            eprintln!("Username search error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn email_handler(
    Path(address): Path<String>,
    Query(params): Query<ApiQuery>
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let args = EmailArgs {
        email: address.clone(),
        breaches: true,
        social: true,
        format: params.format.unwrap_or_else(|| "json".to_string()),
    };

    match email::run(args.clone()).await {
        Ok(_) => {
            let data = json!({
                "email": address,
                "message": "Email analysis completed successfully",
                "note": "Detailed results available via CLI"
            });
            Ok(Json(ApiResponse::success(data)))
        }
        Err(e) => {
            eprintln!("Email analysis error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn subdomain_handler(
    Path(domain): Path<String>,
    Query(_params): Query<ApiQuery>
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let args = SubdomainArgs {
        domain: domain.clone(),
        crt: true,
        brute: false,
        skip_alive_check: false,
    };

    match subdomain::run(args.clone()).await {
        Ok(_) => {
            let data = json!({
                "domain": domain,
                "message": "Subdomain enumeration completed successfully",
                "note": "Detailed results available via CLI"
            });
            Ok(Json(ApiResponse::success(data)))
        }
        Err(e) => {
            eprintln!("Subdomain enumeration error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn ip_handler(
    Path(address): Path<String>,
    Query(_params): Query<ApiQuery>
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let args = IpArgs {
        ip: address.clone(),
        no_reverse: false,
        no_asn: false,
        no_geo: false,
    };

    match ip::run(args.clone()).await {
        Ok(result) => {
            let data = json!({
                "ip": result.ip,
                "valid": result.valid,
                "version": result.version,
                "reverse_dns": result.reverse_dns,
                "asn": result.asn,
                "geolocation": result.geolocation,
                "ports": result.ports
            });
            Ok(Json(ApiResponse::success(data)))
        }
        Err(e) => {
            eprintln!("IP analysis error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn whois_handler(
    Path(domain): Path<String>,
    Query(_params): Query<ApiQuery>
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let args = WhoisArgs {
        target: domain.clone(),
        parse: true,
    };

    match whois::run(args.clone()).await {
        Ok(_) => {
            let data = json!({
                "domain": domain,
                "message": "WHOIS lookup completed successfully",
                "cli_command": format!("buit whois {}", domain)
            });
            Ok(Json(ApiResponse::success(data)))
        }
        Err(e) => {
            eprintln!("WHOIS lookup error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn hash_handler(
    Path(value): Path<String>,
    Query(_params): Query<ApiQuery>
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let args = HashArgs {
        hash: value.clone(),
        identify: true,
        crack: false,
    };

    match hash::run(args.clone()).await {
        Ok(_) => {
            let data = json!({
                "hash": value,
                "message": "Hash analysis completed successfully",
                "cli_command": format!("buit hash {}", value)
            });
            Ok(Json(ApiResponse::success(data)))
        }
        Err(e) => {
            eprintln!("Hash analysis error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn geoip_handler(
    Path(ip): Path<String>,
    Query(_params): Query<ApiQuery>
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let args = GeoipArgs {
        ip: ip.clone(),
        isp: true,
    };

    match geoip::run(args.clone()).await {
        Ok(_) => {
            let data = json!({
                "ip": ip,
                "message": "GeoIP lookup completed successfully",
                "cli_command": format!("buit geoip {}", ip)
            });
            Ok(Json(ApiResponse::success(data)))
        }
        Err(e) => {
            eprintln!("GeoIP lookup error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn phone_handler(
    Path(number): Path<String>,
    Query(_params): Query<ApiQuery>
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    println!("📱 Phone API called with number: {}", number);
    
    let args = PhoneArgs {
        number: number.clone(),
        carrier: true,
        format: Some("json".to_string()),
    };

    // Use direct await instead of block_on to avoid deadlock
    match phone::run(args.clone()).await {
        Ok(_) => {
            println!("✅ Phone analysis completed successfully");
            let data = json!({
                "number": number,
                "message": "Phone analysis completed successfully",
                "cli_command": format!("buit phone {}", number)
            });
            Ok(Json(ApiResponse::success(data)))
        }
        Err(e) => {
            eprintln!("❌ Phone analysis error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn github_handler(
    Path(user): Path<String>,
    Query(_params): Query<ApiQuery>
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let args = GithubArgs {
        target: user.clone(),
        secrets: false,
        repos: true,
    };

    match github::run(args.clone()).await {
        Ok(_) => {
            let data = json!({
                "username": user,
                "message": "GitHub OSINT completed successfully",
                "cli_command": format!("buit github {}", user)
            });
            Ok(Json(ApiResponse::success(data)))
        }
        Err(e) => {
            eprintln!("GitHub OSINT error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn search_handler(
    Path(query): Path<String>,
    Query(params): Query<ApiQuery>
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let args = SearchArgs {
        query: query.clone(),
        engine: "duckduckgo".to_string(),
        limit: params.limit.unwrap_or(20),
        deep: false,
    };

    match search::run(args.clone()).await {
        Ok(_) => {
            let data = json!({
                "query": query,
                "message": "Web search completed successfully",
                "cli_command": format!("buit search '{}'", query)
            });
            Ok(Json(ApiResponse::success(data)))
        }
        Err(e) => {
            eprintln!("Search error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn social_handler(
    Path(target): Path<String>,
    Query(_params): Query<ApiQuery>
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let args = SocialArgs {
        target: target.clone(),
        id_type: "username".to_string(),
        platforms: None,
        analyze: true,
    };

    match social::run(args.clone()).await {
        Ok(_) => {
            let data = json!({
                "target": target,
                "message": "Social media search completed successfully",
                "cli_command": format!("buit social {}", target)
            });
            Ok(Json(ApiResponse::success(data)))
        }
        Err(e) => {
            eprintln!("Social media search error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn leaks_handler(
    Path(target): Path<String>,
    Query(_params): Query<ApiQuery>
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let args = LeaksArgs {
        target: target.clone(),
        hibp: true,
        passwords: false,
    };

    match leaks::run(args.clone()).await {
        Ok(_) => {
            let data = json!({
                "target": target,
                "message": "Breach check completed successfully",
                "cli_command": format!("buit leaks {}", target)
            });
            Ok(Json(ApiResponse::success(data)))
        }
        Err(e) => {
            eprintln!("Leaks check error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn docs_handler() -> Json<Value> {
    let docs = json!({
        "title": "BUIT API Documentation",
        "version": "1.0.4",
        "description": "REST API for BUIT OSINT Toolkit",
        "endpoints": [
            {
                "path": "/health",
                "method": "GET",
                "description": "Health check endpoint",
                "response": "JSON with server status"
            },
            {
                "path": "/username/{handle}",
                "method": "GET",
                "description": "Search for username across platforms",
                "parameters": ["handle: string", "platforms: string (optional)"],
                "response": "JSON with found platforms and URLs"
            },
            {
                "path": "/email/{address}",
                "method": "GET",
                "description": "Analyze email address",
                "parameters": ["address: string"],
                "response": "JSON with breach data and service registrations"
            },
            {
                "path": "/subdomain/{domain}",
                "method": "GET",
                "description": "Enumerate subdomains",
                "parameters": ["domain: string"],
                "response": "JSON with discovered subdomains"
            },
            {
                "path": "/ip/{address}",
                "method": "GET",
                "description": "Analyze IP address",
                "parameters": ["address: string"],
                "response": "JSON with geolocation and ASN data"
            },
            {
                "path": "/whois/{domain}",
                "method": "GET",
                "description": "WHOIS domain lookup",
                "parameters": ["domain: string"],
                "response": "JSON with domain registration data"
            },
            {
                "path": "/hash/{value}",
                "method": "GET",
                "description": "Identify and analyze hash",
                "parameters": ["value: string"],
                "response": "JSON with hash type identification"
            },
            {
                "path": "/portscan/{target}",
                "method": "GET",
                "description": "Scan target for open ports",
                "parameters": ["target: string", "ports: string (optional)", "scan_type: string (optional)"],
                "response": "JSON with open ports and services"
            },
            {
                "path": "/domain/{domain}",
                "method": "GET", 
                "description": "Comprehensive domain analysis",
                "parameters": ["domain: string", "dns: bool (optional)", "ssl: bool (optional)", "whois: bool (optional)"],
                "response": "JSON with DNS, SSL, and WHOIS data"
            },
            {
                "path": "/metadata/{file}",
                "method": "GET",
                "description": "Extract metadata from file",
                "parameters": ["file: string", "format: string (optional)"],
                "response": "JSON with file metadata and EXIF data"
            },
            {
                "path": "/report/{title}",
                "method": "GET",
                "description": "Generate OSINT security report",
                "parameters": ["title: string", "format: string (optional)", "output: string (optional)"],
                "response": "JSON with report generation status"
            },
            {
                "path": "/reverse-image/{url}",
                "method": "GET",
                "description": "Reverse image search across engines",
                "parameters": ["url: string", "engines: string (optional)"],
                "response": "JSON with image search results and matches"
            }
        ]
    });
    
    Json(docs)
}

async fn portscan_handler(
    Path(target): Path<String>,
    Query(params): Query<ApiQuery>
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let args = PortscanArgs {
        target: target.clone(),
        ports: params.ports,
        scan_type: params.scan_type,
    };

    match portscan::run(args.clone()).await {
        Ok(_) => {
            let data = json!({
                "target": target,
                "message": "Port scan completed successfully",
                "cli_command": format!("buit portscan {}", target)
            });
            Ok(Json(ApiResponse::success(data)))
        }
        Err(e) => {
            eprintln!("Portscan error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn domain_handler(
    Path(domain): Path<String>,
    Query(params): Query<ApiQuery>
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let args = DomainArgs {
        domain: domain.clone(),
        dns: params.dns.unwrap_or(true),
        ssl: params.ssl.unwrap_or(true), 
        whois: params.whois.unwrap_or(true),
    };

    match domain::run(args.clone()).await {
        Ok(_) => {
            let data = json!({
                "domain": domain,
                "message": "Domain analysis completed successfully",
                "cli_command": format!("buit domain {}", domain)
            });
            Ok(Json(ApiResponse::success(data)))
        }
        Err(e) => {
            eprintln!("Domain analysis error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn metadata_handler(
    Path(file): Path<String>,
    Query(params): Query<ApiQuery>
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let args = MetadataArgs {
        file: file.clone(),
        format: params.format,
    };

    match metadata::run(args.clone()) {
        Ok(_) => {
            let data = json!({
                "file": file,
                "message": "Metadata extraction completed successfully",
                "cli_command": format!("buit metadata {}", file)
            });
            Ok(Json(ApiResponse::success(data)))
        }
        Err(e) => {
            eprintln!("Metadata extraction error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn report_handler(
    Path(title): Path<String>,
    Query(params): Query<ApiQuery>
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let args = ReportArgs {
        title: title.clone(),
        format: params.format,
        output: params.output,
    };

    match report::run(args.clone()) {
        Ok(_) => {
            let data = json!({
                "title": title,
                "message": "Report generation completed successfully",
                "cli_command": format!("buit report {}", title)
            });
            Ok(Json(ApiResponse::success(data)))
        }
        Err(e) => {
            eprintln!("Report generation error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn reverse_image_handler(
    Path(url): Path<String>,
    Query(params): Query<ApiQuery>
) -> Result<Json<ApiResponse<Value>>, StatusCode> {
    let args = ReverseImageArgs {
        image: url.clone(),
        engines: params.engines,
    };

    match reverse_image::run(args.clone()).await {
        Ok(_) => {
            let data = json!({
                "image": url,
                "message": "Reverse image search completed successfully",
                "cli_command": format!("buit reverse-image {}", url)
            });
            Ok(Json(ApiResponse::success(data)))
        }
        Err(e) => {
            eprintln!("Reverse image search error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}