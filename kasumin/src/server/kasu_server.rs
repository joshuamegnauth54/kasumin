use std::{fmt::Debug, io, collections::HashMap};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::{self, TcpListener, ToSocketAddrs},
    sync::{mpsc, watch}
};
use tracing::{debug, error, info};
use yohane::{KasuminRequest, KasuminResponse};

pub(crate) struct KasuminServer {
    /// Structured messages to send to a client
    client_send: mpsc::UnboundedSender<KasuminMessage>,
    /// Structured (parsed) messages received from a client
    client_recv: mpsc::UnboundedReceiver<KasuminMessage>,
    // client_sync: watch::Receiver<KasuminMessage>,
    /// Confirmed and alive clients
    clients: HashMap<String, Client>,
}

impl KasuminServer {
    #[inline]
    pub fn new(buffer_size: usize) -> Self {
        let (client_send, client_recv) = mpsc::unbounded_channel(buffer_size);
        Self {
            client_send,
            client_recv,
            clients: HashMap::new()
        }
    }

    #[tracing::instrument]
    pub async fn start<A>(&mut self, address: A) -> io::Result<()>
    where
        A: ToSocketAddrs + Debug,
    {
        debug!("Looking up address prior to binding a socket.");
        match net::lookup_host(&address).await {
            Ok(hosts) => {
                for host in hosts {
                    info!("Resolved host address: {host}")
                }
            }
            Err(e) => error!("Failed to look up address(es): {address:?}\n\tError: {e}"),
        }

        let server = TcpListener::bind(address).await?;
        let address = server.local_addr()?;
        info!("Listening on {address}");

        while let (client, client_addr) = server.accept().await? {
            info!("Client `{client_addr}` connected");
            tokio::spawn(self.handle_client(client));
        }

        Ok(())
    }
}
