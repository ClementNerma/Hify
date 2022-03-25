use std::collections::HashMap;

use miniserde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Library {
    pub creation_time: u64,
    pub tracks: Vec<Track>,
    pub invalid_files: Vec<String>,
    pub tracks_files: HashMap<u64, String>,
}

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub id: u64,

    pub format: AudioFormat,

    pub title: Option<String>,

    pub artist: Option<String>,

    pub album: Option<String>,
    pub album_artist: Option<String>,

    pub disc: Option<u32>,
    pub track_no: Option<u32>,

    pub year: Option<i32>,
    // pub month: Option<u8>,
    // pub day: Option<u8>,
    pub genre: Option<String>,

    pub duration: Option<u32>,
    // pub bitrate: u32,
    // pub frequency: u8,

    // pub note: Option<u8>,
}

#[derive(Serialize, Deserialize)]
pub enum AudioFormat {
    Wave,
    MP3,
    WebM,
    OGG,
    FLAC,
}
