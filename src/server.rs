use hyper::{Body, Request, Server};
use hyper::service::{make_service_fn, service_fn};
use crate::router::handle_request;
use crate::config::RouteConfig;
use std::sync::Arc;

pub async fn run(
    port: u16, 
    routes: Arc<Vec<RouteConfig>>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let make_svc = make_service_fn(move |_conn| {
        let routes = routes.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
                let routes = routes.clone();
                async move { handle_request(req, routes).await }
            }))
        }
    });

    let addr = ([0, 0, 0, 0], port).into();
    let server = Server::bind(&addr).serve(make_svc);
    println!("HTTP server in ascolto su http://{}", addr);
    server.await?;
    Ok(())
}