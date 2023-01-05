use anyhow::{bail, Result};
use serde::Deserialize;

use crate::index::{AudioFormat, TrackMetadata};

use super::tags::{parse_exiftool_tags, ExifToolFileTags};

pub fn process_analyzed_file(analyzed: ExifToolFile) -> Result<TrackMetadata> {
    let format = match analyzed.FileType.as_str() {
        "FLAC" => AudioFormat::FLAC,
        "MP3" => AudioFormat::MP3,
        "WAV" => AudioFormat::WAV,
        "AAC" => AudioFormat::AAC,
        "OGG" => AudioFormat::OGG,
        "M4A" => AudioFormat::M4A,
        "OPUS" => AudioFormat::OPUS,
        codec_name => bail!("Unknown codec name: {codec_name}"),
    };

    Ok(TrackMetadata {
        format,
        size: analyzed.FileSize,
        duration: analyzed.Duration.round() as u32,
        tags: parse_exiftool_tags(analyzed.tags)?,
    })
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct ExifToolFile {
    FileType: String,
    Duration: f64,
    FileSize: u64,

    #[serde(flatten)]
    tags: ExifToolFileTags,
}
