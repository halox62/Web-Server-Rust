use wasmtime::{Engine, Module, Store, Instance};
use std::path::Path;
use std::error::Error;

//cargo build --target wasm32-unknown-unknown --release

const PLUGIN_DIR: &str = "./plugins";


#[derive(Clone)]
pub struct Plugin {
    pub name: String,
    pub module: Module,
}

pub fn load_plugins(engine: &Engine) -> Result<Vec<Plugin>, Box<dyn Error + Send + Sync>> {
    let mut plugins = Vec::new();
    for entry in std::fs::read_dir("plugins")? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("wasm") {
            let name = path.file_stem().unwrap().to_string_lossy().to_string();
            let module = Module::from_file(engine, &path)?;
            plugins.push(Plugin { name, module });
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