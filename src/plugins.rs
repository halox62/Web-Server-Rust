use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use hyper::{Request, Body};
use notify::{recommended_watcher, RecursiveMode, Watcher, Result as NotifyResult, Event};
use std::sync::mpsc::channel;
use std::time::Duration;
use wasmtime::{Engine, Store, Module, Instance, Linker};
use wasmtime_wasi::sync::WasiCtxBuilder;
use hyper::HeaderMap;

pub type PluginMap = Arc<Mutex<HashMap<String, Plugin>>>;


#[derive(Clone)]
pub struct Plugin {
    pub name: String,
    pub module: Module,
    pub path: PathBuf,
}

impl Plugin {
    /// Esegui il plugin: istanzia il modulo con WASI che eredita stdout/stderr
    pub async fn run(&self, req: &Request<Body>) -> anyhow::Result<()> {
        // Crea uno Store con WasiCtx che eredita stdout/stderr così stampa del wasm va sullo stdout del processo.
        let engine = Engine::default();
        let wasi = WasiCtxBuilder::new().inherit_stdio().build();
        let mut store = Store::new(&engine, wasi);

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut wasmtime_wasi::WasiCtx| cx)?;
        // Prepara WASI context
        let wasi = WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .build();
        // Instanzia
        let instance = linker.instantiate(&mut store, &self.module)?;
        // Cerca la funzione `run()`
        if let func = instance.get_typed_func::<(), ()>(&mut store, "run")? {
            func.call(&mut store, ())?;
        } else {
            // Se non esiste, abbiamo comunque successo: plugin non definito correttamente
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

/// Carica tutti i wasm nella cartella plugins/ in una HashMap
pub fn load_plugins(engine: &Engine) -> anyhow::Result<HashMap<String, Plugin>> {
    let mut map = HashMap::new();
    let dir = PathBuf::from("./plugins");
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }
    for entry in std::fs::read_dir(dir)? {
        let p = entry?.path();
        if p.extension().and_then(|e| e.to_str()) == Some("wasm") {
            let name = p.file_stem().unwrap().to_string_lossy().to_string();
            let module = Module::from_file(engine, &p)?;
            let plugin = Plugin { name: name.clone(), module, path: p.clone() };
            map.insert(name.clone(), plugin);
            println!("Plugin caricato: {}", name);
        }
    }
    Ok(map)
}

/// Hot reload: crea watcher e quando un file cambia richiama reload (semplice)
pub fn start_hot_reload(engine: Engine, map: PluginMap) -> NotifyResult<()> {
    // Canale sync fra watcher e thread
    let (tx, rx) = channel::<notify::Result<Event>>();

    // recommended_watcher è cross-platform
    let mut watcher = recommended_watcher(move |res| {
        let _ = tx.send(res);
    })?;
    watcher.watch(std::path::Path::new("./plugins"), RecursiveMode::NonRecursive)?;

    // Thread che processa gli eventi e ricarica i moduli (usa tokio runtime se vuoi async)
    std::thread::spawn(move || {
        while let Ok(res) = rx.recv() {
            match res {
                Ok(event) => {
                    println!("hot-reload: evento {:?}", event);
                    // ricarica tutti i wasm (semplice: ricarica mappa completa)
                    if let Ok(new_map) = load_plugins(&engine) {
                        // aggiorna la mappa in modo sincronizzato (blocco sincrono su tokio::sync::Mutex)
                        let cloned = map.clone();
                        // spawn small task nel runtime tokio
                        let _ = tokio::runtime::Handle::current().block_on(async move {
                            let mut guard = cloned.lock().await;
                            *guard = new_map;
                        });
                    }
                }
                Err(e) => eprintln!("watcher error: {:?}", e),
            }
        }
    });

    Ok(())
}

pub fn run_on_connect(name: &str) {
    println!("Plugin {} connesso", name);
}

pub fn run_on_ws_connect(name: &str) {
    println!("Plugin {} WS connesso", name);
}