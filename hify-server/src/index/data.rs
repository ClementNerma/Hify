use std::{
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
    path::PathBuf,
};

use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};

use super::sorted_map::SortedMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct Index {
    pub from: PathBuf,
    pub fingerprint: String,
    pub tracks: SortedMap<TrackID, Track>,
    pub albums_arts: HashMap<AlbumID, Option<PathBuf>>,
    pub cache: IndexCache,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct IndexCache {
    pub tracks_paths: HashMap<TrackID, PathBuf>,

    pub artists_albums: HashMap<ArtistID, SortedMap<AlbumID, AlbumInfos>>,
    pub artists_tracks: HashMap<ArtistID, Vec<TrackID>>,

    pub albums_tracks: HashMap<AlbumID, Vec<TrackID>>,
    pub albums_artists_albums: HashMap<ArtistID, SortedMap<AlbumID, AlbumInfos>>,

    pub albums_mean_score: HashMap<AlbumID, f64>,
    pub artists_mean_score: HashMap<ArtistID, f64>,
    pub albums_artists_mean_score: HashMap<ArtistID, f64>,

    pub artists_infos: SortedMap<ArtistID, ArtistInfos>,
    pub albums_artists_infos: SortedMap<ArtistID, ArtistInfos>,
    pub albums_infos: SortedMap<AlbumID, AlbumInfos>,
    pub genre_infos: SortedMap<GenreID, GenreInfos>,

    pub genres_albums: HashMap<GenreID, SortedMap<AlbumID, AlbumInfos>>,
    pub genres_tracks: HashMap<GenreID, Vec<TrackID>>,
    pub no_genre_tracks: HashSet<TrackID>,
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AlbumInfos {
    pub name: String,
    pub album_artists: Vec<ArtistInfos>,
}

impl AlbumInfos {
    fn new(name: String, album_artists: Vec<ArtistInfos>) -> Self {
        Self {
            name,
            album_artists,
        }
    }

    pub fn get_id(&self) -> AlbumID {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        AlbumID(format!("{:x}", hasher.finish()))
    }
}

#[derive(Serialize, Deserialize, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArtistInfos {
    pub name: String,
}

impl ArtistInfos {
    fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get_id(&self) -> ArtistID {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        ArtistID(format!("{:x}", hasher.finish()))
    }
}

#[derive(Serialize, Deserialize, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenreInfos {
    pub name: String,
}

impl GenreInfos {
    fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get_id(&self) -> GenreID {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        GenreID(format!("{:x}", hasher.finish()))
    }
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct TrackID(pub String);

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct AlbumID(pub String);

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct ArtistID(pub String);

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct GenreID(pub String);

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
            .then_with(|| a_tags.disc.cmp(&b_tags.disc))
            .then_with(|| a_tags.track_no.cmp(&b_tags.track_no))
            .then_with(|| self.path.cmp(&other.path))
    }
}

#[derive(Serialize, Deserialize, Clone, SimpleObject, PartialEq, Eq)]
pub struct TrackMetadata {
    pub format: AudioFormat,
    pub size: u64,
    pub duration: u32,
    pub tags: TrackTags,
}

#[derive(Serialize, Deserialize, Clone, SimpleObject, PartialEq, Eq)]
#[graphql(complex)]
pub struct TrackTags {
    pub title: String,

    #[graphql(skip)]
    pub artists: Vec<String>,

    pub composers: Vec<String>,

    #[graphql(skip)]
    pub album: String,

    #[graphql(skip)]
    pub album_artists: Vec<String>,

    pub disc: Option<u32>,
    pub track_no: Option<u32>,

    pub date: Option<TrackDate>,

    #[graphql(skip)]
    pub genres: Vec<String>,

    pub rating: Option<u8>,
}

impl TrackTags {
    pub fn get_album_infos(&self) -> AlbumInfos {
        AlbumInfos::new(self.album.clone(), self.get_album_artists_infos().collect())
    }

    pub fn get_artists_infos(&self) -> impl Iterator<Item = ArtistInfos> + '_ {
        self.artists.iter().cloned().map(ArtistInfos::new)
    }

    pub fn get_album_artists_infos(&self) -> impl Iterator<Item = ArtistInfos> + '_ {
        let artists = if !self.album_artists.is_empty() {
            &self.album_artists
        } else {
            &self.artists
        };
        artists.iter().cloned().map(ArtistInfos::new)
    }

    pub fn get_genres_infos(&self) -> impl Iterator<Item = GenreInfos> + '_ {
        self.genres.iter().cloned().map(GenreInfos::new)
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
    pub year: u32,
    pub month: Option<u8>,
    pub day: Option<u8>,
}
