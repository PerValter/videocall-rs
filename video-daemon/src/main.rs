use clap::Parser;
use rustls::quic;
use std::sync::mpsc::channel;
use video_daemon::camera;
use video_daemon::quic::{Client, Opt};

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .finish(),
    )
    .unwrap();
    let opt = Opt::parse();
    let mut client = Client::new(opt).expect("failed to create client");
    client.connect().await.expect("failed to connect");
    let (quic_tx, quic_rx) = channel::<Vec<u8>>();
    let camera_task = tokio::spawn(async {
        camera::start(quic_tx).await;
    });
    while let Ok(data) = quic_rx.recv() {
        client.send(&data).await.expect("failed to send");
    }
}
