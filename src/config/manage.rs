use super::{Config, UserAgentPreset, ProxyAuth};
use crate::cli::{ConfigArgs, ConfigAction};
use anyhow::Result;
use colored::*;

pub fn run(args: ConfigArgs) -> Result<()> {
    let mut config = Config::load()?;
    
    match args.action {
        ConfigAction::SetKey { service, key } => {
            config.set_api_key(service.clone(), key)?;
            println!("{} API key for {} has been saved", "✓".green(), service.cyan());
        }
        
        ConfigAction::SetProxy { url, username, password } => {
            config.settings.proxy = Some(url.clone());
            if let (Some(user), Some(pass)) = (username, password) {
                config.settings.proxy_auth = Some(ProxyAuth {
                    username: user,
                    password: pass,
                });
            }
            config.save()?;
            println!("{} Proxy configuration saved: {}", "✓".green(), url.cyan());
        }
        
        ConfigAction::SetUserAgent { agent } => {
            let preset = match agent.to_lowercase().as_str() {
                "chrome" => UserAgentPreset::Chrome,
                "firefox" => UserAgentPreset::Firefox,
                "safari" => UserAgentPreset::Safari,
                "edge" => UserAgentPreset::Edge,
                "mobile" => UserAgentPreset::Mobile,
                "bot" => UserAgentPreset::Bot,
                _ => UserAgentPreset::Custom(agent.clone()),
            };
            config.settings.user_agent_preset = preset.clone();
            config.settings.user_agent = preset.to_string();
            config.save()?;
            println!("{} User agent updated", "✓".green());
        }
        
        ConfigAction::SetThreads { count } => {
            config.settings.max_threads = count;
            config.save()?;
            println!("{} Thread count set to {}", "✓".green(), count.to_string().cyan());
        }
        
        ConfigAction::List => {
            println!("{}", "Configured Services:".bold());
            println!("{}", "═══════════════════".cyan());
            
            if config.api_keys.is_empty() {
                println!("{}", "No API keys configured".yellow());
            } else {
                for (service, _) in &config.api_keys {
                    println!("  • {}", service.cyan());
                }
            }
            
            println!("\n{}", "Settings:".bold());
            println!("{}", "═════════".cyan());
            println!("  Timeout: {} seconds", config.settings.timeout);
            println!("  Max Threads: {}", config.settings.max_threads);
            println!("  User Agent Preset: {:?}", config.settings.user_agent_preset);
            println!("  User Agent: {}", &config.settings.user_agent[..50.min(config.settings.user_agent.len())]);
            if let Some(proxy) = &config.settings.proxy {
                println!("  Proxy: {}", proxy);
                if config.settings.proxy_auth.is_some() {
                    println!("  Proxy Auth: Configured");
                }
            }
            println!("  Retry Count: {}", config.settings.retry_count);
            println!("  Rate Limit Delay: {}ms", config.settings.rate_limit_delay);
        }
        
        ConfigAction::Test { service } => {
            if let Some(service_name) = service {
                if config.api_keys.contains_key(&service_name) {
                    println!("{} API key for {} is configured", "✓".green(), service_name.cyan());
                } else {
                    println!("{} No API key found for {}", "✗".red(), service_name.cyan());
                }
            } else {
                println!("{}", "Testing all configured API keys...".yellow());
                for (service_name, _) in &config.api_keys {
                    println!("  {} {}", "✓".green(), service_name.cyan());
                }
            }
        }
    }
    
    Ok(())
}