use reqwest::{Client, StatusCode, Proxy};
use anyhow::Result;
use std::time::Duration;
use crate::config::Config;

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        let mut builder = Client::builder()
            .timeout(Duration::from_secs(config.settings.timeout))
            .user_agent(&config.settings.user_agent)
            .use_rustls_tls()  // Force rustls instead of native Windows TLS
            .pool_max_idle_per_host(0)  // Disable connection pooling on Windows
            .pool_idle_timeout(Duration::from_secs(30))
            .tcp_keepalive(Duration::from_secs(600));
        
        if let Some(proxy_url) = &config.settings.proxy {
            let mut proxy = Proxy::all(proxy_url)?;
            
            if let Some(auth) = &config.settings.proxy_auth {
                proxy = proxy.basic_auth(&auth.username, &auth.password);
            }
            
            builder = builder.proxy(proxy);
        }
        
        let client = builder.build()?;
        
        Ok(HttpClient { client })
    }
    
    pub async fn check_url(&self, url: &str) -> Result<bool> {
        let response = self.client.get(url).send().await?;
        Ok(response.status() == StatusCode::OK)
    }
    
    pub async fn get(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send().await?;
        // Limit response size to prevent memory exhaustion on Windows
        let bytes = response.bytes().await?;
        if bytes.len() > 10_000_000 { // 10MB limit
            return Err(anyhow::anyhow!("Response too large: {} bytes", bytes.len()));
        }
        let text = String::from_utf8_lossy(&bytes).to_string();
        Ok(text)
    }
    
    #[allow(dead_code)]
    pub async fn get_json<T: for<'de> serde::Deserialize<'de>>(&self, url: &str) -> Result<T> {
        let response = self.client.get(url).send().await?;
        // Limit JSON response size
        let bytes = response.bytes().await?;
        if bytes.len() > 5_000_000 { // 5MB limit for JSON
            return Err(anyhow::anyhow!("JSON response too large: {} bytes", bytes.len()));
        }
        let json: T = serde_json::from_slice(&bytes)?;
        Ok(json)
    }
    
    pub async fn get_with_headers(&self, url: &str, headers: &[(&str, &str)]) -> Result<String> {
        let mut request = self.client.get(url);
        for (key, value) in headers {
            request = request.header(*key, *value);
        }
        let response = request.send().await?;
        // Limit response size for headers requests too
        let bytes = response.bytes().await?;
        if bytes.len() > 10_000_000 { // 10MB limit
            return Err(anyhow::anyhow!("Response too large: {} bytes", bytes.len()));
        }
        let text = String::from_utf8_lossy(&bytes).to_string();
        Ok(text)
    }
}