use crate::cli::ReverseImageArgs;
use anyhow::Result;
use colored::*;
pub async fn run(args: ReverseImageArgs) -> Result<()> {
    println!("{} Reverse image search: {}", "🔍".cyan(), args.image.yellow().bold());
    Ok(())
}
