mod config;
mod router;
mod plugins;
mod server;
mod websocket;
mod cache;
use std::sync::Arc;
use wasmtime::Engine;
use anyhow::Result;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::Config::load_from_file("config.yaml").await?;
    let config = Arc::new(config); // <-- wrap in Arc

    println!("Config: {:?}", config);

    let routes = Arc::new(config.routes.clone());
    cache::init();

    // HTTP
    let http_handle = tokio::spawn(server::run_http(config.server.http_port, routes.clone()));
    // WebSocket
    let ws_handle = tokio::spawn(websocket::run(config.server.ws_port));
    // QUIC/HTTP3
    let quic_handle = tokio::spawn(server::run_quic(config.server.quic_port, config.clone())); // <-- pass Arc

    // Attendi tutte le task
    let _ = tokio::try_join!(http_handle, ws_handle, quic_handle)?;

    Ok(())
}