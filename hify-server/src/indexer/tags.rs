use crate::index::TrackDate;

use std::{collections::HashSet, sync::LazyLock};

use anyhow::{Context, Result, bail};
use pomsky_macro::pomsky;
use regex::Regex;
use symphonia::core::meta::{MetadataRevision, StandardTag};

pub fn convert_symphonia_metadata(rev: &MetadataRevision) -> Result<TrackStrTags> {
    // TODO: chain &rev.per_track.tags?
    let std_tags = rev
        .media
        .tags
        .iter()
        .filter_map(|tag| tag.std.as_ref())
        .collect::<Vec<_>>();

    macro_rules! tag_str_matcher {
        ($tag:ident) => {
            |std| match std {
                StandardTag::$tag(value) => Some(value.trim().to_owned()),
                _ => None,
            }
        };
    }

    macro_rules! tag_int_matcher {
        ($tag:ident) => {
            |std| match std {
                StandardTag::$tag(value) => Some(*value),
                _ => None,
            }
        };
    }

    let tags = TrackStrTags {
        // Track title
        title: get_tag_str(&std_tags, tag_str_matcher!(TrackTitle))?
            .context("Track title is missing")?,

        // Track artists
        artists: get_tag_str_array(&std_tags, tag_str_matcher!(Artist)),

        // Track composers
        composers: get_tag_str_array(&std_tags, tag_str_matcher!(Composer)),

        // Album name
        album: get_tag_str(&std_tags, tag_str_matcher!(Album))?.context("Album name is missing")?,

        // Album artists
        album_artists: get_tag_str_array(&std_tags, tag_str_matcher!(AlbumArtist)),

        // Disc number
        disc: get_tag_int(&std_tags, tag_int_matcher!(DiscNumber))?
            .map(|disc| u16::try_from(disc).unwrap()),

        // Track number (inside the disc)
        track_no: get_tag_int(&std_tags, tag_int_matcher!(TrackNumber))?
            .map(|track_no| u16::try_from(track_no).unwrap()),

        // Release date
        date: get_tag_str(&std_tags, tag_str_matcher!(ReleaseDate))?
            .or(get_tag_str(
                &std_tags,
                tag_str_matcher!(OriginalReleaseDate),
            )?)
            .map(|date| parse_date(&date))
            .transpose()?,

        // Musical genres
        genres: get_tag_str_array(&std_tags, tag_str_matcher!(Genre)),
    };

    if tags.album_artists.is_empty() {
        bail!("Missing or empty album artist tag!");
    }

    Ok(tags)
}

fn get_tag_str(
    standard_tags: &[&StandardTag],
    matcher: impl Fn(&&StandardTag) -> Option<String>,
) -> Result<Option<String>> {
    let mut iter = standard_tags
        .iter()
        .filter_map(matcher)
        .map(|value| value.trim().to_owned())
        .filter(|entry| !entry.is_empty());

    let Some(value) = iter.next() else {
        return Ok(None);
    };

    if iter.next().is_some() {
        bail!("Multiple values found for tag {:?}", stringify!($tag));
    }

    Ok(Some(value))
}

fn get_tag_str_array(
    standard_tags: &[&StandardTag],
    matcher: impl Fn(&&StandardTag) -> Option<String>,
) -> Vec<String> {
    let mut already_seen = HashSet::new();
    let mut values = vec![];

    for value in standard_tags.iter().filter_map(matcher) {
        for part in value
            .split(&[';', ',', '/'])
            .map(str::trim)
            .filter(|entry| !entry.is_empty())
            .map(str::to_owned)
        {
            if already_seen.insert(part.clone()) {
                values.push(part);
            }
        }
    }

    values
}

fn get_tag_int(
    standard_tags: &[&StandardTag],
    matcher: impl Fn(&&StandardTag) -> Option<u64>,
) -> Result<Option<u64>> {
    let mut iter = standard_tags.iter().filter_map(matcher);

    let Some(value) = iter.next() else {
        return Ok(None);
    };

    if iter.next().is_some() {
        bail!("Multiple values found for tag {:?}", stringify!($tag));
    }

    Ok(Some(value))
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
