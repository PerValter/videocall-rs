use anyhow::{anyhow, Result};
use clap::Parser;
use quinn::{ClientConfig, Endpoint};
use rustls::RootCertStore;
use tracing::debug;
use std::{
    error::Error, fs, net::SocketAddr, net::ToSocketAddrs, path::PathBuf, sync::Arc,
    time::Instant,
};
use url::Url;

pub const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];

/// Constructs a QUIC endpoint configured for use a client only.
///
/// ## Args
///
/// - server_certs: list of trusted certificates.
pub fn make_client_endpoint(
    bind_addr: SocketAddr,
    server_certs: &[&[u8]],
) -> Result<Endpoint, Box<dyn Error>> {
    let client_cfg = configure_client(server_certs)?;
    let mut endpoint = Endpoint::client(bind_addr)?;
    endpoint.set_default_client_config(client_cfg);
    Ok(endpoint)
}

/// Builds default quinn client config and trusts given certificates.
///
/// ## Args
///
/// - server_certs: a list of trusted certificates in DER format.
fn configure_client(server_certs: &[&[u8]]) -> Result<ClientConfig, Box<dyn Error>> {
    let mut certs = rustls::RootCertStore::empty();
    for cert in server_certs {
        certs.add(&rustls::Certificate(cert.to_vec()))?;
    }

    let client_config = ClientConfig::with_root_certificates(certs);
    Ok(client_config)
}

/// Connects to a QUIC server.
///
/// ## Args
///
/// - opt: command line options.
pub async fn connect(opt: &Opt, roots: &RootCertStore) -> Result<quinn::Connection> {
    // let remote = opt
    //     .url
    //     .to_string()
    //     .to_socket_addrs()?
    //     .next()
    //     .ok_or_else(|| anyhow!("couldn't resolve to an address"))?;
    let mut client_crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(roots.clone())
        .with_no_client_auth();
    client_crypto.alpn_protocols = ALPN_QUIC_HTTP.iter().map(|&x| x.into()).collect();
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
    roots: RootCertStore,
    connection: Option<quinn::Connection>,
}

impl Client {
    /// Initialize a new QUIC client and load trusted certificates.
    ///
    /// ## Args
    ///
    /// - options: command line options.
    pub fn new(options: Opt) -> Result<Client> {
        let mut roots = rustls::RootCertStore::empty();
        if let Some(ca_path) = &options.ca {
            roots.add(&rustls::Certificate(fs::read(ca_path)?))?;
        }
        Ok(Client {
            options,
            roots,
            connection: None,
        })
    }

    pub async fn connect(&mut self) -> Result<()> {
        let conn = connect(&self.options, &self.roots).await?;
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
