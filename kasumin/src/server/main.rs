mod kasu_server;

use kasu_server::KasuminServer;
use tracing::info;

#[tokio::main]
#[tracing::instrument]
async fn main() {
    // Register a subscriber for tracing.
    let subscriber = tracing_subscriber::fmt()
        .pretty()
        .with_line_number(true)
        .with_thread_ids(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Starting Kasumin music daemon");
    KasuminServer::start("127.0.0.1:6666").await.unwrap();
}
