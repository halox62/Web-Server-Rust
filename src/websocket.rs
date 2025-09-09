use tokio_tungstenite::accept_async;
use tokio::net::TcpListener;
use crate::plugins;

pub async fn run() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9001").await?;
    println!("WebSocket in ascolto su ws://127.0.0.1:9001");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            println!("Nuovo WebSocket connesso");

            // Esegui plugin on_ws_connect
            plugins::on_ws_connect(&ws_stream).await;

            // Qui puoi gestire messaggi WebSocket...
        });
    }

    Ok(())
}