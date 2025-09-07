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
            .user_agent(&config.settings.user_agent);
        
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
        let text = response.text().await?;
        Ok(text)
    }
    
    #[allow(dead_code)]
    pub async fn get_json<T: for<'de> serde::Deserialize<'de>>(&self, url: &str) -> Result<T> {
        let response = self.client.get(url).send().await?;
        let json = response.json::<T>().await?;
        Ok(json)
    }
}