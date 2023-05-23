use std::{fmt::Debug, io};
use tokio::net::{self, TcpListener, ToSocketAddrs};
use tracing::{debug, error, info};
use yohane::{KasuminRequest, KasuminResponse};

pub(crate) struct KasuminServer;

impl KasuminServer {
    #[tracing::instrument]
    pub async fn start<A>(address: A) -> io::Result<()>
    where
        A: ToSocketAddrs + Debug,
    {
        debug!("Looking up address prior to binding a socket.");
        match net::lookup_host(&address).await {
            Ok(hosts) => {
                for host in hosts {
                    info!("Host address: {host}")
                }
            }
            Err(e) => error!("Failed to look up address(es): {address:?}\n\tError: {e}"),
        }

        let server = TcpListener::bind(address).await?;
        let address = server.local_addr()?;
        info!("Listening on {address}");

        Ok(())
    }
}
