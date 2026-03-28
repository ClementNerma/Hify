use std::{
    hash::Hash,
    path::{Path, PathBuf},
    time::SystemTime,
};

use anyhow::Result;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

use crate::{
    stable_hash,
    utils::{iter_stable_hash, u64_base62_serialization},
};

// TODO: custom debug impl for artistID etc. with base62 encoding

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Index {
    pub tracks: Vec<Track>,
    pub albums: Vec<Album>,
    pub artists: Vec<Artist>,
    pub genres: Vec<Genre>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrackID(#[serde(with = "u64_base62_serialization")] u64);

impl TrackID {
    pub fn compute(relative_path: &Path) -> Self {
        Self(stable_hash!(relative_path))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: TrackID,
    pub relative_path: PathBuf,
    pub file_size_bytes: u64,
    pub file_times: FileTimes,
    pub metadata: TrackMetadata,
    pub tags: TrackTags,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct TrackMetadata {
    pub duration_s: u32,
    pub audio_codec: TrackAudioCodec,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
#[allow(clippy::upper_case_acronyms)]
pub enum TrackAudioCodec {
    FLAC,
    OPUS,
    VORBIS,
    MP3,
    AAC,
}

/// List of audio tags
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TrackTags {
    /// The track's title
    pub title: String,

    /// The track's artists list
    pub artists_id: IndexSet<ArtistID>,

    /// The track's composers
    pub composers_id: IndexSet<ArtistID>,

    /// The track's album
    pub album_id: AlbumID,

    /// The disc number the track is present on
    pub disc_number: Option<u32>,

    /// The track's number in its own disc
    pub track_number: Option<u32>,

    /// The track's genres list
    pub genres_id: IndexSet<GenreID>,

    /// The track's release date
    pub date: Option<TrackDate>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TrackDate {
    pub year: u16,
    pub month: Option<u8>,
    pub day: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AlbumID(#[serde(with = "u64_base62_serialization")] u64);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: AlbumID,
    pub name: String,
    pub artists_id: IndexSet<ArtistID>,
}

impl Album {
    pub fn new(name: String, artists: IndexSet<ArtistID>) -> Self {
        Self {
            id: AlbumID(stable_hash!(name, iter_stable_hash(artists.iter()))),
            name,
            artists_id: artists,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArtistID(#[serde(with = "u64_base62_serialization")] u64);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Artist {
    pub id: ArtistID,
    pub name: String,
}

impl Artist {
    pub fn new(name: String) -> Self {
        Self {
            id: ArtistID(stable_hash!(name)),
            name,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenreID(#[serde(with = "u64_base62_serialization")] u64);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Genre {
    pub id: GenreID,
    pub name: String,
}

impl Genre {
    pub fn new(name: String) -> Self {
        Self {
            id: GenreID(stable_hash!(name)),
            name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rating {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Rating {
    pub fn get_zero_to_five(self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
        }
    }
}

impl TryFrom<u8> for Rating {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct FileTimes {
    pub ctime: Option<SystemTime>,
    pub mtime: SystemTime,
}

pub trait IdType: std::fmt::Debug + Copy + Eq + Hash + Send + Sync + 'static {
    fn encode(&self) -> String;
    fn decode(s: &str) -> Result<Self>;
}

macro_rules! impl_id_type {
    ($($id_type:ty),+) => {
        $(
            impl std::fmt::Debug for $id_type {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}(\"{}\")", stringify!($id_type), self.encode())
                }
            }

            impl IdType for $id_type {
                fn encode(&self) -> String {
                    format!("{}", $crate::utils::encode_base62_u64(self.0))
                }

                fn decode(s: &str) -> ::anyhow::Result<Self> {
                    $crate::utils::decode_base62_u64(s).map_err(|err| ::anyhow::anyhow!(err)).map(Self)
                }
            }
        )+
    };
}

impl_id_type!(TrackID, AlbumID, ArtistID, GenreID);
