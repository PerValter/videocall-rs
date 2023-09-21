use anyhow::{anyhow, Result};
use clap::Parser;
use quinn::{ClientConfig, Endpoint};
use rustls::RootCertStore;
use tracing::debug;
use std::{
    error::Error, fs, net::SocketAddr, path::PathBuf, sync::Arc,
    time::Instant,
};
use url::Url;

pub const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];


/// Connects to a QUIC server.
///
/// ## Args
///
/// - opt: command line options.
pub async fn connect(opt: &Opt) -> Result<quinn::Connection> {
    let remote = opt
        .url
        .socket_addrs(|| Some(443))?
        .first()
        .ok_or_else(|| anyhow!("couldn't resolve to an address"))?.to_owned();
    let mut root_store = rustls::RootCertStore::empty();
    root_store.add_trust_anchors(
        webpki_roots::TLS_SERVER_ROOTS
            .iter()
            .map(|ta| {
                rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            })
    );
    let mut client_crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let alpn: Vec<Vec<u8>> = vec![
        b"h3".to_vec(),
        b"h3-32".to_vec(),
        b"h3-31".to_vec(),
        b"h3-30".to_vec(),
        b"h3-29".to_vec(),
    ];
    client_crypto.alpn_protocols = alpn;
    if opt.keylog {
        client_crypto.key_log = Arc::new(rustls::KeyLogFile::new());
    }
    let client_config = quinn::ClientConfig::new(Arc::new(client_crypto));
    
    let mut endpoint = quinn::Endpoint::client("[::]:0".parse().unwrap())?;
    endpoint.set_default_client_config(client_config);
    let start = Instant::now();
    let host = opt
        .host
        .as_ref()
        .map_or_else(|| opt.url.host_str(), |x| Some(x))
        .ok_or_else(|| anyhow!("no hostname specified"))?;
    eprintln!("connecting to {host} at {remote}");
    let conn = endpoint
        .connect(remote, host)?
        .await
        .map_err(|e| anyhow!("failed to connect: {}", e))?;
    eprintln!("connected at {:?}", start.elapsed());
    Ok(conn)
}

/// HTTP/0.9 over QUIC client
#[derive(Parser, Debug)]
#[clap(name = "client")]
pub struct Opt {
    /// Perform NSS-compatible TLS key logging to the file specified in `SSLKEYLOGFILE`.
    #[clap(long = "keylog")]
    keylog: bool,

    url: Url,

    /// Override hostname used for certificate verification
    #[clap(long = "host")]
    host: Option<String>,

    /// Certificate authority to trust, in DER format
    #[clap(long = "ca")]
    ca: Option<PathBuf>,
}

pub struct Client {
    options: Opt,
    connection: Option<quinn::Connection>,
}

impl Client {
    /// Initialize a new QUIC client and load trusted certificates.
    ///
    /// ## Args
    ///
    /// - options: command line options.
    pub fn new(options: Opt) -> Result<Client> {
        Ok(Client {
            options,
            connection: None,
        })
    }

    pub async fn connect(&mut self) -> Result<()> {
        let conn = connect(&self.options).await?;
        self.connection = Some(conn);
        debug!("connected to server {}", self.options.url);
        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        if let Some(conn) = &self.connection {
            let (mut send, _recv) = conn
                .open_bi()
                .await
                .map_err(|e| anyhow!("failed to open stream: {}", e))?;
            send.finish()
                .await
                .map_err(|e| anyhow!("failed to shutdown stream: {}", e))?;
        }

        Ok(())
    }
}
