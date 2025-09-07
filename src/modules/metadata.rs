use crate::cli::MetadataArgs;
use anyhow::Result;
use colored::*;
pub fn run(args: MetadataArgs) -> Result<()> {
    println!("{} Extracting metadata from: {}", "📄".cyan(), args.file.yellow().bold());
    Ok(())
}
