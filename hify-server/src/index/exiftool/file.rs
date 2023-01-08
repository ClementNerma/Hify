use anyhow::{bail, Context, Result};
use once_cell::sync::Lazy;
use pomsky_macro::pomsky;
use regex::Regex;
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
        duration: parse_exiftool_duration(&analyzed.Duration)?,
        tags: parse_exiftool_tags(analyzed.tags)?,
    })
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct ExifToolFile {
    FileType: String,
    Duration: ExifToolDuration,
    FileSize: u64,

    #[serde(flatten)]
    tags: ExifToolFileTags,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ExifToolDuration {
    Seconds(f64),
    Raw(String),
}

fn parse_exiftool_duration(duration: &ExifToolDuration) -> Result<u32> {
    match duration {
        ExifToolDuration::Seconds(secs) => Ok(secs.ceil() as u32),
        ExifToolDuration::Raw(raw) => {
            let captured = PARSE_EXIF_TOOL_DURATION
                .captures(raw)
                .with_context(|| format!("Unknown duration provided by ExifTool: {raw}"))?;

            let hours = captured
                .name("hours")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();

            let minutes = captured
                .name("minutes")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();

            let seconds = captured
                .name("seconds")
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();

            let rest = captured
                .name("rest")
                .unwrap()
                .as_str()
                .parse::<u128>()
                .unwrap();

            Ok(hours * 3600 + minutes * 60 + seconds + u32::from(rest > 0))
        }
    }
}

static PARSE_EXIF_TOOL_DURATION: Lazy<Regex> = Lazy::new(|| {
    Regex::new(pomsky!(
        Start :hours([digit]{2,}) ':' :minutes([digit]{2}) ':' :seconds([digit]{2}) '.' :rest([digit]+) End
    ))
    .unwrap()
});
