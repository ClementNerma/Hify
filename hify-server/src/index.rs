use juniper::{GraphQLEnum, GraphQLObject};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, GraphQLObject, Clone)]
pub struct Index {
    pub creation_time: String,
    pub tracks: Vec<Track>,
    pub observations: Vec<String>,
}

#[derive(Serialize, Deserialize, GraphQLObject, Clone)]
pub struct Track {
    pub id: String,
    pub path: String,
    pub metadata: TrackMetadata,
}

#[derive(Serialize, Deserialize, GraphQLObject, Clone)]
pub struct TrackMetadata {
    pub format: AudioFormat,
    pub size: i32,
    pub duration: f64,
    pub bitrate: i32,
    pub tags: TrackTags,
}

#[derive(Serialize, Deserialize, GraphQLObject, Clone)]
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

#[derive(Serialize, Deserialize, GraphQLEnum, Clone, Copy)]
pub enum AudioFormat {
    MP3,
    FLAC,
}

#[derive(Serialize, Deserialize, GraphQLObject, Clone)]
pub struct TrackDate {
    pub year: i32,
    pub month: Option<i32>,
    pub day: Option<i32>,
}
