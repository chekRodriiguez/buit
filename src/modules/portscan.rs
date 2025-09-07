use crate::cli::PortscanArgs;
use crate::config::Config;
use anyhow::Result;
use colored::*;
use futures::future::join_all;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

pub async fn run(args: PortscanArgs) -> Result<()> {
    println!("{} Port scanning: {}", "🔍".cyan(), args.target.yellow().bold());
    
    let config = Config::load()?;
    let thread_count = config.settings.max_threads;
    
    let ports = parse_port_range(&args.ports.unwrap_or_else(|| "1-1000".to_string()))?;
    let scan_type = args.scan_type.as_deref().unwrap_or("tcp");
    
    println!("{} Scanning {} ports with {} threads", "⚡".yellow(), ports.len(), thread_count);
    
    let target_ip = resolve_target(&args.target).await?;
    let open_ports = match scan_type {
        "tcp" => tcp_scan(&target_ip, &ports, thread_count).await?,
        "udp" => {
            println!("{} UDP scanning not implemented yet", "⚠️".yellow());
            Vec::new()
        }
        _ => {
            println!("{} Unknown scan type: {}", "❌".red(), scan_type);
            return Ok(());
        }
    };
    
    display_results(&args.target, &open_ports);
    Ok(())
}

fn parse_port_range(port_str: &str) -> Result<Vec<u16>> {
    let mut ports = Vec::new();
    
    for part in port_str.split(',') {
        if part.contains('-') {
            let range: Vec<&str> = part.split('-').collect();
            if range.len() == 2 {
                let start: u16 = range[0].parse()?;
                let end: u16 = range[1].parse()?;
                for port in start..=end {
                    ports.push(port);
                }
            }
        } else {
            ports.push(part.parse()?);
        }
    }
    
    Ok(ports)
}

async fn resolve_target(target: &str) -> Result<IpAddr> {
    if let Ok(ip) = target.parse::<IpAddr>() {
        return Ok(ip);
    }
    
    let addrs: Vec<SocketAddr> = format!("{}:80", target).to_socket_addrs()?.collect();
    if let Some(addr) = addrs.first() {
        Ok(addr.ip())
    } else {
        Err(anyhow::anyhow!("Could not resolve target: {}", target))
    }
}

async fn tcp_scan(target_ip: &IpAddr, ports: &[u16], max_concurrent: usize) -> Result<Vec<u16>> {
    let mut open_ports = Vec::new();
    let mut _tasks: Vec<()> = Vec::new();
    
    for chunk in ports.chunks(max_concurrent) {
        let chunk_tasks: Vec<_> = chunk.iter().map(|&port| {
            let addr = SocketAddr::new(*target_ip, port);
            async move {
                match timeout(Duration::from_millis(1000), TcpStream::connect(addr)).await {
                    Ok(Ok(_)) => Some(port),
                    _ => None,
                }
            }
        }).collect();
        
        let results = join_all(chunk_tasks).await;
        for result in results {
            if let Some(port) = result {
                open_ports.push(port);
            }
        }
        
        print!(".");
        tokio::task::yield_now().await;
    }
    
    println!();
    Ok(open_ports)
}

fn display_results(target: &str, open_ports: &[u16]) {
    println!("\n{} Scan Results for {}", "📋".cyan(), target.yellow().bold());
    println!("{}", "=".repeat(50));
    
    if open_ports.is_empty() {
        println!("{} No open ports found", "❌".red());
        return;
    }
    
    println!("{} Found {} open ports:", "✅".green(), open_ports.len());
    
    for &port in open_ports {
        let service = get_common_service(port);
        println!("  {} {}/tcp  {}", "🔓".green(), port, service);
    }
}

fn get_common_service(port: u16) -> &'static str {
    match port {
        21 => "ftp",
        22 => "ssh", 
        23 => "telnet",
        25 => "smtp",
        53 => "dns",
        80 => "http",
        110 => "pop3",
        143 => "imap",
        443 => "https",
        993 => "imaps",
        995 => "pop3s",
        3389 => "rdp",
        5432 => "postgresql",
        3306 => "mysql",
        1433 => "mssql",
        6379 => "redis",
        27017 => "mongodb",
        _ => "unknown"
    }
}