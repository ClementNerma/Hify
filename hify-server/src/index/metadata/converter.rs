use anyhow::{anyhow, bail, Context, Result};
use once_cell::sync::Lazy;
use pomsky_macro::pomsky;
use regex::Regex;
use symphonia::core::meta::{MetadataRevision, StandardTagKey, Tag, Value};

use crate::index::{Rating, TrackDate, TrackTags};

pub fn convert_symphonia_metadata(rev: MetadataRevision) -> Result<TrackTags> {
    // TODO: change to HashMap when StandardTagKey implements Hash
    let mut standard_tags = vec![];

    for tag in rev.tags() {
        let Tag {
            key: _, // NOTE: can be duplicates
            std_key,
            value,
        } = tag;

        if let Some(std_key) = std_key {
            standard_tags.push((*std_key, value));
        }
    }

    let get_tag_str = |name: StandardTagKey| -> Result<Option<String>> {
        let value = standard_tags
            .iter()
            .find_map(|(key, value)| if *key == name { Some(value) } else { None });

        let value = value
            .map(|value| {
                decode_value_as_string(value)
                    .with_context(|| format!("Failed to decode tag '{name:?}'"))
            })
            .transpose()?;

        Ok(value.flatten())
    };

    let tags = TrackTags {
        title: get_tag_str(StandardTagKey::TrackTitle)?.context("Track title is missing")?,
        artists: get_tag_str(StandardTagKey::Artist)?
            .map(parse_array_tag)
            .unwrap_or_default(),
        composers: get_tag_str(StandardTagKey::Composer)?
            .map(parse_array_tag)
            .unwrap_or_default(),
        album: get_tag_str(StandardTagKey::Album)?.context("Album name is missing")?,
        album_artists: get_tag_str(StandardTagKey::AlbumArtist)?
            .map(parse_array_tag)
            .unwrap_or_default(),
        disc: get_tag_str(StandardTagKey::DiscNumber)?
            .map(|value| parse_set_number(&value, "disc number"))
            .transpose()?,
        track_no: get_tag_str(StandardTagKey::TrackNumber)?
            .map(|value| parse_set_number(&value, "track number"))
            .transpose()?,
        date: get_tag_str(StandardTagKey::ReleaseDate)?
            .map(|date| parse_date(&date))
            .transpose()?,
        genres: get_tag_str(StandardTagKey::Genre)?
            .map(parse_array_tag)
            .unwrap_or_default(),
        rating: get_tag_str(StandardTagKey::Rating)?
            .map(parse_popularimeter)
            .transpose()?
            .flatten()
            .map(|rating| {
                Rating::parse(rating).map_err(|()| {
                    anyhow!(
                        "Invalid rating found in file: expected a value between 0 and 10, got {rating}")
                    })
            })
            .transpose()?,
    };

    if tags.artists.is_empty() && tags.album_artists.is_empty() {
        bail!("Both artist(s) AND album artist(s) are missing!");
    }

    Ok(tags)
}

fn parse_set_number(input: &str, category: &'static str) -> Result<u32> {
    PARSE_TRACK_OR_DISC_NUMBER
        .captures(input)
        .with_context(|| format!("Invalid {category} value: {input}"))
        .and_then(|c| {
            c.name("number").unwrap().as_str().parse().with_context(|| {
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
            .parse::<u32>()
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

fn parse_popularimeter(popm: impl AsRef<str>) -> Result<Option<u8>> {
    match popm.as_ref() {
        // No rating
        "0" => Ok(None),

        // Normal
        "10" => Ok(Some(1)),
        "20" => Ok(Some(2)),
        "30" => Ok(Some(3)),
        "40" => Ok(Some(4)),
        "50" => Ok(Some(5)),
        "60" => Ok(Some(6)),
        "70" => Ok(Some(7)),
        "80" => Ok(Some(8)),
        "90" => Ok(Some(9)),
        "100" => Ok(Some(10)),

        // MusicBee
        "1" => Ok(Some(2)),
        "13" => Ok(Some(1)),
        "54" => Ok(Some(3)),
        "64" => Ok(Some(4)),
        "118" => Ok(Some(5)),
        "128" => Ok(Some(6)),
        "186" => Ok(Some(7)),
        "196" => Ok(Some(8)),
        "242" => Ok(Some(9)),
        "255" => Ok(Some(10)),

        // Unknown
        score => bail!(
            "Failed to parse rating tag: found invalid value '{}'",
            score
        ),
    }
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

fn decode_value_as_string(value: &Value) -> Result<Option<String>> {
    Ok(Some(match value {
        Value::Binary(_) => bail!("Found unsupported tag data (binary)"),
        Value::Boolean(_) => bail!("Found unsupported tag data (boolean)"),
        Value::Flag => bail!("Found unsupported tag data (flag)"),
        Value::Float(float) => float.to_string(),
        Value::SignedInt(int) => int.to_string(),
        Value::UnsignedInt(uint) => uint.to_string(),
        Value::String(str) => {
            if str.trim().is_empty() {
                return Ok(None);
            }

            str.clone()
        }
    }))
}

static PARSE_TRACK_OR_DISC_NUMBER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(pomsky!(
        Start
            :number([digit]+)
            (("/" | " of ") [digit]+)?
        End
    ))
    .unwrap()
});

static PARSE_TRACK_YEAR_OR_DATE_1: Lazy<Regex> = Lazy::new(|| {
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

static PARSE_TRACK_YEAR_OR_DATE_2: Lazy<Regex> = Lazy::new(|| {
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

static PARSE_TRACK_YEAR_OR_DATE_3: Lazy<Regex> = Lazy::new(|| {
    Regex::new(pomsky!(
        Start
            :year([digit]{4})
            (';' | End)
    ))
    .unwrap()
});

// static PARSE_MUSICBEE_WMP_POPM: Lazy<Regex> = Lazy::new(|| {
//     Regex::new(pomsky!(
//         Start ("MusicBee" | "Windows Media Player 9 Series") " " :score([digit]+) " 0" End
//     ))
//     .unwrap()
// });
