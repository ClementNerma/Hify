use std::{cmp::Ordering, collections::HashMap, hash::Hash};

use indexmap::IndexMap;

use super::{Album, AlbumID, Artist, ArtistID, Genre, GenreID, Index, IndexCache, Track, TrackID};

enum Either<'a, K, V> {
    A(&'a IndexMap<K, V>),
    B(HashMap<K, &'a V>),
}

impl<K: Hash + Eq, V> Either<'_, K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        match self {
            Either::A(map) => map.get(key),
            Either::B(map) => map.get(key).copied(),
        }
    }
}

pub struct CmpIndex<'a> {
    tracks: Either<'a, TrackID, Track>,
    albums: Either<'a, AlbumID, Album>,
    artists: Either<'a, ArtistID, Artist>,
    genres: Either<'a, GenreID, Genre>,
}

impl<'a> CmpIndex<'a> {
    pub fn new(index: &'a IndexCache) -> Self {
        Self {
            tracks: Either::A(&index.tracks),
            albums: Either::A(&index.albums),
            artists: Either::A(&index.artists),
            genres: Either::A(&index.genres),
        }
    }

    pub fn build(index: &'a Index) -> Self {
        Self {
            tracks: Either::B(index.tracks.iter().map(|track| (track.id, track)).collect()),
            albums: Either::B(index.albums.iter().map(|album| (album.id, album)).collect()),
            artists: Either::B(
                index
                    .artists
                    .iter()
                    .map(|artist| (artist.id, artist))
                    .collect(),
            ),
            genres: Either::B(index.genres.iter().map(|genre| (genre.id, genre)).collect()),
        }
    }

    pub fn cmp_tracks(&self, a: &Track, b: &Track) -> Ordering {
        self.cmp_albums_by_id(a.tags.album_id, b.tags.album_id)
            .then_with(|| a.tags.disc_number.cmp(&b.tags.disc_number))
            .then_with(|| a.tags.track_number.cmp(&b.tags.track_number))
            .then_with(|| a.tags.title.cmp(&b.tags.title))
            .then_with(|| a.relative_path.cmp(&b.relative_path))
    }

    pub fn cmp_tracks_by_id(&self, a: TrackID, b: TrackID) -> Ordering {
        self.cmp_tracks(self.tracks.get(&a).unwrap(), self.tracks.get(&b).unwrap())
    }

    pub fn cmp_albums(&self, a: &Album, b: &Album) -> Ordering {
        a.name.cmp(&b.name).then_with(|| {
            for (a, b) in a.artists_id.iter().zip(b.artists_id.iter()) {
                let ord = self.cmp_artists_by_id(*a, *b);

                if ord != Ordering::Equal {
                    return ord;
                }
            }

            a.artists_id.len().cmp(&b.artists_id.len())
        })
    }

    pub fn cmp_albums_by_id(&self, a: AlbumID, b: AlbumID) -> Ordering {
        self.cmp_albums(self.albums.get(&a).unwrap(), self.albums.get(&b).unwrap())
    }

    pub fn cmp_artists(a: &Artist, b: &Artist) -> Ordering {
        a.name.cmp(&b.name)
    }

    pub fn cmp_artists_by_id(&self, a: ArtistID, b: ArtistID) -> Ordering {
        Self::cmp_artists(self.artists.get(&a).unwrap(), self.artists.get(&b).unwrap())
    }

    pub fn cmp_genres(a: &Genre, b: &Genre) -> Ordering {
        a.name.cmp(&b.name)
    }

    pub fn cmp_genres_by_id(&self, a: GenreID, b: GenreID) -> Ordering {
        Self::cmp_genres(self.genres.get(&a).unwrap(), self.genres.get(&b).unwrap())
    }
}
