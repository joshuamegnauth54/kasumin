use crate::server_error::EnvelopeError;

use super::server_error::{ServerError, ServerErrorKind};
use bytes::{buf, Buf, BufMut, Bytes, BytesMut};
use rmp_serde::decode::Error as DecodeError;
use std::{net::SocketAddr, num::NonZeroU32};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, BufStream},
    net::TcpStream,
    sync::{mpsc, watch, oneshot},
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
    stream: BufStream<TcpStream>,
    /// Buffer to decode Kasumin's communication frames
    buf: BytesMut,
    /// Server message receiver
    server_recv: watch::Receiver<KasuminResponse>,
    /// Server structured message sender
    send: mpsc::Sender<KasuminRequest>
}

impl Client {
    #[tracing::instrument]
    pub async fn connect(
        stream: TcpStream,
        server_recv: watch::Receiver<KasuminResponse>,
        uuid_send: oneshot::Sender<KasuminRequest>,
        limit: NonZeroU32,
    ) -> Result<Self, ServerError> {
        // Try to get an envelope and name from the client as part of the connection
        // process
        let buf_cap = limit.get().try_into().expect("usize < u32");
        let stream = BufStream::with_capacity(buf_cap, buf_cap, stream);

        // Connection handshake; client should send its name
        let data_len = Client::decode_envelope(&mut stream, limit)
            .await
            .map_err(|kind| ServerError::new(kind, None, None))?;

        // Client fields
        let buf = BytesMut::with_capacity(buf_cap);
    }

    pub async fn process_frames(&mut self, limit: NonZeroU32) -> Result<(), ServerError> {
        loop {
            self.buf.clear();

            // Decode the length of the frame from the envelope. Zero length data is always
            // invalid.
            let frame_len = Client::decode_envelope(&mut self.stream, limit)
                .await
                .map_err(|kind| {
                    ServerError::new(kind, Some(self.name.as_ref()), Some(self.uuid.as_ref()))
                })?;
            loop {
                let request = self.stream.read(&mut self.buf).await.map_err(|e| {
                    ServerError::new(e.into(), Some(self.name.as_ref()), Some(self.uuid.as_ref()))
                })?;
            }
        }
    }

    async fn decode_envelope(
        stream: &mut BufStream<TcpStream>,
        limit: NonZeroU32,
    ) -> Result<NonZeroU32, ServerErrorKind> {
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
    fn decode_message(msgpack: &[u8]) -> Result<Option<KasuminRequest>, ServerErrorKind> {
        rmp_serde::decode::from_read(msgpack)
            .map(Option::Some)
            .map_err(ServerErrorKind::Decode)
    }

    // Encode a response from the server into bytes to send to the client.
    #[inline]
    fn encode_message(&mut self) -> Result<KasuminResponse, ServerError> {}
}
