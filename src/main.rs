mod server;
mod websocket;
mod router;
mod cache;
mod plugins;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inizializza cache
    cache::init().await;

    // Carica plugin WASM
    //plugins::load_plugins("./plugins").await?;

    // Avvia server HTTP
    tokio::spawn(async {
        server::run().await.unwrap();
    });

    // Avvia server WebSocket
    tokio::spawn(async {
        websocket::run().await.unwrap();
    });

    println!("Server HTTP/1.1 e WebSocket avviato...");
    tokio::signal::ctrl_c().await?;
    println!("Server chiuso.");
    Ok(())
}