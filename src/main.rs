mod config;
mod router;
mod plugins;
mod server;
mod websocket;
mod cache;

use std::sync::Arc;
use wasmtime::Engine;
use anyhow::Result;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = config::Config::load_from_file("config.yaml").await?;
    let config = Arc::new(cfg.clone());
    let shared_config = Arc::new(Mutex::new(cfg));
    println!("Config: {:?}", config);

    cache::init();

    let engine = Engine::default();
    let plugin_map = Arc::new(Mutex::new(plugins::load_plugins(&engine,&config)?));
    let _watcher = plugins::watch_config_file(
        "config.yaml".to_string(),
        shared_config.clone(),
        Arc::new(engine.clone()),
        plugin_map.clone()
    ).await?;
    let routes = Arc::new(config.routes.clone());

    // Collezione di task da avviare
    let mut tasks = Vec::new();

    if config.server.enable_http {
        tasks.push(tokio::spawn(server::run_http(
            config.server.http_port,
            routes.clone(),
            plugin_map.clone(),
        )));
    }

    if config.server.enable_ws {
        tasks.push(tokio::spawn(websocket::run(
            config.server.ws_port,
            plugin_map.clone(),
        )));
    }

    if config.server.enable_quic {
        tasks.push(tokio::spawn(server::run_quic(
            config.server.quic_port,
            config.clone(),
            plugin_map.clone(),
        )));
    }

    for t in tasks {
        t.await
            .map_err(|e| anyhow::Error::new(e))?
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;
    }
    Ok(())
}