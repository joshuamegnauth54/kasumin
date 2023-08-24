//! Kasumin library

use std::{collections::BTreeMap, sync::Arc};

use fst::Set;
use smallvec::SmallVec;
use tracing::{debug, trace};

#[derive(Clone)]
pub struct Artist {
    name: Arc<str>,
    albums: Vec<Album>,
}

#[derive(Clone)]
pub struct Album {
    title: Arc<str>,
    date: Arc<str>,
    tracks: Vec<Track>,
}

#[derive(Clone)]
pub struct Track {
    title: Arc<str>,
    link: Arc<str>,
}

pub struct Library<D> {
    // All artists in the library.
    // Duplicate artists should be handled via the file's metadata.
    // For example, if two bands share the same name then the tags (e.g. ID3) should disambiguate them.
    // As far as I know, there isn't a good way to do this with arbitrary libraries and tagging schemes.
    artists: BTreeMap<Arc<str>, Artist>,
    // SmallVec justification:
    // https://users.rust-lang.org/t/when-is-it-morally-correct-to-use-smallvec/46375/5
    // "You know that your vector will hold a very small number of items most of the time, and only rarely need to hold more. (SmallVec may speed up the
    // common case a lot, while slightly slowing down the uncommon case.)"
    //
    // At least one album or track will exist per key. Instead of storing a full Vec per key, an inline SmallVec should be better for now as
    // it optimizes for the common case (one album or track).
    albums: BTreeMap<Arc<str>, SmallVec<[Album; 1]>>,
    tracks: BTreeMap<Arc<str>, SmallVec<[Track; 1]>>,
    // Finite state tranducers
    search: Search<D>,
}

impl<D> Library<D> {
    #[tracing::instrument(skip_all)]
    pub fn new(artists: impl IntoIterator<Item = Artist>) -> Self {
        trace!("Constructing library");

        let artists: BTreeMap<Arc<str>, Artist> = artists
            .into_iter()
            .map(|artist| (Arc::clone(&artist.name), artist))
            .collect();
        trace!("Processed {} artists", artists.len());

        let albums = artists
            .values()
            .flat_map(|artist| artist.albums.iter())
            .fold(
                BTreeMap::<Arc<str>, SmallVec<[Album; 1]>>::new(),
                |mut map, album| {
                    map.entry(Arc::clone(&album.title))
                        .or_default()
                        .push(album.clone());
                    map
                },
            );
        trace!("Processed {} albums", albums.len());

        let tracks = albums
            .values()
            .flat_map(|albums| albums.iter().flat_map(|album| album.tracks.iter()))
            .fold(
                BTreeMap::<Arc<str>, SmallVec<[Track; 1]>>::new(),
                |mut map, track| {
                    map.entry(Arc::clone(&track.title))
                        .or_default()
                        .push(track.clone());
                    map
                },
            );
        trace!("Processed {} tracks", tracks.len());

        let search = Search::new();

        Self {
            artists,
            albums,
            tracks,
            search,
        }
    }

    // pub fn view(&self)
}

pub struct Search<D> {
    artists: Set<D>,
    albums: Set<D>,
    titles: Set<D>,
    dates: Set<D>,
}

impl<D> Search<D> {
    fn new() -> Self {
        unimplemented!()
    }
}
