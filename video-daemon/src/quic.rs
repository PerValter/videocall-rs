use anyhow::{anyhow, Result};
use clap::Parser;

use std::{path::PathBuf, sync::Arc, time::Instant};
use tracing::{debug, info};
use url::Url;

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
        .ok_or_else(|| anyhow!("couldn't resolve to an address"))?
        .to_owned();
    let mut root_store = rustls::RootCertStore::empty();
    root_store.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.iter().map(|ta| {
        rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));
    let mut client_crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let alpn = vec![b"hq-29".to_vec()];
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
    info!("connecting to {host} at {remote}");
    let conn = endpoint
        .connect(remote, host)?
        .await
        .map_err(|e| anyhow!("failed to connect: {}", e))?;
    info!("connected at {:?}", start.elapsed());
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

    pub async fn send(&mut self, data: &[u8]) -> Result<()> {
        let conn = self
            .connection
            .as_mut()
            .ok_or_else(|| anyhow!("not connected"))?;
        let mut stream = conn.open_uni().await?;
        stream.write_all(data).await?;
        stream.finish().await?;
        debug!("sent {} bytes", data.len());
        Ok(())
    }
}
