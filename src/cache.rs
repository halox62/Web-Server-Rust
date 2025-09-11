use dashmap::DashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CACHE: DashMap<String, Vec<u8>> = DashMap::new();
}

pub fn get(key: &str) -> Option<Vec<u8>> {
    CACHE.get(key).map(|v| v.clone())
}

pub fn set(key: String, value: Vec<u8>) {
    CACHE.insert(key, value);
}

pub fn init() {
    // eventuale inizializzazione
}