use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Library {
    pub creation_time: u64,
    pub tracks: Vec<Track>,
    // pub invalid_files: Vec<String>,
    pub tracks_files: HashMap<u64, String>,
}

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub id: u64,
    pub format: AudioFormat,
    pub metadata: TrackMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct TrackMetadata {
    pub title: Option<String>,

    pub artist: Option<String>,
    pub composer: Option<String>,

    pub album: Option<String>,
    pub album_artist: Option<String>,

    pub disc: Option<u32>,
    pub track_no: Option<u32>,

    pub year: Option<u32>,
    // pub month: Option<u8>,
    // pub day: Option<u8>,
    pub genre: Option<String>,

    pub duration: Option<TrackDuration>,
    pub bitrate: Option<u32>,
    pub resolution: Option<u8>,
    // pub note: Option<u8>,
}

#[derive(Serialize, Deserialize)]
pub enum AudioFormat {
    MP3,
    FLAC,
}

#[derive(Serialize, Deserialize)]
pub struct TrackDuration {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub approx: bool,
}
