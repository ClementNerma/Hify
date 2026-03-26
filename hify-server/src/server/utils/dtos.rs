use serde::Serialize;

use crate::{
    index::{Album, Artist, Genre, IndexCache, Rating, Track},
    manager::Ratings,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistCompleteInfos {
    artist: Artist,
    albums_count: usize,
    tracks_count: usize,
}

impl ArtistCompleteInfos {
    pub fn new(artist: Artist, index: &IndexCache) -> Self {
        Self {
            albums_count: index.artists_albums.get(&artist.id).unwrap().len(),
            tracks_count: index
                .artists_track_participations
                .get(&artist.id)
                .unwrap()
                .len(),
            artist,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumCompleteInfos {
    album: Album,
    artists: Vec<ArtistCompleteInfos>,
    genres: Vec<GenreCompleteInfos>,
    tracks_count: usize,
}

impl AlbumCompleteInfos {
    pub fn new(album: Album, index: &IndexCache) -> Self {
        Self {
            artists: album
                .artists_id
                .iter()
                .map(|artist| index.artists.get(artist).unwrap())
                .map(|artist| ArtistCompleteInfos::new(artist.clone(), index))
                .collect(),

            genres: index
                .albums_genres
                .get(&album.id)
                .unwrap()
                .iter()
                .map(|genre| index.genres.get(genre).unwrap())
                .map(|genre| GenreCompleteInfos::new(genre.clone(), index))
                .collect(),

            tracks_count: index.albums_tracks.get(&album.id).unwrap().len(),

            album,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackCompleteInfos {
    track: Track,
    artists: Vec<ArtistCompleteInfos>,
    genres: Vec<GenreCompleteInfos>,
    album: AlbumCompleteInfos,
    rating: Option<Rating>,
}

impl TrackCompleteInfos {
    pub fn new(track: Track, index: &IndexCache, ratings: &Ratings) -> Self {
        let album = index.albums.get(&track.tags.album_id).unwrap().clone();

        Self {
            artists: track
                .tags
                .artists_id
                .iter()
                .map(|artist| index.artists.get(artist).unwrap())
                .map(|artist| ArtistCompleteInfos::new(artist.clone(), index))
                .collect(),

            genres: track
                .tags
                .genres_id
                .iter()
                .map(|genre| index.genres.get(genre).unwrap())
                .map(|genre| GenreCompleteInfos::new(genre.clone(), index))
                .collect(),

            album: AlbumCompleteInfos::new(album, index),

            rating: ratings.get(&track.id).copied(),

            track,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenreCompleteInfos {
    genre: Genre,
    albums_count: usize,
    tracks_count: usize,
}

impl GenreCompleteInfos {
    pub fn new(genre: Genre, index: &IndexCache) -> Self {
        Self {
            albums_count: index.genres_albums.get(&genre.id).unwrap().len(),
            tracks_count: index.genres_tracks.get(&genre.id).unwrap().len(),
            genre,
        }
    }
}
