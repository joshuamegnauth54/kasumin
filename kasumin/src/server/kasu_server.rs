use super::connection::Client;
use std::{collections::HashMap, fmt::Debug, io};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::{self, TcpListener, TcpStream, ToSocketAddrs},
    sync::{mpsc, watch, oneshot},
};
use tracing::{debug, error, info};
use yohane::{KasuminRequest, KasuminResponse};

#[derive(Debug)]
pub(crate) struct KasuminServer {
    /// Channel for clients to send structured messages
    client_send: mpsc::UnboundedSender<KasuminRequest>,
    /// Structured (parsed) messages received from a client
    recv: mpsc::UnboundedReceiver<KasuminRequest>,
    /// Responses to send to client to synchronize their state (i.e. when the
    /// volume or song is changed)
    sync: watch::Sender<KasuminResponse>,
    /// Watcher for clients
    sync_recv: watch::Receiver<KasuminResponse>,
    /// Confirmed and alive clients
    clients: HashMap<String, LocalClient>,
}

impl KasuminServer {
    #[inline]
    pub fn new(buffer_size: usize) -> Self {
        let (client_send, recv) = mpsc::unbounded_channel();
        let (sync, sync_recv) = watch::channel(KasuminResponse {
            message: yohane::ResponseKind::Ready,
        });
        Self {
            client_send,
            recv,
            sync,
            sync_recv,
            clients: HashMap::new(),
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
            tokio::spawn(KasuminServer::handle_client(client));
        }

        Ok(())
    }

    #[tracing::instrument]
    async fn handle_client(client: TcpStream) {
        let (name_send, name_recv) = oneshot::channel();
        Client::connect(client, self.sync_recv.clone(), name_send, self.buffer_size).await?;
    }
}
