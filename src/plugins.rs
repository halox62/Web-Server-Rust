use wasmtime::{Engine, Module, Store, Instance};
use std::path::Path;
use std::error::Error;

const PLUGIN_DIR: &str = "./plugins";

#[derive(Clone)]
pub struct Plugin {
    pub name: String,
    pub instance: Instance,
}

pub fn load_plugins(engine: &Engine) -> Result<Vec<Plugin>, Box<dyn std::error::Error + Send + Sync>> {
    let mut plugins = Vec::new();
    if !Path::new(PLUGIN_DIR).exists() {
        std::fs::create_dir_all(PLUGIN_DIR)?;
    }

    for entry in std::fs::read_dir(PLUGIN_DIR)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map(|x| x == "wasm").unwrap_or(false) {
            let module = Module::from_file(&engine, &path)?;
            let mut store = Store::new(&engine, ());
            let instance = Instance::new(&mut store, &module, &[])?;
            let name = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            plugins.push(Plugin { name: name.clone(), instance });
            println!("Plugin {} caricato", name);
        }
    }

    Ok(plugins)
}

pub fn run_on_connect(plugin: &Plugin) {
    println!("Plugin {}: on_connect eseguito", plugin.name);
}

pub fn run_on_ws_connect(plugin: &Plugin) {
    println!("Plugin {}: on_ws_connect eseguito", plugin.name);
}