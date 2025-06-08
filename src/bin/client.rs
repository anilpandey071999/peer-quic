use anyhow::Result;
use quinn::{ClientConfig, Endpoint};
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .unwrap();

    let server_addr: SocketAddr = "127.0.0.1:4433".parse().unwrap();
    let local_addr: SocketAddr = "127.0.0.1:0".parse().unwrap();

    let mut root_cert_store = rustls::RootCertStore::empty();
    let cert = fs::read("cert.der")?;
    root_cert_store.add(rustls::pki_types::CertificateDer::from(cert))?;

    let client_config = ClientConfig::with_root_certificates(Arc::new(root_cert_store))?;
    let mut endpoint = Endpoint::client(local_addr)?;
    endpoint.set_default_client_config(client_config);

    println!("Connecting to {}...", server_addr);
    let connection = endpoint.connect(server_addr, "localhost")?.await?;
    println!("Connected!");

    let (mut send, mut rcev) = connection.open_bi().await.expect("Failed bro!!");
    let message = "Hello, QUIC!";
    send.write_all(message.as_bytes()).await?;
    println!("Sent: '{}'", message);
    send.finish()?;

    let response = rcev.read_to_end(1024).await?;
    println!("Recived echo: '{}'", String::from_utf8_lossy(&response));
    Ok(())
}
