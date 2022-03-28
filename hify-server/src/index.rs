use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Library {
    pub creation_time: u64,
    pub tracks: Vec<Track>,
    pub tracks_files: HashMap<u64, QuickFileInfo>,
    pub observations: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct QuickFileInfo {
    pub path: String,
    pub format: AudioFormat,
}

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub id: u64,
    pub metadata: TrackMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct TrackMetadata {
    pub format: AudioFormat,
    pub size: u64,
    pub duration: u32,
    pub bitrate: u32,
    pub tags: TrackTags,
}

#[derive(Serialize, Deserialize)]
pub struct TrackTags {
    pub title: Option<String>,

    pub artist: Option<String>,
    pub composer: Option<String>,

    pub album: Option<String>,
    pub album_artist: Option<String>,

    pub disc: Option<u32>,
    pub track_no: Option<u32>,

    pub date: Option<TrackDate>,
    pub genre: Option<String>,
    // pub note: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum AudioFormat {
    MP3,
    FLAC,
}

#[derive(Serialize, Deserialize)]
pub struct TrackDate {
    pub year: u16,
    pub month: Option<u8>,
    pub day: Option<u8>,
}
