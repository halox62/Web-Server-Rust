use serde::Deserialize;
use anyhow::Result;
use tokio::fs;
use tokio::fs::File;
use std::io::Read;
use serde_yaml::from_reader;
use tokio::io::AsyncReadExt;


#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub http_port: u16,
    pub ws_port: u16,
    pub quic_port: u16,
    pub cert_path: String,
    pub key_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RouteConfig {
    pub path: String,
    pub upstream: String,
    pub cache: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub routes: Vec<RouteConfig>,
}

impl Config {
    pub async fn load_from_file(path: &str) -> anyhow::Result<Self> {
        let text = tokio::fs::read_to_string(path).await?;
        Ok(serde_yaml::from_str(&text)?)
    }
}

async fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}