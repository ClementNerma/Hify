use anyhow::{bail, Context, Result};
use once_cell::sync::Lazy;
use pomsky_macro::pomsky;
use regex::Regex;
use serde::{de::Error, Deserialize, Deserializer};
use serde_json::Value;

use crate::index::{TrackDate, TrackTags};

pub fn parse_exiftool_tags(tags: ExifToolFileTags) -> Result<TrackTags> {
    let tags = TrackTags {
        title: tags.Title.context("Missing 'title' tag")?,
        artists: tags.Artist.map(parse_array_tag).unwrap_or_default(),
        composers: tags.Composer.map(parse_array_tag).unwrap_or_default(),
        album: tags.Album.context("Missing 'album' tag")?,
        album_artists: tags
            .Albumartist
            .or(tags.Band)
            .map(parse_array_tag)
            .unwrap_or_default(),

        disc: tags
            .Discnumber
            .or(tags.DiscNumber)
            .or(tags.PartOfSet)
            .map(|value| parse_set_number(&value, "disc number"))
            .transpose()?,

        track_no: tags
            .Track
            .or(tags.TrackNumber)
            .map(|value| parse_set_number(&value, "track number"))
            .transpose()?,

        date: tags
            .Year
            .or(tags.ReleaseTime)
            .map(|date| parse_date(&date))
            .transpose()?,

        genres: tags.Genre.map(parse_array_tag).unwrap_or_default(),

        rating: tags
            .Rating
            .map(|rating| Ok(Some(rating as u8)))
            .or_else(|| tags.Popularimeter.map(parse_popularimeter))
            .transpose()?
            .flatten(),
    };

    if tags.artists.is_empty() && tags.album_artists.is_empty() {
        bail!("Both artist(s) AND album artist(s) are missing!");
    }

    Ok(tags)
}

fn parse_set_number(input: &str, category: &'static str) -> Result</*u16*/ i32> {
    PARSE_TRACK_OR_DISC_NUMBER
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

fn parse_popularimeter(popm: impl AsRef<str>) -> Result<Option<u8>> {
    let popm = popm.as_ref();

    let captured = PARSE_MUSICBEE_WMP_POPM
        .captures(popm)
        .with_context(|| format!("Failed to parse 'Popularimeter' value: {}", popm))?;

    match captured.name("score").unwrap().as_str() {
        "0" => Ok(None),
        "1" => Ok(Some(20)),
        "13" => Ok(Some(10)),
        "54" => Ok(Some(30)),
        "64" => Ok(Some(40)),
        "118" => Ok(Some(50)),
        "128" => Ok(Some(60)),
        "186" => Ok(Some(70)),
        "196" => Ok(Some(80)),
        "242" => Ok(Some(90)),
        "255" => Ok(Some(100)),
        score => bail!(
            "Failed to parse score in 'Popularimeter' tag: invalid value {}",
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

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct ExifToolFileTags {
    #[serde(default, deserialize_with = "ensure_string")]
    Album: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    Albumartist: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    Artist: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    Band: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    Composer: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    Discnumber: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    DiscNumber: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    Genre: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    PartOfSet: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    Popularimeter: Option<String>,

    #[serde(default)]
    Rating: Option<f64>,

    #[serde(default, deserialize_with = "ensure_string")]
    ReleaseTime: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    Title: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    Track: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    TrackNumber: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    Year: Option<String>,
}

fn ensure_string<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    let matched: Option<Value> = Deserialize::deserialize(deserializer)?;

    Ok(matched
        .map(decode_value_as_string)
        .transpose()
        .map_err(D::Error::custom)?
        .flatten())
}

fn decode_value_as_string(value: Value) -> Result<Option<String>, String> {
    match value {
        // NOTE: Approx. but no way to do otherwise :(
        Value::Bool(bool) => Ok(Some(if bool { "True" } else { "False" }.to_string())),

        Value::Number(num) => {
            if num.is_u64() {
                Ok(Some(num.to_string()))
            } else {
                Err(format!("Invalid number type (expected u64): {num}"))
            }
        }

        Value::String(str) => Ok(if !str.is_empty() { Some(str) } else { None }),

        Value::Array(values) => {
            let decoded = values
                .into_iter()
                .map(decode_value_as_string)
                .filter_map(|decoded| match decoded {
                    Ok(Some(decoded)) => Some(Ok(decoded)),
                    Ok(None) => None,
                    Err(err) => Some(Err(err)),
                })
                .collect::<Result<Vec<_>, String>>()?;

            // NOTE: joined as ',' as this is the default separator used by ExifTool
            Ok(Some(decoded.join(",")))
        }

        invalid => Err(format!(
            "Invalid value type (expected string or integer): {}",
            invalid
        )),
    }
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
            ['-' '\\' '.' ' ']
            :month([digit]{2})
            ['-' '\\' '.' ' ']
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
            ['-' '\\' '.' ' ']
            :day([digit]{2})
            ['-' '\\' '.' ' ']
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

static PARSE_MUSICBEE_WMP_POPM: Lazy<Regex> = Lazy::new(|| {
    Regex::new(pomsky!(
        Start ("MusicBee" | "Windows Media Player 9 Series") " " :score([digit]+) " 0" End
    ))
    .unwrap()
});
