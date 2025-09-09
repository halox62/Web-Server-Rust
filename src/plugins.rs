use wasmtime::*;
use std::path::Path;

pub async fn load_plugins(dir: &str) -> anyhow::Result<()> {
    let engine = Engine::default();
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
            let module = Module::from_file(&engine, &path)?;
            println!("Plugin caricato: {:?}", path);
        }
    }
    Ok(())
}

pub async fn on_connect(_req: &impl std::fmt::Debug) {
    // Puoi eseguire funzioni WASM qui
}

pub async fn on_ws_connect(_ws: &impl std::fmt::Debug) {
    // Puoi eseguire funzioni WASM qui
}