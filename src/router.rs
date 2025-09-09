use hyper::{Request, Response, Body};

pub async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Semplice risposta
    Ok(Response::new(Body::from("Ciao franco!")))
}