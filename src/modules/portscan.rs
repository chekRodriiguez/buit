use crate::cli::PortscanArgs;
use anyhow::Result;
use colored::*;
pub async fn run(args: PortscanArgs) -> Result<()> {
    println!("{} Port scanning: {}", "🔍".cyan(), args.target.yellow().bold());
    Ok(())
}
