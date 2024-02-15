use anyhow::{Result, Context as _};
use std::net::{SocketAddr, Ipv4Addr};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = init_endpoint()?;
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 2024));

    println!("Connecting");
    let conn = endpoint.connect(addr, "unused.server.name")
        .context("Could not create QUIC connection")?
        .await.context("Could not establish QUIC connection")?;

    println!("Connected");
    let mut i = 0;
    loop {
        let _datagram = conn.read_datagram().await
            .context("Could not receive datagram")?;
        i += 1;
        if i % 1_000_000 == 0 {
            println!("{}", i);
        }
    }
}

fn init_endpoint() -> Result<quinn::Endpoint> {
    let crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(Arc::new(danger::NoCertificateVerification))
        .with_no_client_auth();

    let config = quinn::ClientConfig::new(Arc::new(crypto));
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0));
    let mut endpoint = quinn::Endpoint::client(addr)
        .context("Could not create QUIC endpoint")?;
    endpoint.set_default_client_config(config);
    Ok(endpoint)
}

mod danger {
    pub struct NoCertificateVerification;

    impl rustls::client::ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(
            &self,
            _end_entity: &rustls::Certificate,
            _intermediates: &[rustls::Certificate],
            _server_name: &rustls::ServerName,
            _scts: &mut dyn Iterator<Item = &[u8]>,
            _ocsp: &[u8],
            _now: std::time::SystemTime,
        ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
            Ok(rustls::client::ServerCertVerified::assertion())
        }
    }
}
