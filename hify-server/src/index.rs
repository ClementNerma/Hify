use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
    path::PathBuf,
};

use juniper::{GraphQLEnum, GraphQLObject, GraphQLScalarValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub from: PathBuf,
    pub fingerprint: String,
    pub tracks: Vec<Track>,
    pub observations: Vec<String>,
    pub cache: IndexCache,
}

#[derive(Serialize, Deserialize)]
pub struct IndexCache {
    pub tracks_paths: HashMap<TrackID, PathBuf>,

    pub no_title_tracks: HashSet<TrackID>,
    pub no_album_tracks: HashSet<TrackID>,
    pub no_album_artist_tracks: HashSet<TrackID>,

    pub artists_albums: HashMap<String, HashSet<AlbumID>>,
    pub artists_tracks: HashMap<String, HashSet<TrackID>>,

    pub album_artists_albums: HashMap<String, HashSet<AlbumID>>,

    pub album_tracks: HashMap<AlbumID, HashSet<TrackID>>,
}

#[derive(
    GraphQLScalarValue, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[graphql(transparent)]
pub struct TrackID(pub String);

#[derive(
    GraphQLScalarValue, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[graphql(transparent)]
pub struct AlbumID(pub String);

#[derive(GraphQLObject, Serialize, Deserialize, Clone)]
pub struct Track {
    pub id: TrackID,
    pub path: String,
    pub metadata: TrackMetadata,
}

#[derive(GraphQLObject, Serialize, Deserialize, Clone)]
pub struct TrackMetadata {
    pub format: AudioFormat,
    pub size: i32,
    pub duration: f64,
    pub bitrate: i32,
    pub tags: TrackTags,
}

#[derive(GraphQLObject, Serialize, Deserialize, Clone)]
pub struct TrackTags {
    pub title: Option<String>,

    pub artist: Option<String>,
    pub composer: Option<String>,

    pub album: Option<String>,
    pub album_artist: Option<String>,

    pub disc: Option<i32>,
    pub track_no: Option<i32>,

    pub date: Option<TrackDate>,
    pub genre: Option<String>,
    // pub note: Option<u8>,
}

impl TrackTags {
    pub fn get_album_id(&self) -> Option<AlbumID> {
        let mut hasher = DefaultHasher::new();
        self.album.as_ref()?.hash(&mut hasher);
        self.album_artist.hash(&mut hasher);
        // self.date.as_ref().map(|date| date.year).hash(&mut hasher);
        Some(AlbumID(hasher.finish().to_string()))
    }
}

#[derive(GraphQLEnum, Serialize, Deserialize, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum AudioFormat {
    MP3,
    FLAC,
}

#[derive(GraphQLObject, Serialize, Deserialize, Clone)]
pub struct TrackDate {
    pub year: i32,
    pub month: Option<i32>,
    pub day: Option<i32>,
}
