use crate::server_error::EnvelopeError;

use super::server_error::{ServerError, ServerErrorKind};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use rmp_serde::decode::Error as DecodeError;
use std::{net::SocketAddr, num::NonZeroU32};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, BufStream},
    net::TcpStream,
    sync::mpsc,
};
use yohane::{KasuminRequest, KasuminResponse};

const ENVELOPE_HEADER: [u8; 5] = b"kasu:";
const ENVELOPE_FOOTER: [u8; 2] = b"\r\n";
// This is 11 bytes so I don't know why I calculated it.
const ENVELOPE_SIZE: usize =
    ENVELOPE_HEADER.len() + std::mem::size_of::<u32>() + ENVELOPE_FOOTER.len();

/// Client connected to the music server.
///
/// Clients are asynchronous TCP streams that encode and decode frames to
/// communicate with the server. Valid clients must provide a name to identify
/// itself, but that name doesn't have to be unique as UUIDs are provided to
/// separate clients. Clients are "trusted" once registered because Kasumin is
/// intended to be used locally.
pub(crate) struct Client {
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
    server_recv: mpsc::UnboundedReceiver<KasuminMessage>,
}

impl Client {
    #[tracing::instrument]
    pub async fn connect(
        stream: TcpStream,
        server_recv: mpsc::UnboundedReceiver<KasuminMessage>,
    ) -> Result<Self, ServerError> {
        let stream = BufStream::new(stream);
        let buf = BytesMut::new();
    }

    pub async fn process_frames(&mut self) -> Result<(), ServerError> {
        'envelope: loop {
            // Decode the length of the frame from the envelope. Zero length data is always
            // invalid.
            let frame_len = self.decode_envelope().await.map_err(|kind| ServerError {
                client_id: Some(self.uuid.clone()),
                client_name: Some(self.name.clone()),
                kind,
            })?;
            'frame: loop {
                self.stream.read_buf(&mut self.buf)?;
            }
        }

        Ok(())
    }

    async fn decode_envelope(&mut self, limit: NonZeroU32) -> Result<NonZeroU32, ServerErrorKind> {
        // The transport envelope is `kasu:u32\r\n`
        let mut envelope = [0u8; ENVELOPE_SIZE];
        let read_len = self
            .stream
            .read_exact(&mut envelope.0)
            .await
            .map_err(|e| ServerErrorKind::Envelope(e.into()))?;

        if read_len == ENVELOPE_SIZE {
            let header_len = ENVELOPE_HEADER.len();
            let u32_len = std::mem::size_of::<u32>();
            // let footer_len = ENVELOPE_FOOTER.len();

            // SAFETY: `envelope`'s length is always `ENVELOPE_SIZE`
            match (
                // Header
                envelope[..header_len],
                // Integer
                envelope[header_len..u32_len],
                // Footer
                envelope[u32_len..],
            ) {
                (ENVELOPE_HEADER, size_bytes, ENVELOPE_FOOTER) => {
                    // Network byte order is big endian
                    let data_length = u32::from_be_bytes(size_bytes);
                    NonZeroU32::new(data_length)
                        .ok_or_else(|| {
                            ServerErrorKind::Envelope(EnvelopeError::ZeroOrNegative(data_length))
                        })
                        .and_then(|data_length| {
                            if data_length > data_length_limit {
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
                    String::from_utf8_lossy(&envelope),
                ))),
            }
        } else {
            Err(EnvelopeError::InvalidSize.into())
        }
    }

    // Decode a request from the client to send to the server.
    fn decode_message(buf: &BytesMut) -> Result<Option<KasuminRequest>, ServerError> {
        match rmp_serde::decode::from_read(buf) {
            Ok(message) => Ok(Some(message)),
            Err(e) => match e {},
        }
    }

    // Encode a response from the server into bytes to send to the client.
    fn encode_message(&mut self) -> Result<KasuminResponse, ServerError> {}
}
