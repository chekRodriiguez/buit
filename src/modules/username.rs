use crate::cli::UsernameArgs;
use crate::utils::http::HttpClient;
use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use futures::future::join_all;
#[derive(Debug, Serialize, Deserialize)]
pub struct UsernameResult {
    pub platform: String,
    pub url: String,
    pub exists: bool,
    pub profile_data: Option<HashMap<String, String>>,
}
pub async fn run(args: UsernameArgs) -> Result<()> {
    println!("{} Searching for username: {}", "🔍".cyan(), args.username.yellow().bold());
    let platforms = get_platforms(&args.platforms);
    let pb = ProgressBar::new(platforms.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    let client = HttpClient::new()?;
    let mut tasks = vec![];
    for platform in platforms {
        let username = args.username.clone();
        let client_clone = client.clone();
        let pb_clone = pb.clone();
        tasks.push(async move {
            let result = check_platform(&client_clone, &platform, &username).await;
            pb_clone.inc(1);
            pb_clone.set_message(format!("Checking {}", platform.name));
            result
        });
    }
    let results: Vec<Result<UsernameResult>> = join_all(tasks).await;
    pb.finish_and_clear();
    println!("\n{}", "Results:".green().bold());
    println!("{}", "════════".cyan());
    let mut found_count = 0;
    let mut not_found_count = 0;
    for result in &results {
        match result {
            Ok(res) => {
                if res.exists {
                    found_count += 1;
                    println!("  {} {} - {}",
                        "✓".green().bold(),
                        res.platform.cyan(),
                        res.url.blue().underline()
                    );
                    if let Some(data) = &res.profile_data {
                        for (key, value) in data {
                            println!("      {} {}",
                                format!("{}:", key).yellow(),
                                value
                            );
                        }
                    }
                } else {
                    not_found_count += 1;
                    if args.format == "verbose" {
                        println!("  {} {}", "✗".red(), res.platform);
                    }
                }
            }
            Err(e) => {
                if args.format == "verbose" {
                    eprintln!("  {} Error: {}", "⚠".yellow(), e);
                }
            }
        }
    }
    println!("\n{}", "Summary:".bold());
    println!("  Found: {} profiles", found_count.to_string().green());
    println!("  Not found: {} platforms", not_found_count.to_string().yellow());
    if let Some(output_file) = args.output {
        save_results(&output_file, &results, &args.format)?;
        println!("\n{} Results saved to: {}", "💾".cyan(), output_file.blue());
    }
    Ok(())
}
#[derive(Clone)]
struct Platform {
    name: String,
    url_template: String,
    #[allow(dead_code)]
    check_type: CheckType,
}
#[derive(Clone)]
#[allow(dead_code)]
enum CheckType {
    StatusCode,
    StringMatch(String),
    JsonField(String),
}
fn get_platforms(filter: &Option<String>) -> Vec<Platform> {
    let mut platforms = vec![
        Platform {
            name: "GitHub".to_string(),
            url_template: "https://github.com/{}".to_string(),
            check_type: CheckType::StatusCode,
        },
        Platform {
            name: "Twitter/X".to_string(),
            url_template: "https://twitter.com/{}".to_string(),
            check_type: CheckType::StatusCode,
        },
        Platform {
            name: "Instagram".to_string(),
            url_template: "https://www.instagram.com/{}/".to_string(),
            check_type: CheckType::StatusCode,
        },
        Platform {
            name: "LinkedIn".to_string(),
            url_template: "https://www.linkedin.com/in/{}/".to_string(),
            check_type: CheckType::StatusCode,
        },
        Platform {
            name: "Reddit".to_string(),
            url_template: "https://www.reddit.com/user/{}".to_string(),
            check_type: CheckType::StatusCode,
        },
        Platform {
            name: "TikTok".to_string(),
            url_template: "https://www.tiktok.com/@{}".to_string(),
            check_type: CheckType::StatusCode,
        },
        Platform {
            name: "YouTube".to_string(),
            url_template: "https://www.youtube.com/@{}".to_string(),
            check_type: CheckType::StatusCode,
        },
        Platform {
            name: "Twitch".to_string(),
            url_template: "https://www.twitch.tv/{}".to_string(),
            check_type: CheckType::StatusCode,
        },
        Platform {
            name: "Steam".to_string(),
            url_template: "https://steamcommunity.com/id/{}".to_string(),
            check_type: CheckType::StatusCode,
        },
        Platform {
            name: "Pinterest".to_string(),
            url_template: "https://www.pinterest.com/{}/".to_string(),
            check_type: CheckType::StatusCode,
        },
        Platform {
            name: "Telegram".to_string(),
            url_template: "https://t.me/{}".to_string(),
            check_type: CheckType::StatusCode,
        },
        Platform {
            name: "Medium".to_string(),
            url_template: "https://medium.com/@{}".to_string(),
            check_type: CheckType::StatusCode,
        },
        Platform {
            name: "DeviantArt".to_string(),
            url_template: "https://www.deviantart.com/{}".to_string(),
            check_type: CheckType::StatusCode,
        },
        Platform {
            name: "Spotify".to_string(),
            url_template: "https://open.spotify.com/user/{}".to_string(),
            check_type: CheckType::StatusCode,
        },
        Platform {
            name: "Snapchat".to_string(),
            url_template: "https://www.snapchat.com/add/{}".to_string(),
            check_type: CheckType::StatusCode,
        },
    ];
    if let Some(filter_str) = filter {
        let filters: Vec<String> = filter_str.split(',').map(|s| s.trim().to_lowercase()).collect();
        platforms.retain(|p| filters.contains(&p.name.to_lowercase()));
    }
    platforms
}
async fn check_platform(client: &HttpClient, platform: &Platform, username: &str) -> Result<UsernameResult> {
    let url = platform.url_template.replace("{}", username);
    let exists = client.check_url(&url).await?;
    Ok(UsernameResult {
        platform: platform.name.clone(),
        url: url.clone(),
        exists,
        profile_data: None,
    })
}
fn save_results(filename: &str, results: &Vec<Result<UsernameResult>>, format: &str) -> Result<()> {
    let successful_results: Vec<&UsernameResult> = results
        .iter()
        .filter_map(|r| r.as_ref().ok())
        .filter(|r| r.exists)
        .collect();
    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&successful_results)?;
            std::fs::write(filename, json)?;
        }
        "csv" => {
            let mut wtr = csv::Writer::from_path(filename)?;
            wtr.write_record(&["Platform", "URL", "Exists"])?;
            for result in successful_results {
                wtr.write_record(&[&result.platform, &result.url, "true"])?;
            }
            wtr.flush()?;
        }
        _ => {
            let mut content = String::new();
            for result in successful_results {
                content.push_str(&format!("{}: {}\n", result.platform, result.url));
            }
            std::fs::write(filename, content)?;
        }
    }
    Ok(())
}
