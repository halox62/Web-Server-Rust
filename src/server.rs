use hyper::{Server, Request, Response, Body};
use hyper::service::{make_service_fn, service_fn};
use crate::router;
use crate::plugins;

pub async fn run() -> anyhow::Result<()> {
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, hyper::Error>(service_fn(|req: Request<Body>| async move {
            // Esegui plugin on_connect
            plugins::on_connect(&req).await;

            // Gestisci richiesta tramite router
            router::handle_request(req).await
        }))
    });

    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("HTTP server in ascolto su http://{}", addr);
    server.await?;
    Ok(())
}