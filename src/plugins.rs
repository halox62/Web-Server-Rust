use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use hyper::{Request, Body};
use std::sync::mpsc::channel;
use std::time::Duration;
use wasmtime::{Engine, Store, Module, Instance, Linker};
use wasmtime_wasi::sync::WasiCtxBuilder;
use hyper::HeaderMap;
use crate::config::Config;
use notify::{Config as NotifyConfig, RecommendedWatcher, RecursiveMode, Watcher, EventKind, recommended_watcher, Result as NotifyResult, Event};
use std::path::Path;
use futures::executor;
pub type PluginMap = Arc<Mutex<HashMap<String, Plugin>>>;
use std::process::Command;

#[derive(Clone)]
pub struct Plugin {
    pub name: String,
    pub module: Module,
    pub path: PathBuf,
}

impl Plugin {
    pub async fn run(&self, req: &Request<Body>) -> anyhow::Result<()> {
        let engine = Engine::default();
        let wasi = WasiCtxBuilder::new().inherit_stdio().build();
        let mut store = Store::new(&engine, wasi);

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut wasmtime_wasi::WasiCtx| cx)?;
        let wasi = WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .build();
        let instance = linker.instantiate(&mut store, &self.module)?;
        if let func = instance.get_typed_func::<(), ()>(&mut store, "run")? {
            func.call(&mut store, ())?;
        } else {
            println!("Plugin {} non espone `run()`", self.name);
        }
        Ok(())
    }

    pub async fn run_with_data(
        &self,
        path: String,
        headers: HeaderMap,
    ) -> anyhow::Result<()> {
        println!(
            "Plugin {} eseguito su path {} con {} header",
            self.name,
            path,
            headers.len()
        );
        // Qui puoi aggiungere la logica reale (chiamata WASM ecc.)
        Ok(())
    }
}

pub fn load_plugins(engine: &Engine, config: &Config) -> anyhow::Result<HashMap<String, Plugin>>  {
    let mut map = HashMap::new();
    let dir = PathBuf::from("./plugins");

    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }

    let required: std::collections::HashSet<String> = config
        .routes
        .iter()
        .flat_map(|r| r.plugins.iter().map(|p| p.name.clone()))
        .collect();

        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            let p = entry.path();

            if let Some(p_str) = p.to_str() {
                if verify_plugin(p_str, "/Users/giorgiomartucci/rust_web/public.key") {
                    if p.extension().and_then(|e| e.to_str()) == Some("wasm") { 
                        let name = p.file_stem().unwrap().to_string_lossy().to_string();
        
                        if required.contains(&name) {
                            let module = Module::from_file(engine, &p)?;
                            let plugin = Plugin {
                                name: name.clone(),
                                module,
                                path: p.clone(),
                            };
                            map.insert(name.clone(), plugin);
                            println!("Plugin caricato: {}", name);
                        }
                    }
                }
            } else {
                eprintln!("Percorso plugin non UTF-8 valido: {:?}", p);
            }
        }

    Ok(map)
}

fn verify_plugin(plugin_path: &str, public_key_path: &str) -> bool {
    let output = Command::new("wasmsign2")
        .arg("verify")
        .arg("--input-file")
        .arg(plugin_path)
        .arg("--public-key")
        .arg(public_key_path)
        .output()
        .expect("Failed to execute command");

    output.status.success()
}


/// Avvia il watcher sul file config.yaml
pub async fn watch_config_file(
    config_path: String,
    shared_config: Arc<Mutex<Config>>,
    engine: Arc<Engine>,
    plugin_map: Arc<Mutex<HashMap<String, Plugin>>>
) -> NotifyResult<RecommendedWatcher> {
    let config_path = Arc::new(config_path);
    let path = config_path.clone();

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            tx.send(res).unwrap();
        },
        notify::Config::default(),
    )?;

    watcher.watch(Path::new(path.as_ref()), RecursiveMode::NonRecursive)?;

    tokio::spawn(async move {
        while let Some(res) = rx.recv().await {
            match res {
                Ok(event) => {
                    println!("Evento file: {:?}", event);
                    if let Ok(cfg) = Config::load_from_file(path.as_ref()).await {
                        let mut guard = shared_config.lock().await;
                        *guard = cfg.clone();

                        let new_plugins = match load_plugins(&engine, &cfg) {
                            Ok(p) => p,
                            Err(e) => {
                                eprintln!("Errore caricamento plugin: {e}");
                                return;
                            }
                        };
                        let mut plugin_guard = plugin_map.lock().await;
                        *plugin_guard = new_plugins;
                        println!("Plugin ricaricati!");
                    }
                }
                Err(err) => eprintln!("watch error: {:?}", err),
            }
        }
    });

    Ok(watcher) 
}

pub fn run_on_connect(name: &str) {
    println!("Plugin {} connesso", name);
}

pub fn run_on_ws_connect(name: &str) {
    println!("Plugin {} WS connesso", name);
}