use hyper::{Body, Client, Request, Response, Uri};
use crate::config::RouteConfig;
use crate::cache;
use std::sync::Arc;

pub async fn handle_request(
    mut req: Request<Body>,
    routes: Arc<Vec<RouteConfig>>,
) -> Result<Response<Body>, hyper::Error> {

    let path = req.uri().path().to_string();
    if let Some(route) = routes.iter().find(|r| path.starts_with(&r.path)) {

        println!("Richiesta {} → proxy a {} (cache: {})", route.path, route.upstream, route.cache);

        if route.cache {
            if let Some(cached_body) = cache::get(&path) {
                println!("→ Risposta servita dalla cache");
                return Ok(Response::new(Body::from(cached_body)));
            }
        }

        let backend_uri: Uri = format!("{}{}", route.upstream, req.uri().path_and_query().map(|x| x.as_str()).unwrap_or(""))
            .parse()
            .expect("Errore costruzione URI backend");

        *req.uri_mut() = backend_uri;

        let client = Client::new();
        let resp = client.request(req).await?;

        let bytes = hyper::body::to_bytes(resp.into_body()).await?;
        let body_vec = bytes.to_vec();

        if route.cache {
            cache::set(path.clone(), body_vec.clone());
        }

        Ok(Response::new(Body::from(body_vec)))
    } else {
        Ok(Response::new(Body::from("404 - Route non trovata")))
    }
}