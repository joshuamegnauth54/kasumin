//! Kasumin's music player.

use std::{collections::BinaryHeap, sync::RwLock};

use rodio::OutputStreamHandle;

use crate::{
    library::Library,
    playlist::{Playlist, PlaylistItem},
};

pub struct Player<'tracks, D> {
    stream: OutputStreamHandle,
    library: Library<D>,
    playlist: Playlist<'tracks>,
}

impl<'tracks, D> Player<'tracks, D> {
    pub fn from_stream_handle(
        stream: OutputStreamHandle,
        library: Library<D>,
        playlist: Option<BinaryHeap<PlaylistItem<'tracks>>>,
    ) -> Self {
        Self {
            stream,
            library,
            playlist: playlist.unwrap_or_default().into(),
        }
    }
}
