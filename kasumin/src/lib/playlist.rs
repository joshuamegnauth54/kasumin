//! Kasumin's playlist
//!
//! shuffle

use std::{cmp::Ordering, collections::BinaryHeap};

use super::track::TrackMetadata;

/// Song in a [Playlist].
#[derive(Clone, Copy)]
pub struct PlaylistItem<'track> {
    /// Current position in the playlist.
    /// A [u32] should be enough for a playlist - a [u16] may be fine too.
    pos: u32,
    song: TrackMetadata<'track>,
}

impl PartialEq for PlaylistItem<'_> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for PlaylistItem<'_> {}

impl PartialOrd for PlaylistItem<'_> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.pos.partial_cmp(&other.pos)
    }
}

impl Ord for PlaylistItem<'_> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.pos.cmp(&other.pos)
    }
}

/// An orderable queue of tracks.
#[derive(Clone, Default)]
pub struct Playlist<'tracks> {
    queue: BinaryHeap<PlaylistItem<'tracks>>,
}

impl<'tracks> Playlist<'tracks> {
    #[inline]
    pub fn insert(&mut self, track: PlaylistItem<'tracks>) {
        self.queue.push(track);
    }

    /// Return tracks in the order which they will be played.
    #[inline]
    pub fn ordered_tracks(&self) -> Vec<PlaylistItem<'tracks>> {
        self.queue.clone().into_sorted_vec()
    }
}

impl<'tracks> From<BinaryHeap<PlaylistItem<'tracks>>> for Playlist<'tracks> {
    #[inline]
    fn from(value: BinaryHeap<PlaylistItem<'tracks>>) -> Self {
        Self { queue: value }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    use super::{Playlist, PlaylistItem};
    use crate::track::TrackMetadata;

    fn empty_track(pos: u32) -> PlaylistItem<'static> {
        PlaylistItem {
            pos,
            song: TrackMetadata { path: "" },
        }
    }

    #[test]
    fn proper_playlist_ordering_simple() {
        let playlist: Playlist<'static> =
            (0..10).map(empty_track).collect::<BinaryHeap<_>>().into();

        assert!(
            playlist
                .ordered_tracks()
                .iter()
                .map(|track| track.pos)
                .eq(0..10),
            "playlist should be in order",
        );
    }

    #[test]
    fn proper_playlist_ordering_complex() {
        unimplemented!("Do this later")
    }
}
