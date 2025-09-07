use crate::cli::ReportArgs;
use anyhow::Result;
use colored::*;
pub fn run(args: ReportArgs) -> Result<()> {
    println!("{} Generating report: {}", "📊".cyan(), args.title.yellow().bold());
    Ok(())
}
