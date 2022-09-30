use std::{
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
    path::PathBuf,
};

use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::define_scalar_string;

use super::sorted_map::SortedMap;

/// Global index, contains all data on the music files contained in a provided directory
#[derive(Clone, Serialize, Deserialize)]
pub struct Index {
    pub from: PathBuf,
    pub fingerprint: String,
    pub tracks: SortedMap<TrackID, Track>,
    pub albums_arts: HashMap<AlbumID, Option<PathBuf>>,
    pub cache: IndexCache,
}

/// Index cache, used to accelerate requests by pre-computing some results once after index generation.
#[derive(Clone, Serialize, Deserialize)]
pub struct IndexCache {
    /// Absolute filesystem path to each track
    pub tracks_paths: HashMap<TrackID, PathBuf>,

    /// Albums where the artist is listed in the "album artists" tag
    pub artists_albums: HashMap<ArtistID, SortedMap<AlbumID, AlbumInfos>>,

    /// Albums where the artist is listed in one of the tracks but not in the "album artists" tag
    pub artists_album_participations: HashMap<ArtistID, SortedMap<AlbumID, AlbumInfos>>,

    /// Tracks where the artist is listed in
    pub artists_tracks: HashMap<ArtistID, Vec<TrackID>>,

    /// Trachs where the artist is listed in but belonging to an album they're not an "album artist" of
    pub artists_track_participations: HashMap<ArtistID, Vec<TrackID>>,

    /// Tracks belonging to an album
    pub albums_tracks: HashMap<AlbumID, Vec<TrackID>>,

    /// Mean score of a score
    pub albums_mean_score: HashMap<AlbumID, f64>,

    /// Mean score of an artist
    pub artists_mean_score: HashMap<ArtistID, f64>,

    /// Mean score of an album artist (= artist who has at least 1 album)
    pub albums_artists_mean_score: HashMap<ArtistID, f64>,

    /// Informations about artists
    pub artists_infos: SortedMap<ArtistID, ArtistInfos>,

    /// Informations about album arists
    pub albums_artists_infos: SortedMap<ArtistID, ArtistInfos>,

    /// Informations about albums
    pub albums_infos: SortedMap<AlbumID, AlbumInfos>,

    /// Informations about genres
    pub genre_infos: SortedMap<GenreID, GenreInfos>,

    /// List of album for each genre
    pub genres_albums: HashMap<GenreID, SortedMap<AlbumID, AlbumInfos>>,

    /// List of tracks for each genre
    pub genres_tracks: HashMap<GenreID, Vec<TrackID>>,

    /// List of tracks who don't have a genre
    pub no_genre_tracks: HashSet<TrackID>,
}

/// Album infos, identifying an album
/// Mainly aimed to allow fetching album-related data (e.g. tracks) from the GraphQL API
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

/// Artist infos, identifying an artist
/// Mainly aimed to allow fetching artist-related data (e.g. albums) from the GraphQL API
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

/// Genre infos, identifying a genre
/// Mainly aimed to allow fetching genre-related data (e.g. albums) from the GraphQL API
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

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct TrackID(pub String);

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct AlbumID(pub String);

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct ArtistID(pub String);

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct GenreID(pub String);

define_scalar_string!(TrackID, AlbumID, ArtistID, GenreID);

/// Full track informations
/// Does not have a layer like ArtistInfos or AlbumInfos as most of the data will be fetched in GraphQL anyway
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

/// List of audio-related metadata
#[derive(Serialize, Deserialize, Clone, SimpleObject, PartialEq, Eq)]
pub struct TrackMetadata {
    /// Audio file format
    pub format: AudioFormat,

    /// File size, in bytes
    pub size: u64,

    /// Duration, in seconds
    pub duration: u32,

    /// Audio tags
    pub tags: TrackTags,
}

/// List of audio tags
#[derive(Serialize, Deserialize, Clone, SimpleObject, PartialEq, Eq)]
#[graphql(complex)]
pub struct TrackTags {
    /// The track's title
    pub title: String,

    /// The track's artists list
    /// Not shown in GraphQL as another method is present to fetch a list of ArtistInfos instead
    #[graphql(skip)]
    pub artists: Vec<String>,

    /// The track's composers
    pub composers: Vec<String>,

    /// The track's album
    /// Not shown in GraphQL as another method is present to fetch an AlbumInfos instead
    #[graphql(skip)]
    pub album: String,

    /// The track's album artists list
    /// Not shown in GraphQL as another method is present to fetch a list of ArtistInfos instead
    #[graphql(skip)]
    pub album_artists: Vec<String>,

    /// The disc number the track is present on
    pub disc: Option<u32>,

    /// The track's number in its own disc
    pub track_no: Option<u32>,

    /// The track's release date
    pub date: Option<TrackDate>,

    /// The track's genres list
    /// Not shown in GraphQL as another method is present to fetch a list of GenreInfos instead
    #[graphql(skip)]
    pub genres: Vec<String>,

    /// The track's rating, from 0 to 100
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

/// List of supported audio formats
/// Other formats may be supported by the extraction tool, but not listed here as not supported by web browsers
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

/// The release date of a track
#[derive(Serialize, Deserialize, Clone, Copy, SimpleObject, PartialEq, Eq)]
pub struct TrackDate {
    /// The full year
    pub year: u32,

    /// Day, starting from 1
    pub month: Option<u8>,

    /// Day, starting from 1
    pub day: Option<u8>,
}
