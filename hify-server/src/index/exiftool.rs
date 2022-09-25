use std::{path::Path, process::Command};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;

use crate::index::{AudioFormat, TrackDate, TrackMetadata, TrackTags};

pub fn run_on(file: &Path) -> Result<Option<TrackMetadata>, String> {
    let audio_ext = match file.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => ext.to_ascii_lowercase(),
        None => return Ok(None),
    };

    if matches!(
        audio_ext.as_str(),
        "mpeg" | "mp4" | "alac" | "webm" | "aiff" | "dsf"
    ) {
        return Err(format!(
            "File format unsupported by web players: {audio_ext}"
        ));
    }

    if !matches!(
        audio_ext.as_str(),
        "mp3" | "flac" | "wav" | "aac" | "ogg" | "m4a"
    ) {
        return Ok(None);
    }

    let exiftool_out = Command::new("exiftool")
        .args(&[
            "-n",
            "-json",
            file.to_str()
                .ok_or("File doesn't have a valid UTF-8 filename")?,
        ])
        .output()
        .map_err(|e| format!("Failed to launch ExifTool: {e}"))?;

    if !exiftool_out.status.success() {
        let stderr = std::str::from_utf8(&exiftool_out.stderr).unwrap_or("<invalid UTF-8 output>");

        return Err(format!("ExifTool failed: {stderr}"));
    }

    let json_str = std::str::from_utf8(&exiftool_out.stdout)
        .map_err(|e| format!("ExifTool returned an invalid UTF-8 response: {e}"))?;

    let parsed_output = serde_json::from_str::<ExifToolOutput>(json_str)
        .map_err(|e| format!("Failed to parse ExifTool output: {e}"))?;

    let mut files = parsed_output.0;

    let file = match files.len() {
        0 => return Err("File does nto contain any audio stream".into()),
        1 => files.remove(0),
        _ => return Err("File contains multiple audio streams".into()),
    };

    let format = match file.FileType.as_str() {
        "FLAC" => AudioFormat::FLAC,
        "MP3" => AudioFormat::MP3,
        "WAV" => AudioFormat::WAV,
        "AAC" => AudioFormat::AAC,
        "OGG" => AudioFormat::OGG,
        "M4A" => AudioFormat::M4A,
        codec_name => return Err(format!("Unknown codec name: {codec_name}")),
    };

    Ok(Some(TrackMetadata {
        format,
        size: i32::try_from(file.FileSize).map_err(|_| {
            format!(
                "Size is too big to be returned to GraphQL: {}",
                file.FileSize
            )
        })?,
        duration: file.Duration as i32,
        bitrate: file.AudioBitrate as i32,
        tags: parse_exiftool_tags(file.tags)?,
    }))
}

fn parse_exiftool_tags(tags: ExifToolFileTags) -> Result<TrackTags, String> {
    Ok(TrackTags {
        title: tags.Title.ok_or("Missing 'title' tag")?,
        artists: tags.Artist.map(parse_array_tag).unwrap_or_default(),
        composers: tags.Composer.map(parse_array_tag).unwrap_or_default(),
        album: tags.Album.ok_or("Missing 'album' tag")?,
        album_artists: tags.Band.map(parse_array_tag).unwrap_or_default(),

        disc: tags
            .PartOfSet
            .map(|date| int_or_string(date).and_then(|date| parse_set_number(&date, "disc number")))
            .transpose()?,

        track_no: tags
            .Track
            .map(|date| {
                int_or_string(date).and_then(|date| parse_set_number(&date, "track number"))
            })
            .transpose()?,

        date: tags
            .Year
            .map(|date| int_or_string(date).and_then(|date| parse_date(&date)))
            .transpose()?,

        genres: tags.Genre.map(parse_array_tag).unwrap_or_default(),
    })
}

fn int_or_string(value: Value) -> Result<String, String> {
    match value {
        Value::Number(num) => Ok(num.to_string()),
        Value::String(str) => Ok(str),
        _ => Err(format!("Failed to parse value: {:?}", value)),
    }
}

fn parse_set_number(input: &str, category: &'static str) -> Result</*u16*/ i32, String> {
    PARSE_DISC_NUMBER
        .captures(input)
        .ok_or_else(|| format!("Invalid {category} value: {input}"))
        .and_then(|c| {
            c.get(1)
                .unwrap()
                .as_str()
                .parse::<u16>()
                .map(i32::from)
                .map_err(|_| {
                    format!("Internal error: failed to parse validated {category} number: {input}")
                })
        })
}

fn parse_date(input: &str) -> Result<TrackDate, String> {
    let captured = PARSE_TRACK_YEAR_OR_DATE_1
        .captures(input)
        .or_else(|| PARSE_TRACK_YEAR_OR_DATE_2.captures(input))
        .or_else(|| PARSE_TRACK_YEAR_OR_DATE_3.captures(input))
        .ok_or_else(|| format!("Invalid date value: {input}"))?;

    Ok(TrackDate {
        year: captured
            .name("year")
            .unwrap()
            .as_str()
            .parse::<u16>()
            .map(i32::from)
            .map_err(|e| format!("Invalid year number: {e}"))?,
        month: captured
            .name("month")
            .map(|month| {
                month
                    .as_str()
                    .parse::<u8>()
                    .map_err(|e| format!("Invalid month number: {e}"))
            })
            .transpose()?
            .map(i32::from),
        day: captured
            .name("day")
            .map(|day| {
                day.as_str()
                    .parse::<u8>()
                    .map_err(|e| format!("Invalid day number: {e}"))
            })
            .transpose()?
            .map(i32::from),
    })
}

fn parse_array_tag(tag_content: impl AsRef<str>) -> Vec<String> {
    tag_content
        .as_ref()
        .split(&[';', ',', '/'])
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(str::to_string)
        .collect()
}

#[derive(Deserialize)]
struct ExifToolOutput(Vec<ExifToolFile>);

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct ExifToolFile {
    FileType: String,
    Duration: f32,
    FileSize: u64,
    AudioBitrate: f64,

    #[serde(flatten)]
    tags: ExifToolFileTags,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct ExifToolFileTags {
    #[serde(default)]
    Album: Option<String>,

    #[serde(default)]
    Artist: Option<String>,

    #[serde(default)]
    Band: Option<String>,

    #[serde(default)]
    Composer: Option<String>,

    #[serde(default)]
    Genre: Option<String>,

    #[serde(default)]
    PartOfSet: Option<Value>,

    #[serde(default)]
    Title: Option<String>,

    #[serde(default)]
    Track: Option<Value>,

    #[serde(default)]
    Year: Option<Value>,
    // #[serde(default)]
    // Popularimeter: Option<String>,
}

static PARSE_DISC_NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)(/\d+)?$").unwrap());

static PARSE_TRACK_YEAR_OR_DATE_1: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
    r"^(?P<year>\d\d\d\d)[-/\.\s](?P<month>\d{1,2})[-/\.\s](?P<day>\d{1,2})(?:T\d\d:\d\d:\d\dZ)?$",
).unwrap()
});

static PARSE_TRACK_YEAR_OR_DATE_2: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
    r"^(?P<month>\d{1,2})[-/\.\s](?P<day>\d{1,2})[-/\.\s](?P<year>\d\d\d\d)(?:T\d\d:\d\d:\d\dZ)?$",
).unwrap()
});

static PARSE_TRACK_YEAR_OR_DATE_3: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?P<year>\d\d\d\d)(?:;.*)?$").unwrap());
