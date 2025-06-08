use quinn::rustls::pki_types::PrivateKeyDer;
use quinn::{Endpoint, ServerConfig};
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let address: SocketAddr = "127.0.0.1:4433".parse().unwrap();
    
    let cert = fs::read("cert.der").unwrap();
    let key = fs::read("key.der").unwrap();
    let server_config = ServerConfig::with_single_cert(vec![cert.into()], PrivateKeyDer::try_from(key).unwrap())?;
    let endpoint = Endpoint::server(server_config, address)?;
    println!("Listening on {}", endpoint.local_addr()?);
    
    while let Some(conn) = endpoint.accept().await{
        println!("Connection incoming from: {}", conn.remote_address());
        tokio::spawn(async move {
            let conn = conn.accept().unwrap();
            handle_connection(conn).await;
        });
    }
    Ok(())
}

async fn handle_connection(conn: quinn::Connecting) {
    // Handle the connection here
    match conn.await {
        Ok(data) => {
            while let (mut send, mut recv) = data.accept_bi().await.unwrap(){
                let msg = recv.read_to_end(1024).await.unwrap_or_default();
                let msg_str = String::from_utf8_lossy(&msg);
                println!("Recevied: {}", msg_str);
                
                send.write_all(&msg).await.unwrap();
                println!("Echoed message back.");
            }
        },
        Err(e) => {
            eprintln!("Failed to establish connection: {}", e)
        },
    }
}
