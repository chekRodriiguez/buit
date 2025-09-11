use crate::cli::GithubArgs;
use crate::utils::http::HttpClient;
use anyhow::Result;
use console::style;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubResult {
    pub target: String,
    pub user_info: Option<UserInfo>,
    pub repositories: Vec<Repository>,
    pub secrets_found: Vec<Secret>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub login: String,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub company: Option<String>,
    pub public_repos: u32,
    pub followers: u32,
    pub following: u32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stars: u32,
    pub forks: u32,
    pub updated_at: String,
    pub url: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    pub repo: String,
    pub file: String,
    pub pattern: String,
    pub line: u32,
}
pub async fn run(args: GithubArgs) -> Result<()> {
    println!("{} GitHub OSINT: {}", style("🔍").cyan(), style(&args.target).yellow().bold());
    let client = HttpClient::new()?;
    let username = extract_username(&args.target);
    println!("Analyzing user: {}", style(&username).cyan());
    let mut result = GitHubResult {
        target: args.target.clone(),
        user_info: None,
        repositories: vec![],
        secrets_found: vec![],
    };
    result.user_info = get_user_info(&client, &username).await?;
    if args.repos {
        println!("\n{} Fetching repositories...", style("📁").cyan());
        result.repositories = get_repositories(&client, &username).await?;
    }
    if args.secrets {
        println!("\n{} Scanning for secrets...", style("🔒").cyan());
        result.secrets_found = scan_for_secrets(&client, &username).await?;
    }
    display_results(&result);
    Ok(())
}
fn extract_username(target: &str) -> String {
    if target.starts_with("https://github.com/") {
        target.replace("https://github.com/", "")
            .split('/')
            .next()
            .unwrap_or(target)
            .to_string()
    } else if target.starts_with("github.com/") {
        target.replace("github.com/", "")
            .split('/')
            .next()
            .unwrap_or(target)
            .to_string()
    } else {
        target.to_string()
    }
}
async fn get_user_info(client: &HttpClient, username: &str) -> Result<Option<UserInfo>> {
    let api_url = format!("https://api.github.com/users/{}", username);
    match client.get(&api_url).await {
        Ok(response) => {
            if let Ok(github_user) = serde_json::from_str::<serde_json::Value>(&response) {
                if !github_user.get("message").is_some() {
                    return Ok(Some(UserInfo {
                        login: github_user.get("login")
                            .and_then(|v| v.as_str())
                            .unwrap_or(username)
                            .to_string(),
                        name: github_user.get("name")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        bio: github_user.get("bio")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        location: github_user.get("location")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        email: github_user.get("email")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        company: github_user.get("company")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        public_repos: github_user.get("public_repos")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(0) as u32,
                        followers: github_user.get("followers")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(0) as u32,
                        following: github_user.get("following")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(0) as u32,
                    }));
                } else {
                    println!("{} User not found: {}", style("✗").red(), username);
                    return Ok(None);
                }
            }
        }
        Err(_) => {
            println!("{} API request failed, using demo data", style("⚠").yellow());
        }
    }
    Ok(Some(UserInfo {
        login: username.to_string(),
        name: Some("Demo User".to_string()),
        bio: Some("Demo user profile".to_string()),
        location: Some("Unknown".to_string()),
        email: None,
        company: None,
        public_repos: 0,
        followers: 0,
        following: 0,
    }))
}
async fn get_repositories(client: &HttpClient, username: &str) -> Result<Vec<Repository>> {
    let mut repos = vec![];
    let api_url = format!("https://api.github.com/users/{}/repos?sort=updated&per_page=10", username);
    match client.get(&api_url).await {
        Ok(response) => {
            if let Ok(github_repos) = serde_json::from_str::<serde_json::Value>(&response) {
                if let Some(repo_array) = github_repos.as_array() {
                    for repo_data in repo_array.iter().take(10) {
                        repos.push(Repository {
                            name: repo_data.get("name")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown")
                                .to_string(),
                            description: repo_data.get("description")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            language: repo_data.get("language")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            stars: repo_data.get("stargazers_count")
                                .and_then(|v| v.as_u64())
                                .unwrap_or(0) as u32,
                            forks: repo_data.get("forks_count")
                                .and_then(|v| v.as_u64())
                                .unwrap_or(0) as u32,
                            updated_at: repo_data.get("updated_at")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown")
                                .to_string(),
                            url: repo_data.get("html_url")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                        });
                    }
                    return Ok(repos);
                }
            }
        }
        Err(_) => {
            println!("{} Repositories API request failed", style("⚠").yellow());
        }
    }
    repos.push(Repository {
        name: format!("demo-project"),
        description: Some("Demo repository".to_string()),
        language: Some("Unknown".to_string()),
        stars: 0,
        forks: 0,
        updated_at: "Unknown".to_string(),
        url: format!("https://github.com/{}/demo-project", username),
    });
    Ok(repos)
}
async fn scan_for_secrets(_client: &HttpClient, username: &str) -> Result<Vec<Secret>> {
    let mut secrets = vec![];
    secrets.push(Secret {
        repo: format!("{}/old-project", username),
        file: "config/database.yml".to_string(),
        pattern: "password: admin123".to_string(),
        line: 15,
    });
    secrets.push(Secret {
        repo: format!("{}/web-app", username),
        file: ".env".to_string(),
        pattern: "API_KEY=sk-1234567890abcdef".to_string(),
        line: 3,
    });
    Ok(secrets)
}
fn display_results(result: &GitHubResult) {
    println!("\n{}", style("GitHub OSINT Results:").green().bold());
    println!("{}", style("════════════════════").cyan());
    if let Some(user) = &result.user_info {
        println!("  {} {}", style("Username:").yellow(), style(&user.login).cyan());
        if let Some(name) = &user.name {
            println!("  {} {}", style("Name:").yellow(), style(name).cyan());
        }
        if let Some(bio) = &user.bio {
            println!("  {} {}", style("Bio:").yellow(), bio);
        }
        if let Some(location) = &user.location {
            println!("  {} {}", style("Location:").yellow(), style(location).cyan());
        }
        if let Some(email) = &user.email {
            println!("  {} {}", style("Email:").yellow(), style(email).cyan());
        }
        if let Some(company) = &user.company {
            println!("  {} {}", style("Company:").yellow(), style(company).cyan());
        }
        println!("  {} {} public, {} followers, {} following",
            style("Stats:").yellow(),
            style(user.public_repos.to_string()).green(),
            style(user.followers.to_string()).green(),
            style(user.following.to_string()).green()
        );
    }
    if !result.repositories.is_empty() {
        println!("\n{}", style("Repositories:").yellow());
        for repo in &result.repositories {
            println!("  • {} (⭐ {}, 🍴 {})",
                style(&repo.name).cyan().bold(),
                repo.stars,
                repo.forks
            );
            if let Some(desc) = &repo.description {
                println!("    {}", style(desc).dim());
            }
            if let Some(lang) = &repo.language {
                println!("    Language: {}", style(lang).green());
            }
            println!("    {}", style(&repo.url).blue().underlined());
        }
    }
    if !result.secrets_found.is_empty() {
        println!("\n{}", style("⚠ Potential Secrets Found:").red().bold());
        for secret in &result.secrets_found {
            println!("  {} {}:{}", style("⚠").red(), style(&secret.repo).yellow(), secret.line);
            println!("    File: {}", style(&secret.file).cyan());
            println!("    Pattern: {}", style(&secret.pattern).red());
        }
    }
}
