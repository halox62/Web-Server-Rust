use hyper::{Body, Client, Request, Response, Uri};
use crate::config::RouteConfig;
use crate::cache;
use crate::plugins::{Plugin, PluginMap};
use std::{collections::HashMap, sync::Arc};

pub async fn handle_request(
    mut req: Request<Body>,
    routes: Arc<Vec<RouteConfig>>,
    plugins: PluginMap, // PluginMap = Arc<Mutex<HashMap<String, Plugin>>>
) -> Result<Response<Body>, hyper::Error> {
    let path = req.uri().path().to_string();

    if let Some(route) = routes.iter().find(|r| path.starts_with(&r.path)) {
        // prendo una copia locale della mappa plugin (per non lockare ripetutamente)
        let map: HashMap<String, Plugin> = plugins.lock().await.clone();

        // esegui ogni plugin dichiarato nella config della route
        for p in &route.plugins {
            if let Some(plugin) = map.get(&p.name) {
                // estrai solo le info che servono al plugin
                let plugin_clone = plugin.clone();
                let path_clone = path.clone();
                let headers_clone = req.headers().clone();

                tokio::spawn(async move {
                    // aggiungi un metodo nel plugin che accetti i dati semplici
                    let _ = plugin_clone
                        .run_with_data(path_clone, headers_clone)
                        .await;
                });
            }
        }

        println!(
            "Richiesta {} → proxy a {} (cache: {})",
            route.path, route.upstream, route.cache
        );

        // cache semplice
        if route.cache {
            if let Some(cached_body) = cache::get(&path) {
                println!("→ Risposta servita dalla cache");
                return Ok(Response::new(Body::from(cached_body)));
            }
        }

        // prepara l’URI del backend
        let backend_uri: Uri = format!(
            "{}{}",
            route.upstream,
            req.uri()
                .path_and_query()
                .map(|x| x.as_str())
                .unwrap_or("")
        )
        .parse()
        .expect("Errore costruzione URI backend");

        *req.uri_mut() = backend_uri;

        // inoltra la richiesta al backend
        let client = Client::new();
        let resp = client.request(req).await?;
        let bytes = hyper::body::to_bytes(resp.into_body()).await?;
        let body_vec = bytes.to_vec();

        if route.cache {
            cache::set(path.clone(), body_vec.clone());
        }

        Ok(Response::new(Body::from(body_vec)))
    } else {
        Ok(Response::builder()
            .status(404)
            .body(Body::from("404 - Route non trovata"))
            .unwrap())
    }
}