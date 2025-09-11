mod cli;
mod config;
mod modules;
mod utils;
mod setup;
use anyhow::Result;
use clap::Parser;
use console::style;
use figlet_rs::FIGfont;

#[cfg(debug_assertions)]
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt, filter::LevelFilter};
#[cfg(debug_assertions)]
use tracing_appender;

// Use mimalloc as global allocator to fix Windows memory fragmentation
#[cfg(windows)]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// Custom panic hook to handle memory allocation errors gracefully
fn setup_panic_handler() {
    std::panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            if s.contains("memory allocation") {
                eprintln!("\n⚠️  Memory allocation error detected.");
                eprintln!("💡 Try using fewer platforms: buit username test -p \"github,twitter\"");
                eprintln!("🔧 Or try sequential mode: buit username test --sequential");
                // Force garbage collection before exit on Windows
                #[cfg(windows)]
                {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    let _ = panic_info;
                }
                std::process::exit(1);
            }
        }
        eprintln!("Program panicked: {:?}", panic_info);
    }));
}
fn init_terminal() {
    #[cfg(windows)]
    {
        let _ = console::Term::stdout().features().colors_supported();
    }
}

fn print_info_box() {
    let content = format!(
        "{}\n{}\n{}\n{}\n{}\n\n{} {} {}\n{} {} {}\n{} {} {}\n{} {} {}",
        style("╔═══════════════════════════════════════════════╗").cyan().bold(),
        style("║      Buu Undercover Intelligence Toolkit      ║").cyan().bold(),
        style("║       Advanced OSINT Security Framework       ║").green().bold(),
        style("║      For Authorized Security Testing Only     ║").yellow(),
        style("╚═══════════════════════════════════════════════╝").cyan().bold(),
        style("📧").red(),
        style("Copyright ©").white(),
        style("BuuDevOff - Open-Source Project").cyan().bold(),
        style("🌟").yellow(),
        style("Like this tool? Star the repo:").white(),
        style("https://github.com/BuuDevOff/BUIT").blue().underlined(),
        style("🚀").green(),
        style("Share with the community &").white(),
        style("contribute!").green().bold(),
        style("💡").yellow(),
        style("Help & Usage:").white(),
        style("buit --help (built-in documentation)").cyan()
    );
    println!("{}", content);
}
#[tokio::main]
async fn main() -> Result<()> {
    setup_panic_handler();
    init_terminal();
    
    #[cfg(debug_assertions)]
    {
        // Setup structured logging with both console and file output (debug only)
        let file_appender = tracing_appender::rolling::daily("logs", "buit.log");
        let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);
        
        tracing_subscriber::registry()
            .with(LevelFilter::DEBUG)
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(std::io::stderr)
                    .with_ansi(true)
                    .with_level(true)
                    .with_target(false)
            )
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(non_blocking_file)
                    .with_ansi(false)
                    .with_level(true)
                    .with_target(true)
            )
            .init();
        
        // Prevent _guard from being dropped immediately
        std::mem::forget(_guard);
    }
    
    match FIGfont::standard() {
        Ok(standard_font) => {
            let buit_text = standard_font.convert("BUIT");
            if let Some(text) = buit_text {
                println!("{}", style(text.to_string()).magenta().bold());
            }
        }
        Err(e) => {
            eprintln!("Warning: Could not load ASCII art font: {}", e);
            println!("{}", style("BUIT").magenta().bold());
        }
    }
    print_info_box();
    println!();
    if let Err(e) = setup::check_and_setup() {
        eprintln!("Setup error: {}", e);
    }
    let cli = cli::Cli::parse();
    
    if cli.api {
        return modules::api::start_api_server(cli.port).await;
    }
    
    // Check for updates at startup if not in API mode
    if let Err(_e) = modules::autoupdater::check_for_updates_at_startup().await {
        #[cfg(debug_assertions)]
        eprintln!("Update check error: {}", _e);
    }
    
    let command = match cli.command {
        Some(cmd) => cmd,
        None => {
            eprintln!("{} No command specified. Use --help for usage information.", style("❌").red());
            return Ok(());
        }
    };
    
    match command {
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
            let _result = modules::ip::run(args).await?;
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
        
        // New high-priority modules
        cli::Commands::ReverseDns(args) => {
            modules::reverse_dns::run(args).await?;
        }
        cli::Commands::AsnLookup(args) => {
            modules::asn_lookup::run(args).await?;
        }
        cli::Commands::SslCert(args) => {
            modules::ssl_cert_simple::run(args).await?;
        }
        cli::Commands::BreachCheck(args) => {
            modules::breach_check::run(args).await?;
        }
        cli::Commands::Update(args) => {
            modules::autoupdater::run(args).await?;
        }
    }
    Ok(())
}
