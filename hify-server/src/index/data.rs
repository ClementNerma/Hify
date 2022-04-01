use std::{
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
    path::PathBuf,
};

use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};

use super::sorted_map::SortedMap;

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub from: PathBuf,
    pub fingerprint: String,
    pub tracks: SortedMap<TrackID, Track>,
    pub observations: Vec<String>,
    pub cache: IndexCache,
}

#[derive(Serialize, Deserialize)]
pub struct IndexCache {
    pub tracks_paths: HashMap<TrackID, PathBuf>,

    pub no_title_tracks: HashSet<TrackID>,
    pub no_album_tracks: HashSet<TrackID>,
    pub no_album_artist_tracks: HashSet<TrackID>,

    pub artists_albums: HashMap<ArtistID, SortedMap<AlbumID, AlbumInfos>>,
    pub artists_tracks: HashMap<ArtistID, Vec<TrackID>>,

    pub albums_artists_albums: HashMap<ArtistID, SortedMap<AlbumID, AlbumInfos>>,

    pub albums_tracks: HashMap<AlbumID, Vec<TrackID>>,

    pub artists_infos: SortedMap<ArtistID, ArtistInfos>,
    pub albums_artists_infos: SortedMap<ArtistID, ArtistInfos>,
    pub albums_infos: SortedMap<AlbumID, AlbumInfos>,
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct AlbumInfos {
    pub name: String,
    pub album_artists: Vec<ArtistInfos>,
}

impl AlbumInfos {
    pub fn get_id(&self) -> AlbumID {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        AlbumID(format!("{:x}", hasher.finish()))
    }
}

impl PartialOrd for AlbumInfos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AlbumInfos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name
            .cmp(&other.name)
            .then_with(|| self.album_artists.cmp(&other.album_artists))
    }
}

#[derive(Serialize, Deserialize, Hash, Clone, PartialEq, Eq)]
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

impl PartialOrd for ArtistInfos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ArtistInfos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct TrackID(pub String);

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct AlbumID(pub String);

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct ArtistID(pub String);

#[derive(Serialize, Deserialize, Clone, SimpleObject, PartialEq, Eq)]
#[graphql(complex)]
pub struct Track {
    #[graphql(skip)]
    pub id: TrackID,
    pub path: String,
    pub metadata: TrackMetadata,
}

impl PartialOrd for Track {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Track {
    fn cmp(&self, other: &Self) -> Ordering {
        let a_tags = &self.metadata.tags;
        let b_tags = &other.metadata.tags;

        a_tags
            .get_album_infos()
            .cmp(&b_tags.get_album_infos())
            .then_with(|| a_tags.track_no.cmp(&b_tags.track_no))
            .then_with(|| self.path.cmp(&other.path))
    }
}

#[derive(Serialize, Deserialize, Clone, SimpleObject, PartialEq, Eq)]
pub struct TrackMetadata {
    pub format: AudioFormat,
    pub size: i32,
    pub duration: i32,
    pub bitrate: i32,
    pub tags: TrackTags,
}

#[derive(Serialize, Deserialize, Clone, SimpleObject, PartialEq, Eq)]
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
            album_artists: self
                .album_artists
                .iter()
                .map(|name| ArtistInfos { name: name.clone() })
                .collect(),
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
    WAV,
    AAC,
    OGG,
    M4A,
}

#[derive(Serialize, Deserialize, Clone, Copy, SimpleObject, PartialEq, Eq)]
pub struct TrackDate {
    pub year: i32,
    pub month: Option<i32>,
    pub day: Option<i32>,
}
