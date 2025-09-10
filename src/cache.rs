use dashmap::DashMap;
use lazy_static::lazy_static;

lazy_static! {
    // Cache globale: key = URL, value = Body
    pub static ref CACHE: DashMap<String, Vec<u8>> = DashMap::new();
}

/// Recupera una risposta dalla cache
pub fn get(key: &str) -> Option<Vec<u8>> {
    CACHE.get(key).map(|v| v.clone())
}

/// Salva una risposta nella cache
pub fn set(key: String, value: Vec<u8>) {
    CACHE.insert(key, value);
}