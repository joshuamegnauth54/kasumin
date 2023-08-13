//! Kasumin's music player.

use std::collections::BinaryHeap;

use rodio::OutputStreamHandle;

use crate::playlist::{Playlist, PlaylistItem};

pub struct Player<'tracks> {
    stream: OutputStreamHandle,
    playlist: Playlist<'tracks>
}

impl<'tracks> Player<'tracks> {
    pub fn from_stream_handle(stream: OutputStreamHandle, tracks: Option<BinaryHeap<PlaylistItem<'tracks>>>) -> Self {
        Self {
            stream,
            playlist: tracks.unwrap_or_default().into()
        }
    }
}
