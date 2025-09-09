use dashmap::DashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref CACHE: DashMap<String, String> = DashMap::new();
}

pub async fn init() {
    println!("Cache inizializzata");
}

pub fn get(key: &str) -> Option<String> {
    CACHE.get(key).map(|v| v.value().clone())
}

pub fn set(key: String, value: String) {
    CACHE.insert(key, value);
}