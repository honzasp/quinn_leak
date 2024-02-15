use anyhow::{Result, Context as _};
use bytes::Bytes;
use std::net::{SocketAddr, Ipv4Addr};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = init_endpoint()?;
    let mut tasks = tokio::task::JoinSet::new();
    println!("Accepting connections");
    loop {
        tokio::select! {
            Some(conn) = endpoint.accept() => {
                println!("Accepted connection");
                tasks.spawn(handle_conn(conn));
            },
            Some(res) = tasks.join_next() => {
                if let Err(err) = res.expect("Task crashed") {
                    println!("Connection failed: {:?}", err);
                } else {
                    println!("Connection terminated");
                }
            },
            else => return Ok(()),
        }
    }
}

async fn handle_conn(conn: quinn::Connecting) -> Result<()> {
    let conn = conn.await
        .context("Could not establish QUIC connection")?;
    loop {
        for _ in 0..200 {
            conn.send_datagram(Bytes::copy_from_slice(b"hello"))
                .context("Could not send datagram")?;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

fn init_endpoint() -> Result<quinn::Endpoint> {
    let cert = rcgen::generate_simple_self_signed(vec![])
        .context("Could not generate self-signed certificate")?;
    let cert_der = cert.serialize_der()
        .context("Could not serialize certificate into DER")?;
    let private_key_der = cert.serialize_private_key_der();

    let config = quinn::ServerConfig::with_single_cert(
        vec![rustls::Certificate(cert_der)],
        rustls::PrivateKey(private_key_der),
    ).context("Could not create QUIC configuration")?;

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 2024));
    quinn::Endpoint::server(config, addr)
        .context("Could not create QUIC endpoint")
}
