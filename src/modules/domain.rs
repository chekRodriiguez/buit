use crate::cli::DomainArgs;
use anyhow::Result;
use colored::*;
pub async fn run(args: DomainArgs) -> Result<()> {
    println!("{} Domain Analysis: {}", "🌐".cyan(), args.domain.yellow().bold());
    println!("DNS: {}, SSL: {}, WHOIS: {}", args.dns, args.ssl, args.whois);
    Ok(())
}
