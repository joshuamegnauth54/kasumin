use super::playlist::PlaylistItem;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, RwLock,
};

const PLAYLIST_WRITE_ERR: &str = "updating the playlist failed (this shouldn't happen)";

pub struct PlaylistIter<'tracks> {
    current_tracks: Arc<RwLock<Vec<PlaylistItem<'tracks>>>>,
    index: Arc<AtomicUsize>,
}

impl<'tracks> PlaylistIter<'tracks> {
    #[inline]
    pub fn new(tracks: &Arc<RwLock<Vec<PlaylistItem<'tracks>>>>, pos: &Arc<AtomicUsize>) -> Self {
        Self {
            current_tracks: Arc::clone(tracks),
            index: Arc::clone(pos),
        }
    }

    #[inline]
    pub fn current(&self) -> Option<PlaylistItem<'tracks>> {
        self.current_tracks
            .read()
            .expect(PLAYLIST_WRITE_ERR)
            .get(self.index.load(Ordering::SeqCst))
            .copied()
    }
}

impl<'tracks> Iterator for PlaylistIter<'tracks> {
    type Item = PlaylistItem<'tracks>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index.fetch_add(1, Ordering::SeqCst);
        self.current_tracks
            .read()
            .expect(PLAYLIST_WRITE_ERR)
            .get(index)
            .copied()
    }
}
