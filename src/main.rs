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
async fn main() -> Result<()> {
    let cfg = config::Config::load_from_file("config.yaml").await?;
    let config = Arc::new(cfg);
    println!("Config: {:?}", config);

    cache::init();

    let engine = Engine::default();
    let plugin_map = Arc::new(Mutex::new(plugins::load_plugins(&engine)?));
    plugins::start_hot_reload(engine.clone(), plugin_map.clone())?;

    let routes = Arc::new(config.routes.clone());

    // avvia servizi
    let http_handle = tokio::spawn(server::run_http(config.server.http_port, routes.clone(), plugin_map.clone()));
    let ws_handle = tokio::spawn(websocket::run(config.server.ws_port,plugin_map.clone()));
    let quic_handle = tokio::spawn(server::run_quic(config.server.quic_port, config.clone(), plugin_map.clone()));

    let _ = tokio::try_join!(http_handle, ws_handle, quic_handle)?;
    Ok(())
}