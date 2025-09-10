mod config;
mod server;
mod websocket;
mod router;
mod plugins;
mod cache; 

use anyhow::Result;
use config::Config;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load_from_file("config.yaml").await?;
    println!("Config caricata: {:?}", config);

    println!("Avvio su HTTP:{} e WS:{}", config.server.http_port, config.server.ws_port);

    let routes = Arc::new(config.routes.clone());

    let http_handle = tokio::spawn({
        let routes = routes.clone();
        async move {
            if let Err(e) = server::run(config.server.http_port, routes).await {
                eprintln!("Errore HTTP server: {:?}", e);
            }
        }
    });

    let ws_handle = tokio::spawn(async move {
        if let Err(e) = websocket::run(config.server.ws_port).await {
            eprintln!("Errore WS server: {:?}", e);
        }
    });

    tokio::join!(http_handle, ws_handle);

    Ok(())
}