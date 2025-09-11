use hyper::{Server};
use hyper::service::{make_service_fn, service_fn};
use std::sync::Arc;
use crate::router::handle_request;
use crate::config::RouteConfig;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use quinn::{Endpoint, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;
use crate::config::Config;
use tokio::sync::Mutex;
use std::collections::HashMap;
use crate::plugins::Plugin;

pub async fn run_http(
    port: u16,
    routes: Arc<Vec<RouteConfig>>,
    plugins: Arc<Mutex<HashMap<String, Plugin>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = {
        let plugins = plugins.clone();
        make_service_fn(move |_| {
            let routes = routes.clone();
            let plugins = plugins.clone();
            async move {
                Ok::<_, hyper::Error>(service_fn(move |req| {
                    handle_request(req, routes.clone(), plugins.clone())
                }))
            }
        })
    };

    let addr = ([0, 0, 0, 0], port).into();
    println!("HTTP server listening on http://{}", addr);
    Server::bind(&addr).serve(make_svc).await?;
    Ok(())
}

pub async fn run_quic(port: u16, config: Arc<Config>,plugins: Arc<Mutex<HashMap<String, Plugin>>>,) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  
    let mut key_file = BufReader::new(File::open(&config.server.key_path)?);
    let mut cert_file = BufReader::new(File::open(&config.server.cert_path)?);

    
    let cert_chain: Vec<CertificateDer> = certs(&mut cert_file)?
        .into_iter()
        .map(|c| CertificateDer::from(c))
        .collect();

    let mut key_file = BufReader::new(File::open(&config.server.key_path)?);
    let mut keys = pkcs8_private_keys(&mut key_file)?;
    let private_key = PrivateKeyDer::Pkcs8(keys.remove(0).into());

    // Configura server QUIC
    let server_config = ServerConfig::with_single_cert(cert_chain, private_key)?;
    let endpoint = Endpoint::server(server_config, format!("0.0.0.0:{}", port).parse()?)?;
    println!("HTTP/3 server listening on QUIC port {}", port);

    // Mantieni il server in esecuzione
    while let Some(conn) = endpoint.accept().await {
        tokio::spawn(async move {
            let _ = conn.await;
        });
    }

    Ok(())
}