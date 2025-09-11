use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub routes: Vec<RouteConfig>,
}


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
    pub plugins: Vec<RoutePluginConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RoutePluginConfig {
    pub name: String,
    pub triggers: Option<Vec<TriggerConfig>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TriggerConfig {
    pub event: String,
    pub protocol: Option<String>,
}

impl Config {
    pub async fn load_from_file(path: &str) -> anyhow::Result<Config> {
        let s = fs::read_to_string(path)?;
        let cfg: Config = serde_yaml::from_str(&s)?;
        Ok(cfg)
    }
}