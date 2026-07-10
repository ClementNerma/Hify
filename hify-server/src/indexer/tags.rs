use crate::index::TrackDate;

use std::{collections::HashMap, sync::LazyLock};

use anyhow::{Context, Result, bail};
use pomsky_macro::pomsky;
use regex::Regex;
use symphonia::core::meta::{MetadataRevision, StandardTag, Tag};

pub fn convert_symphonia_metadata(rev: &MetadataRevision) -> Result<TrackStrTags> {
    let mut standard_tags = HashMap::new();

    // TODO: chain &rev.per_track.tags?
    for tag in &rev.media.tags {
        let Tag { raw, std } = tag;

        if let Some(std) = std {
            standard_tags.insert(std, raw);
        }
    }

    macro_rules! get_tag_str {
        ($tag:ident) => {
            standard_tags.iter().find_map(|(std, _)| match std {
                StandardTag::$tag(value) => Some(String::clone(&*value)),
                _ => None,
            })
        };
    }

    macro_rules! get_tag_int {
        ($tag:ident) => {
            standard_tags.iter().find_map(|(std, _)| match std {
                StandardTag::$tag(value) => Some(*value),
                _ => None,
            })
        };
    }

    let tags = TrackStrTags {
        title: get_tag_str!(TrackTitle).context("Track title is missing")?,
        artists: get_tag_str!(Artist).map_or_else(Vec::new, parse_array_tag),
        composers: get_tag_str!(Composer).map_or_else(Vec::new, parse_array_tag),
        album: get_tag_str!(Album).context("Album name is missing")?,
        album_artists: get_tag_str!(AlbumArtist).map_or_else(Vec::new, parse_array_tag),
        disc: get_tag_int!(DiscNumber).map(|disc| u16::try_from(disc).unwrap()),
        track_no: get_tag_int!(TrackNumber).map(|track_no| u16::try_from(track_no).unwrap()),
        date: get_tag_str!(ReleaseDate)
            .or_else(|| get_tag_str!(OriginalReleaseDate))
            .map(|date| parse_date(&date))
            .transpose()?,
        genres: get_tag_str!(Genre).map_or_else(Vec::new, parse_array_tag),
    };

    if tags.album_artists.is_empty() {
        bail!("Missing or empty album artist tag!");
    }

    Ok(tags)
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
            .context("Invalid year number")?,
        month: captured
            .name("month")
            .map(|month| month.as_str().parse::<u8>().context("Invalid month number"))
            .transpose()?,
        day: captured
            .name("day")
            .map(|day| day.as_str().parse::<u8>().context("Invalid day number"))
            .transpose()?,
    })
}

fn parse_array_tag(tag_content: impl AsRef<str>) -> Vec<String> {
    tag_content
        .as_ref()
        .split(&[';', ',', '/'])
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(str::to_owned)
        .collect()
}

static PARSE_TRACK_YEAR_OR_DATE_1: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(pomsky!(
        Start
            :year([digit]{4})
            ['-' '/' '\\' '.' ' ']
            :month([digit]{2})
            ['-' '/' '\\' '.' ' ']
            :day([digit]{2})
            ('T' ['0'-'9' ':' 'Z']+)?
        End

    ))
    .unwrap()
});

static PARSE_TRACK_YEAR_OR_DATE_2: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(pomsky!(
        Start
            :month([digit]{2})
            ['-' '/' '\\' '.' ' ']
            :day([digit]{2})
            ['-' '/' '\\' '.' ' ']
            :year([digit]{4})
            ('T' ['0'-'9' ':' 'Z']+)?
        End
    ))
    .unwrap()
});

static PARSE_TRACK_YEAR_OR_DATE_3: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(pomsky!(
        Start
            :year([digit]{4})
            (';' | End)
    ))
    .unwrap()
});

// static PARSE_MUSICBEE_WMP_POPM: LazyLock<Regex> = LazyLock::new(|| {
//     Regex::new(pomsky!(
//         Start ("MusicBee" | "Windows Media Player 9 Series") " " :score([digit]+) " 0" End
//     ))
//     .unwrap()
// });

/// List of audio tags
#[derive(Debug)]
pub struct TrackStrTags {
    /// The track's title
    pub title: String,

    /// The track's artists list
    pub artists: Vec<String>,

    /// The track's composers
    pub composers: Vec<String>,

    /// The track's album
    pub album: String,

    /// The track's album artists list
    pub album_artists: Vec<String>,

    /// The disc number the track is present on
    pub disc: Option<u16>,

    /// The track's number in its own disc
    pub track_no: Option<u16>,

    /// The track's release date
    pub date: Option<TrackDate>,

    /// The track's genres list
    pub genres: Vec<String>,
}
