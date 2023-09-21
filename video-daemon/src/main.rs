use clap::Parser;
use video_daemon::client::{Client, Opt};

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
    let code = {
        if let Err(e) = client.run().await {
            eprintln!("ERROR: {e}");
            1
        } else {
            0
        }
    };
    ::std::process::exit(code);
}
