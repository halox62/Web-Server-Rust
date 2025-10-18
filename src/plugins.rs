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
use std::fs;
use anyhow::{Result, anyhow};

#[derive(Clone)]
pub struct Plugin {
    pub name: String,
    pub module: Module,
    pub path: PathBuf,
}

impl Plugin {

 
    pub async fn run(&self, req: &Request<Body>) -> anyhow::Result<()> {
        let engine = Engine::default();
    
        let wasi = WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .inherit_stdio() 
            .build();
    
        let mut store = Store::new(&engine, wasi);
    

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut wasmtime_wasi::WasiCtx| cx)?;
    
        let instance = linker.instantiate(&mut store, &self.module)?;
    
        match instance.get_typed_func::<(), ()>(&mut store, "run") {
            Ok(func) => {
                func.call(&mut store, ())?;
            }
            Err(_) => {
                println!("Plugin {} non espone `run()`", self.name);
            }
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
        // logica reale
        Ok(())
    }
}

fn verify(plugin_path: &str, public_key_path: &str) -> bool {
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




pub fn verify_with_trusted_keys(plugin_path: &str, trusted_keys_path: &str) -> Result<()> {

    let data = fs::read_to_string(trusted_keys_path)?;
    let list: serde_json::Value = serde_json::from_str(&data)?;
    let keys = list["trusted_keys"]
        .as_array()
        .ok_or(anyhow!("Invalid trusted_keys file"))?;


    for (i, key_entry) in keys.iter().enumerate() {
        let key_path = key_entry.as_str().ok_or(anyhow!("Invalid key path"))?;

        if verify(plugin_path, key_path) {
            return Ok(());
        } else {
            println!("Verification failed");
        }
    }

    Err(anyhow!("Plugin signature invalid or not trusted"))
}

pub fn load_plugins(engine: &Engine, config: &Config) -> anyhow::Result<HashMap<String, Plugin>> {
    let mut map = HashMap::new();
    let dir = PathBuf::from("./plugins");

    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }

    let required_plugins: Vec<String> = config
        .routes
        .iter()
        .flat_map(|r| r.plugins.iter().map(|p| p.name.clone()))
        .collect();

    println!("Plugin richiesti nel config: {:?}", required_plugins);

    for plugin_name in required_plugins {
        let plugin_path = dir.join(format!("{}.wasm", plugin_name));

        if !plugin_path.exists() {
            eprintln!("Plugin dichiarato nel config ma non trovato: {:?}", plugin_path);
            continue;
        }

        if verify_with_trusted_keys(plugin_path.to_str().unwrap(), "./keys/trusted_keys.json").is_ok() {
            println!("Plugin verified and trusted: {:?}", plugin_path);

            let module = Module::from_file(engine, &plugin_path)?;
            let plugin = Plugin {
                name: plugin_name.clone(),
                module,
                path: plugin_path.clone(),
            };

            map.insert(plugin_name.clone(), plugin);
            println!("Plugin caricato: {}", plugin_name);
        } else {
            eprintln!("Plugin non firmato: {:?}", plugin_path);
        }
    }

    Ok(map)
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