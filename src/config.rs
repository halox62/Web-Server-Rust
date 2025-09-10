use serde::Deserialize;
use anyhow::Result;
use tokio::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub routes: Vec<RouteConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub http_port: u16,
    pub ws_port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RouteConfig {
    pub path: String,
    pub upstream: String,
    pub cache: bool,
}

impl Config {
    pub async fn load_from_file(path: &str) -> Result<Self> {
        let contents = fs::read_to_string(path).await?;
        let cfg = serde_yaml::from_str(&contents)?;
        Ok(cfg)
    }
}