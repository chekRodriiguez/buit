mod cli;
mod config;
mod modules;
mod utils;
mod setup;
mod assets;
use anyhow::Result;
use clap::Parser;
use colored::*;
use tracing_subscriber;
use figlet_rs::FIGfont;
fn init_terminal() {
    #[cfg(windows)]
    {
        let _ = enable_ansi_support::enable_ansi_support();
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    init_terminal();
    tracing_subscriber::fmt::init();
    let standard_font = FIGfont::standard().unwrap();
    let buit_text = standard_font.convert("BUIT");
    if let Some(text) = buit_text {
        println!("{}", text.to_string().magenta().bold());
    }
    println!("{}", "╔═══════════════════════════════════════════════╗".cyan().bold());
    println!("{}", "║          Buu Undercover Intelligence Toolkit ║".cyan().bold());
    println!("{}", "║           Advanced OSINT Security Framework  ║".green().bold());
    println!("{}", "║          For Authorized Security Testing Only ║".yellow());
    println!("{}", "╚═══════════════════════════════════════════════╝".cyan().bold());
    println!("");
    println!("{} {} {}", "📧".red(), "Copyright ©".white(), "BuuDevOff - Open-Source Project".cyan().bold());
    println!("{} {} {}", "🌟".yellow(), "Like this tool? Star the repo:".white(), "https://github.com/BuuDevOff/BUIT".blue().underline());
    println!("{} {} {}", "🚀".green(), "Share with the community &".white(), "contribute!".green().bold());
    println!("{} {} {}", "💡".yellow(), "Help & Usage:".white(), "buit --help (built-in documentation)".cyan());
    println!();
    if let Err(e) = setup::check_and_setup() {
        eprintln!("Setup error: {}", e);
    }
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Username(args) => {
            modules::username::run(args).await?;
        }
        cli::Commands::Email(args) => {
            modules::email::run(args).await?;
        }
        cli::Commands::Search(args) => {
            modules::search::run(args).await?;
        }
        cli::Commands::Dork(args) => {
            modules::dork::run(args).await?;
        }
        cli::Commands::Social(args) => {
            modules::social::run(args).await?;
        }
        cli::Commands::Config(args) => {
            config::manage::run(args)?;
        }
        cli::Commands::Phone(args) => {
            modules::phone::run(args).await?;
        }
        cli::Commands::Ip(args) => {
            modules::ip::run(args).await?;
        }
        cli::Commands::Domain(args) => {
            modules::domain::run(args).await?;
        }
        cli::Commands::Leaks(args) => {
            modules::leaks::run(args).await?;
        }
        cli::Commands::Metadata(args) => {
            modules::metadata::run(args)?;
        }
        cli::Commands::Subdomain(args) => {
            modules::subdomain::run(args).await?;
        }
        cli::Commands::Shodan(args) => {
            modules::shodan::run(args).await?;
        }
        cli::Commands::Portscan(args) => {
            modules::portscan::run(args).await?;
        }
        cli::Commands::Whois(args) => {
            modules::whois::run(args).await?;
        }
        cli::Commands::ReverseImage(args) => {
            modules::reverse_image::run(args).await?;
        }
        cli::Commands::Github(args) => {
            modules::github::run(args).await?;
        }
        cli::Commands::Hash(args) => {
            modules::hash::run(args).await?;
        }
        cli::Commands::Urlscan(args) => {
            modules::urlscan::run(args).await?;
        }
        cli::Commands::Wayback(args) => {
            modules::wayback::run(args).await?;
        }
        cli::Commands::Geoip(args) => {
            modules::geoip::run(args).await?;
        }
        cli::Commands::Report(args) => {
            modules::report::run(args)?;
        }
        cli::Commands::Interactive => {
            modules::interactive::run().await?;
        }
        cli::Commands::Setup => {
            setup::force_setup().await?;
        }
    }
    Ok(())
}
