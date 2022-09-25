use std::{path::Path, process::Command};

use anyhow::{bail, Context, Result};
use once_cell::sync::Lazy;
use pomsky_macro::pomsky;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;

use crate::index::{AudioFormat, TrackDate, TrackMetadata, TrackTags};

pub fn run_on(file: &Path) -> Result<Option<TrackMetadata>> {
    let audio_ext = match file.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => ext.to_ascii_lowercase(),
        None => return Ok(None),
    };

    if matches!(
        audio_ext.as_str(),
        "mpeg" | "mp4" | "alac" | "webm" | "aiff" | "dsf"
    ) {
        bail!("File format unsupported by web players: {audio_ext}");
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
                .context("File doesn't have a valid UTF-8 filename")?,
        ])
        .output()
        .context("Failed to launch ExifTool: {e}")?;

    if !exiftool_out.status.success() {
        let stderr = std::str::from_utf8(&exiftool_out.stderr).unwrap_or("<invalid UTF-8 output>");

        bail!("ExifTool failed: {stderr}");
    }

    let json_str = std::str::from_utf8(&exiftool_out.stdout)
        .context("ExifTool returned an invalid UTF-8 response")?;

    let parsed_output = serde_json::from_str::<ExifToolOutput>(json_str)
        .context("Failed to parse ExifTool output")?;

    let mut files = parsed_output.0;

    let file = match files.len() {
        0 => bail!("File does nto contain any audio stream"),
        1 => files.remove(0),
        _ => bail!("File contains multiple audio streams"),
    };

    let format = match file.FileType.as_str() {
        "FLAC" => AudioFormat::FLAC,
        "MP3" => AudioFormat::MP3,
        "WAV" => AudioFormat::WAV,
        "AAC" => AudioFormat::AAC,
        "OGG" => AudioFormat::OGG,
        "M4A" => AudioFormat::M4A,
        codec_name => bail!("Unknown codec name: {codec_name}"),
    };

    Ok(Some(TrackMetadata {
        format,
        size: i32::try_from(file.FileSize).with_context(|| {
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

fn parse_exiftool_tags(tags: ExifToolFileTags) -> Result<TrackTags> {
    Ok(TrackTags {
        title: tags.Title.context("Missing 'title' tag")?,
        artists: tags.Artist.map(parse_array_tag).unwrap_or_default(),
        composers: tags.Composer.map(parse_array_tag).unwrap_or_default(),
        album: tags.Album.context("Missing 'album' tag")?,
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

fn int_or_string(value: Value) -> Result<String> {
    match value {
        Value::Number(num) => Ok(num.to_string()),
        Value::String(str) => Ok(str),
        _ => bail!("Failed to parse value: {:?}", value),
    }
}

fn parse_set_number(input: &str, category: &'static str) -> Result</*u16*/ i32> {
    PARSE_DISC_NUMBER
        .captures(input)
        .with_context(|| format!("Invalid {category} value: {input}"))
        .and_then(|c| {
            c.name("number")
                .unwrap()
                .as_str()
                .parse::<u16>()
                .map(i32::from)
                .with_context(|| {
                    format!("Internal error: failed to parse validated {category} number: {input}")
                })
        })
}

fn parse_date(input: &str) -> Result<TrackDate> {
    let captured = PARSE_TRACK_YEAR_OR_DATE_1
        .captures(input)
        .or_else(|| PARSE_TRACK_YEAR_OR_DATE_2.captures(input))
        .or_else(|| PARSE_TRACK_YEAR_OR_DATE_3.captures(input))
        .with_context(|| format!("Invalid date value: {input}"))?;

    Ok(TrackDate {
        year: captured
            .name("year")
            .unwrap()
            .as_str()
            .parse::<u16>()
            .map(i32::from)
            .context("Invalid year number")?,

        month: captured
            .name("month")
            .map(|month| month.as_str().parse::<u8>().context("Invalid month number"))
            .transpose()?
            .map(i32::from),

        day: captured
            .name("day")
            .map(|day| day.as_str().parse::<u8>().context("Invalid day number"))
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

static PARSE_DISC_NUMBER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(pomsky!(
        Start
            :number([digit]+)
            ("/" [digit]+)?
        End
    ))
    .unwrap()
});

static PARSE_TRACK_YEAR_OR_DATE_1: Lazy<Regex> = Lazy::new(|| {
    Regex::new(pomsky!(
        Start
            :year([digit]{4})
            ['-' '\\' '.' ' ']
            :month([digit]{2})
            ['-' '\\' '.' ' ']
            :day([digit]{2})
            ('T' [digit]{2} ':' [digit]{2} ':' [digit]{2} 'Z')?
        End

    ))
    .unwrap()
});

static PARSE_TRACK_YEAR_OR_DATE_2: Lazy<Regex> = Lazy::new(|| {
    Regex::new(pomsky!(
        Start
            :month([digit]{2})
            ['-' '\\' '.' ' ']
            :day([digit]{2})
            ['-' '\\' '.' ' ']
            :year([digit]{4})
            ('T' [digit]{2} ':' [digit]{2} ':' [digit]{2} 'Z')?
        End
    ))
    .unwrap()
});

static PARSE_TRACK_YEAR_OR_DATE_3: Lazy<Regex> = Lazy::new(|| {
    Regex::new(pomsky!(
        Start
            :year([digit]{4})
            (';' | End)
    ))
    .unwrap()
});
