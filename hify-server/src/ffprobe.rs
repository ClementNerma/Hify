use std::{collections::BTreeMap, path::Path, process::Command};

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;

use crate::index::{AudioFormat, TrackDate, TrackMetadata, TrackTags};

pub fn run_on(file: &Path) -> Result<Option<TrackMetadata>, String> {
    let audio_ext = match file.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => ext.to_ascii_lowercase(),
        None => return Ok(None),
    };

    if matches!(
        audio_ext.as_str(),
        "m4a" | "mpeg" | "ogg" | "opus" | "alac" | "aac" | "wav" | "aiff" | "dsf" | "webm"
    ) {
        return Err(format!("Unsupported audio extension: {}", audio_ext));
    }

    if !matches!(audio_ext.as_str(), "mp3" | "flac") {
        return Ok(None);
    }

    let ffprobe_out = Command::new("ffprobe")
        .args(&[
            "-loglevel",
            "0",
            "-print_format",
            "json",
            "-show_format",
            file.to_str()
                .ok_or("File doesn't have a valid UTF-8 filename")?,
        ])
        .output()
        .map_err(|e| format!("Failed to launch FFProbe: {e}"))?;

    if !ffprobe_out.status.success() {
        let stderr = std::str::from_utf8(&ffprobe_out.stderr).unwrap_or("<invalid UTF-8 output>");

        return Err(format!("FFProbe failed: {stderr}"));
    }

    let json_str = std::str::from_utf8(&ffprobe_out.stdout)
        .map_err(|e| format!("FFProbe returned an invalid UTF-8 response: {e}"))?;

    let parsed_output = serde_json::from_str::<FFProbeOutput>(json_str)
        .map_err(|e| format!("Failed to parse FFProbe output: {e}"))?;

    let data = parsed_output.format;

    let format = match data.format_name.as_str() {
        "flac" => AudioFormat::FLAC,
        "mp3" => AudioFormat::MP3,
        name => return Err(format!("Unknown file format: {name}")),
    };

    let size = data
        .size
        .parse::<u64>()
        .map_err(|e| format!("Failed to parse file size: {e}"))?;

    Ok(Some(TrackMetadata {
        format,
        size: i32::try_from(size)
            .map_err(|_| format!("Size is too big to be returned to GraphQL: {size}"))?,
        duration: data
            .duration
            .parse::<f64>()
            .map_err(|e| format!("Failed to parse duration: {e}"))?,
        bitrate: data
            .bit_rate
            .parse::<i32>()
            .map_err(|e| format!("Failed to parse bit rate: {e}"))?,
        tags: parse_ffprobe_tags(data.tags)?,
    }))
}

fn parse_ffprobe_tags(mut tags: FFProbeTags) -> Result<TrackTags, String> {
    let mut tags = std::mem::take(&mut tags)
        .into_iter()
        .map(|(k, v)| (k.to_lowercase(), v))
        .collect::<BTreeMap<_, _>>();

    Ok(TrackTags {
        title: tags.remove("title"),

        artists: tags
            .remove("artist")
            .map(|str| str.split(";").map(str::to_string).collect())
            .unwrap_or_default(),

        composers: tags
            .remove("composer")
            .map(|str| str.split(";").map(str::to_string).collect())
            .unwrap_or_default(),

        album: tags.remove("album"),

        album_artists: tags
            .remove("album_artist")
            .map(|str| str.split(";").map(str::to_string).collect())
            .unwrap_or_default(),

        disc: tags
            .remove("disc")
            .map(|disc| parse_set_number(&disc, "disc"))
            .transpose()?,

        track_no: tags
            .remove("track")
            .map(|track| parse_set_number(&track, "track number"))
            .transpose()?,

        date: tags
            .remove("date")
            .map(|date| parse_date(&date))
            .transpose()?,

        genre: tags.remove("genre"),
    })
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

#[derive(Deserialize)]
struct FFProbeOutput {
    format: FFProbeFormat,
}

#[derive(Deserialize)]
struct FFProbeFormat {
    format_name: String,
    duration: String,
    size: String,
    bit_rate: String,
    tags: FFProbeTags,
}

type FFProbeTags = BTreeMap<String, String>;

lazy_static! {
    static ref PARSE_DISC_NUMBER: Regex = Regex::new(r"^(\d+)(/\d+)?$").unwrap();
    static ref PARSE_TRACK_YEAR_OR_DATE_1: Regex =
        Regex::new(r"^(?P<year>\d\d\d\d)[-/\.\s](?P<month>\d{1,2})[-/\.\s](?P<day>\d{1,2})(?:T\d\d:\d\d:\d\dZ)?$").unwrap();
    static ref PARSE_TRACK_YEAR_OR_DATE_2: Regex =
        Regex::new(r"^(?P<month>\d{1,2})[-/\.\s](?P<day>\d{1,2})[-/\.\s](?P<year>\d\d\d\d)(?:T\d\d:\d\d:\d\dZ)?$").unwrap();
    static ref PARSE_TRACK_YEAR_OR_DATE_3: Regex =
        Regex::new(r"^(?P<year>\d\d\d\d)(?:;.*)?$").unwrap();
}
