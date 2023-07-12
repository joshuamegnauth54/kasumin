use crate::server_error::EnvelopeError;

use super::server_error::{ServerError, ServerErrorKind};
use bytes::{buf, Buf, BufMut, Bytes, BytesMut};
use rmp_serde::decode::Error as DecodeError;
use std::{net::SocketAddr, num::NonZeroU32};
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, BufReader, BufStream, BufWriter},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::{mpsc, oneshot, watch},
};
use yohane::{KasuminRequest, KasuminResponse};

const ENVELOPE_HEADER: &[u8] = b"kasu:";
const ENVELOPE_FOOTER: &[u8] = b"\r\n";
// This is 11 bytes so I don't know why I calculated it.
const ENVELOPE_SIZE: usize =
    ENVELOPE_HEADER.len() + std::mem::size_of::<u32>() + ENVELOPE_FOOTER.len();
// Length of the deserialized [ConnectRequest] struct.
const CONN_RECV_LEN: usize = 256;

/// Client connected to the music server.
///
/// Clients are asynchronous TCP streams that encode and decode frames to
/// communicate with the server. Valid clients must provide a name to identify
/// itself, but that name doesn't have to be unique as UUIDs are provided to
/// separate clients. Clients are "trusted" once registered because Kasumin is
/// intended to be used locally.
#[derive(Debug)]
pub(super) struct Client {
    /// Client's provided name which does not have to be unique
    name: String,
    /// UUID provided by the server
    uuid: String,
    /// Raw, buffered stream to communicate with the client
    /// Bytes are decoded or encoded into [KasuminMessage]s. Stream errors send
    /// a signal to the server to deregister this client.
    stream_recv: BufReader<OwnedReadHalf>,
    /// Send half of the stream for client communication
    stream_send: BufWriter<OwnedWriteHalf>,
    /// Buffer to decode Kasumin's communication frames
    buf: BytesMut,
    /// Server message receiver. Clients don't need one on one access to the
    /// server, but they do need to be synced
    server_recv: watch::Receiver<KasuminResponse>,
    /// Send structured messages to the server
    server_send: mpsc::Sender<KasuminRequest>,
}

impl Client {
    #[tracing::instrument]
    pub async fn connect(
        // Client stream
        stream: TcpStream,
        // Send updates to the server
        server_send: mpsc::Sender<KasuminRequest>,
        // Receive updates from the server
        server_recv: watch::Receiver<KasuminResponse>,
        // Clients that successfully connect need to send their names and receive a UUID.
        name_send: oneshot::Sender<KasuminRequest>,
        // Buffer capacity
        limit: NonZeroU32,
    ) -> Result<Self, ServerError> {
        // Try to get an envelope and name from the client as part of the connection
        // process
        let buf_cap = limit.get().try_into().expect("usize < u32");
        let (stream_recv, stream_send) = stream.into_split();
        let stream_recv = BufReader::with_capacity(buf_cap, stream_recv);
        let stream_send = BufWriter::with_capacity(buf_cap, stream_send);

        // Connection handshake; client should send its name
        let data_len = Client::decode_envelope(&mut stream, limit)
            .await
            .map_err(|kind| ServerError::new(kind, None, None))?;

        // Send name and receive a UUID
        let (uuid_send, uuid_recv) = oneshot::channel();

        // Client fields
        let buf = BytesMut::with_capacity(buf_cap);

        Ok(Self {
            name,
            uuid,
            stream_send,
            stream_recv,
            buf,
            server_recv,
            server_send,
        })
    }

    pub async fn server_frames(&mut self) {
        while self.server_recv.changed().await.is_ok() {}
    }

    pub async fn request_frames(&mut self, limit: NonZeroU32) -> Result<(), ServerError> {
        loop {
            self.buf.clear();

            // Decode the length of the frame from the envelope. Zero length data is always
            // invalid.
            let frame_len = Client::decode_envelope(&mut self.stream_recv, limit)
                .await
                .map_err(|kind| {
                    ServerError::new(kind, Some(self.name.as_ref()), Some(self.uuid.as_ref()))
                })?;

            // Read the exact amount of bytes specified from the frame length
            let read_len = self
                .stream_recv
                .read_exact(&mut self.buf[..frame_len])
                .await
                .map_err(|e| {
                    ServerError::new(e.into(), Some(self.name.as_ref()), Some(self.uuid.as_ref()))
                })?;

            // Decode MessagePack and send it to the server.
            let request = Client::decode_message(&self.buf[..frame_len.into()])?;
            self.send(request).await?;
        }
    }

    async fn send(&mut self, request: KasuminRequest) -> Result<(), ServerError> {

    }

    async fn decode_envelope<R>(
        stream: &mut R,
        limit: NonZeroU32,
    ) -> Result<NonZeroU32, ServerErrorKind>
    where
        R: AsyncRead + AsyncReadExt + Unpin,
    {
        // The transport envelope is `kasu:u32\r\n`
        let mut envelope = [0u8; ENVELOPE_SIZE];
        let read_len = stream
            .read_exact(&mut envelope)
            .await
            .map_err(|e| ServerErrorKind::Envelope(e.into()))?;

        if read_len == ENVELOPE_SIZE {
            let header_len = ENVELOPE_HEADER.len();
            let u32_len = std::mem::size_of::<u32>();
            // let footer_len = ENVELOPE_FOOTER.len();

            // SAFETY: `envelope`'s length is always `ENVELOPE_SIZE`
            match (
                // Header
                &envelope[..header_len],
                // Integer
                &envelope[header_len..u32_len],
                // Footer
                &envelope[u32_len..],
            ) {
                (ENVELOPE_HEADER, size_bytes, ENVELOPE_FOOTER) => {
                    // Network byte order is big endian
                    let size_bytes = size_bytes
                        .try_into()
                        .expect("`size_bytes` literally can't be != the size of a u32");
                    let data_length = u32::from_be_bytes(size_bytes);

                    NonZeroU32::new(data_length)
                        .ok_or_else(|| ServerErrorKind::Envelope(EnvelopeError::Zero(data_length)))
                        .and_then(|data_length| {
                            if data_length > limit {
                                Err(ServerErrorKind::Envelope(EnvelopeError::DataTooLarge {
                                    limit,
                                    data_length,
                                }))
                            } else {
                                Ok(data_length)
                            }
                        })
                }
                // Any other data except the exact envelope is invalid
                _ => Err(ServerErrorKind::Envelope(EnvelopeError::WrongFormat(
                    // Cow to String since the data has to be owned anyway (`envelope` is bytes on
                    // the stack)
                    String::from_utf8_lossy(&envelope).to_string(),
                ))),
            }
        } else {
            Err(EnvelopeError::InvalidSize(read_len).into())
        }
    }

    // Decode a request from the client to send to the server.
    #[inline]
    fn decode_message(msgpack: &[u8]) -> Result<KasuminRequest, ServerErrorKind> {
        rmp_serde::decode::from_read(msgpack).map_err(ServerErrorKind::Decode)
    }

    // Encode a response from the server into bytes to send to the client.
    #[inline]
    fn encode_message(&mut self) -> Result<KasuminResponse, ServerError> {}
}
