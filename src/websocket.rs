use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use crate::plugins;
use std::error::Error;

pub async fn run(port: u16) -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket listening on ws://{}", addr);

    let engine = wasmtime::Engine::default();
    let loaded_plugins = plugins::load_plugins(&engine)?;

    while let Ok((stream, addr)) = listener.accept().await {
        for plugin in &loaded_plugins {
            plugins::run_on_connect(plugin);
        }

        tokio::spawn({
            let loaded_plugins = loaded_plugins.clone();
            async move {
                let ws_stream = match accept_async(stream).await {
                    Ok(ws) => ws,
                    Err(e) => {
                        eprintln!("Errore handshake WS: {:?}", e);
                        return;
                    }
                };

                for plugin in &loaded_plugins {
                    plugins::run_on_ws_connect(plugin);
                }

                let (mut write, mut read) = ws_stream.split();

                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(msg) => {
                            if msg.is_text() || msg.is_binary() {
                                if let Err(e) = write.send(msg).await {
                                    eprintln!("Errore invio WS: {:?}", e);
                                    break;
                                }
                            } else if msg.is_close() {
                                println!("Connessione WS chiusa da {}", addr);
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Errore WS: {:?}", e);
                            break;
                        }
                    }
                }
            }
        });
    }

    Ok(())
}