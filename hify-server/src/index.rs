use std::{
    collections::{hash_map::DefaultHasher, BTreeMap, BTreeSet},
    hash::{Hash, Hasher},
    path::PathBuf,
};

use juniper::{GraphQLEnum, GraphQLObject};
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
    pub tracks_paths: BTreeMap<TrackID, PathBuf>,
    pub tracks_formats: BTreeMap<TrackID, AudioFormat>,
    pub tracks_index: BTreeMap<TrackID, usize>,

    pub no_title_tracks: BTreeSet<TrackID>,
    pub no_album_tracks: BTreeSet<TrackID>,
    pub no_album_artist_tracks: BTreeSet<TrackID>,

    pub artists_albums: BTreeMap<String, BTreeSet<AlbumID>>,
    pub artists_tracks: BTreeMap<String, BTreeSet<TrackID>>,

    pub albums_artists_albums: BTreeMap<String, BTreeSet<AlbumID>>,

    pub albums_tracks: BTreeMap<AlbumID, BTreeSet<TrackID>>,

    pub albums_infos: BTreeMap<AlbumID, AlbumInfos>,
}

#[derive(GraphQLObject, Serialize, Deserialize, Hash)]
pub struct AlbumInfos {
    pub name: String,
    pub album_artist: Option<String>,
}

impl AlbumInfos {
    pub fn get_id(&self) -> AlbumID {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        AlbumID(format!("{:x}", hasher.finish()))
    }
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TrackID(pub String);

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AlbumID(pub String);

#[derive(Serialize, Deserialize, Clone)]
pub struct Track {
    pub id: TrackID,
    pub path: String,
    pub metadata: TrackMetadata,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TrackMetadata {
    pub format: AudioFormat,
    pub size: i32,
    pub duration: f64,
    pub bitrate: i32,
    pub tags: TrackTags,
}

#[derive(Serialize, Deserialize, Clone)]
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
    pub fn get_album_infos(&self) -> Option<AlbumInfos> {
        Some(AlbumInfos {
            name: self.album.as_ref()?.clone(),
            album_artist: self.album_artist.clone(),
        })
    }
}

#[derive(GraphQLEnum, Serialize, Deserialize, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum AudioFormat {
    MP3,
    FLAC,
}

#[derive(GraphQLObject, Serialize, Deserialize, Clone, Copy)]
pub struct TrackDate {
    pub year: i32,
    pub month: Option<i32>,
    pub day: Option<i32>,
}
