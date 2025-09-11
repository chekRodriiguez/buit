pub mod manage;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_keys: HashMap<String, String>,
    pub settings: Settings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub timeout: u64,
    pub max_threads: usize,
    pub user_agent: String,
    pub user_agent_preset: UserAgentPreset,
    pub proxy: Option<String>,
    pub proxy_auth: Option<ProxyAuth>,
    pub retry_count: usize,
    pub rate_limit_delay: u64,
    pub auto_update: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserAgentPreset {
    Chrome,
    Firefox,
    Safari,
    Edge,
    Mobile,
    Osint,
    Custom(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProxyAuth {
    pub username: String,
    pub password: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api_keys: HashMap::new(),
            settings: Settings {
                timeout: 30,
                max_threads: 10,
                user_agent: UserAgentPreset::Chrome.to_string(),
                user_agent_preset: UserAgentPreset::Chrome,
                proxy: None,
                proxy_auth: None,
                retry_count: 3,
                rate_limit_delay: 100,
                auto_update: true,
            },
        }
    }
}

impl UserAgentPreset {
    pub fn to_string(&self) -> String {
        match self {
            UserAgentPreset::Chrome => {
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string()
            }
            UserAgentPreset::Firefox => {
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0".to_string()
            }
            UserAgentPreset::Safari => {
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_2_1) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15".to_string()
            }
            UserAgentPreset::Edge => {
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0".to_string()
            }
            UserAgentPreset::Mobile => {
                "Mozilla/5.0 (iPhone; CPU iPhone OS 17_2_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1".to_string()
            }
            UserAgentPreset::Osint => {
                "BUIT Osint/1.0 (Open source osint toolkit)".to_string()
            }
            UserAgentPreset::Custom(ua) => ua.clone(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if !config_path.exists() {
            let config = Config::default();
            config.save()?;
            return Ok(config);
        }
        
        let content = fs::read_to_string(&config_path)?;
        match serde_json::from_str::<Config>(&content) {
            Ok(config) => Ok(config),
            Err(_) => {
                // Configuration incompatible, recréer avec valeurs par défaut
                let config = Config::default();
                config.save()?;
                Ok(config)
            }
        }
    }
    
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(config_path, content)?;
        Ok(())
    }
    
    fn config_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        path.push("buit");
        path.push("config.json");
        Ok(path)
    }
    
    #[allow(dead_code)]
    pub fn get_api_key(&self, service: &str) -> Option<&String> {
        self.api_keys.get(service)
    }
    
    pub fn set_api_key(&mut self, service: String, key: String) -> Result<()> {
        self.api_keys.insert(service, key);
        self.save()
    }
    
    pub fn set_auto_update(&mut self, enabled: bool) -> Result<()> {
        self.settings.auto_update = enabled;
        self.save()
    }
}