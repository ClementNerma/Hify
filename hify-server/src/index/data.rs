use std::{
    collections::{hash_map::DefaultHasher, BTreeMap, BTreeSet},
    hash::{Hash, Hasher},
    path::PathBuf,
};

use async_graphql::{Enum, SimpleObject};
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

    pub artists_albums: BTreeMap<ArtistID, BTreeSet<AlbumID>>,
    pub artists_tracks: BTreeMap<ArtistID, BTreeSet<TrackID>>,

    pub albums_artists_albums: BTreeMap<ArtistID, BTreeSet<AlbumID>>,

    pub albums_tracks: BTreeMap<AlbumID, BTreeSet<TrackID>>,

    pub artists_infos: BTreeMap<ArtistID, ArtistInfos>,
    pub albums_infos: BTreeMap<AlbumID, AlbumInfos>,

    pub ordered_artists: Vec<ArtistID>,
    pub ordered_albums_artists: Vec<ArtistID>,
    pub ordered_albums: Vec<AlbumID>,
}

#[derive(Serialize, Deserialize, Hash)]
pub struct AlbumInfos {
    pub name: String,
    pub album_artists: Vec<String>,
}

impl AlbumInfos {
    pub fn get_id(&self) -> AlbumID {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        AlbumID(format!("{:x}", hasher.finish()))
    }
}

#[derive(Serialize, Deserialize, Hash, Clone)]
pub struct ArtistInfos {
    pub name: String,
}

impl ArtistInfos {
    pub fn get_id(&self) -> ArtistID {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        ArtistID(format!("{:x}", hasher.finish()))
    }
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TrackID(pub String);

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AlbumID(pub String);

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArtistID(pub String);

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Track {
    #[graphql(skip)]
    pub id: TrackID,
    pub path: String,
    pub metadata: TrackMetadata,
}

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct TrackMetadata {
    pub format: AudioFormat,
    pub size: i32,
    pub duration: f64,
    pub bitrate: i32,
    pub tags: TrackTags,
}

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct TrackTags {
    pub title: Option<String>,

    pub artists: Vec<String>,
    pub composers: Vec<String>,

    pub album: Option<String>,
    pub album_artists: Vec<String>,

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
            album_artists: self.album_artists.clone(),
        })
    }

    pub fn get_artists_infos(&self) -> impl Iterator<Item = ArtistInfos> + '_ {
        self.artists
            .iter()
            .map(|name| ArtistInfos { name: name.clone() })
    }

    pub fn get_album_artists_infos(&self) -> impl Iterator<Item = ArtistInfos> + '_ {
        self.album_artists
            .iter()
            .map(|name| ArtistInfos { name: name.clone() })
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Enum)]
#[allow(clippy::upper_case_acronyms)]
pub enum AudioFormat {
    MP3,
    FLAC,
}

#[derive(Serialize, Deserialize, Clone, Copy, SimpleObject)]
pub struct TrackDate {
    pub year: i32,
    pub month: Option<i32>,
    pub day: Option<i32>,
}
