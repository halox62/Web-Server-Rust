use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use anyhow::Result;
use tokio::sync::mpsc;
use wasmtime::{Engine, Module};
use std::error::Error;
use hyper::{Request, Body};
use std::sync::mpsc::channel;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, Config, Result as NotifyResult};
use notify::Event;

/// Directory dei plugin
pub const PLUGIN_DIR: &str = "./plugins";

pub type PluginMap = Arc<tokio::sync::Mutex<HashMap<String, Plugin>>>;

/// Plugin caricabile a runtime (modulo WASM)
#[derive(Clone)]
pub struct Plugin {
    pub name: String,
    pub module: Module,
}

impl Plugin {
    pub async fn run(&self, req: &hyper::Request<hyper::Body>) {
        println!(
            "Eseguito plugin {} per path {}",
            self.name,
            req.uri().path()
        );
        // qui potrai aggiungere la logica personalizzata,
        // ad esempio chiamare funzioni Wasm dentro `self.module`
    }
}


/// Carica tutti i plugin presenti nella directory al momento della chiamata
pub fn load_plugins(engine: &Engine) -> Result<HashMap<String, Plugin>> {
    let mut map = HashMap::new();

    for entry in std::fs::read_dir(PLUGIN_DIR)? {
        let path: PathBuf = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("wasm") {
            let name = path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();
            let module = Module::from_file(engine, &path)?;
            map.insert(name.clone(), Plugin { name, module });
        }
    }

    Ok(map)
}

/// Avvia il watcher che effettua l’hot-reload
pub fn start_hot_reload(path: &str) -> NotifyResult<()> {
    let (tx, rx) = std::sync::mpsc::channel::<notify::Event>();

    let mut watcher: RecommendedWatcher = RecommendedWatcher::new(
        move |res| {
            match res {
                Ok(event) => println!("File changed: {:?}", event),
                Err(e) => eprintln!("Watch error: {:?}", e),
            }
        },
        Config::default(),
    )?;

    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;

    std::thread::spawn(move || {
        while let Ok(event) = rx.recv() {
            println!("Evento ricevuto: {:?}", event);
            // qui puoi ricaricare i plugin
            //⚠️ Nota:
            //Attualmente il watcher stampa solo "File changed: …". Per avere l’hot reload effettivo dei plugin, dovresti collegare la logica di ricaricamento dentro il thread che legge rx.recv().
        }
    });

    Ok(())
}

/// Funzioni di “hook” che eseguono codice specifico del plugin
pub fn run_on_connect(plugin: &Plugin) {
    println!("Plugin {}: on_connect eseguito", plugin.name);
}

pub fn run_on_ws_connect(plugin: &Plugin) {
    println!("Plugin {}: on_ws_connect eseguito", plugin.name);
}