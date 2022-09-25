use std::{
    io::{stdout, BufRead, BufReader, Write},
    path::Path,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    time::Instant,
};

use anyhow::{anyhow, bail, Context, Result};
use once_cell::sync::Lazy;
use pomsky_macro::pomsky;
use regex::Regex;
use serde::{de::Error, Deserialize, Deserializer};
use serde_json::Value;

use crate::index::{AudioFormat, TrackDate, TrackMetadata, TrackTags};

pub fn run_on(files: &[impl AsRef<Path>]) -> Result<Vec<TrackMetadata>> {
    let files = files
        .iter()
        .filter_map(|file| {
            let audio_ext = file
                .as_ref()
                .extension()
                .and_then(|ext| ext.to_str())?
                .to_ascii_lowercase();

            if matches!(
                audio_ext.as_str(),
                "mpeg" | "mp4" | "alac" | "webm" | "aiff" | "dsf"
            ) {
                return Some(Err(anyhow!(
                    "File format unsupported by web players: {audio_ext}"
                )));
            }

            if !matches!(
                audio_ext.as_str(),
                "mp3" | "flac" | "wav" | "aac" | "ogg" | "m4a"
            ) {
                return None;
            }

            Some(Ok(file.as_ref().to_path_buf()))
        })
        .collect::<Result<Vec<_>>>()?;

    let started = Instant::now();
    let mut previous = 0;

    let display_progress = |elapsed: u64, current: u64, total: u64| {
        print!(
            "\r[     ] Progress: {} / {} ({}%) in {}s...",
            current,
            total,
            current * 100 / total,
            elapsed
        );

        stdout().flush().unwrap();
    };

    print!("Starting analysis...");

    let files_count = u64::try_from(files.len()).unwrap();

    const FILES_PER_CHUNK: usize = 100;

    let mut analyzed = vec![];

    for (chunk_num, files) in files.chunks(FILES_PER_CHUNK).enumerate() {
        let chunk_start = u64::try_from(FILES_PER_CHUNK * chunk_num).unwrap();

        let mut handle = Command::new("exiftool")
            .args(&["-n", "-json", "-progress"])
            .args(files)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to launch ExifTool")?;

        let stdout_reader = BufReader::new(
            handle
                .stdout
                .take()
                .context("Failed to get command's STDOUT")?,
        );

        let stderr_reader = BufReader::new(
            handle
                .stderr
                .take()
                .context("Failed to get the command's SDTERR")?,
        );

        let stdout_lines = Arc::new(Mutex::new(vec![]));
        let stderr_lines = Arc::new(Mutex::new(vec![]));

        let stdout_lines_for_reader = Arc::clone(&stdout_lines);
        let stderr_lines_for_reader = Arc::clone(&stderr_lines);

        std::thread::spawn(move || {
            for line in stdout_reader.lines() {
                match line {
                    Ok(line) => stdout_lines_for_reader.lock().unwrap().push(line),
                    Err(err) => {
                        eprintln!("{err:?}");
                        // TODO
                    }
                }
            }
        });

        std::thread::spawn(move || {
            for line in stderr_reader.lines() {
                match line {
                    Ok(line) => match PARSE_PROGRESS_LINE.captures(&line) {
                        Some(m) => {
                            let current =
                                m.name("current").unwrap().as_str().parse::<u64>().unwrap();

                            let elapsed = started.elapsed().as_secs();

                            if elapsed != previous || current == files_count {
                                previous = elapsed;
                                display_progress(elapsed, chunk_start + current, files_count);
                            }
                        }
                        None => {
                            stderr_lines_for_reader.lock().unwrap().push(line);
                        }
                    },
                    Err(err) => {
                        eprintln!("{err:?}");
                        // TODO
                    }
                }
            }
        });

        let status = handle.wait().with_context(|| {
            format!(
                "ExifTool failed: {}",
                stderr_lines.lock().unwrap().join("\n")
            )
        })?;

        if !status.success() {
            bail!(
                "ExifTool failed: {}",
                stderr_lines.lock().unwrap().join("\n")
            );
        }

        let stdout_lines = stdout_lines.lock().unwrap().join("\n");

        let parsed_output = serde_json::from_str::<ExifToolOutput>(&stdout_lines)
            .with_context(|| format!("Failed to parse ExifTool output:\n\n{}", stdout_lines))?;

        analyzed.extend(parsed_output.0);
    }

    let files_count = files.len();

    if analyzed.len() != files_count {
        bail!(
            "Found invalid number of results returned by ExifTool: expected {}, found {}",
            files_count,
            analyzed.len()
        );
    }

    let mut successes = vec![];
    let mut errors = vec![];

    for (i, analyzed) in analyzed.into_iter().enumerate() {
        match process_analyzed_file(analyzed) {
            Ok(data) => successes.push(data),
            Err(err) => errors.push((files.get(i).unwrap(), err)),
        }
    }

    if !errors.is_empty() {
        bail!(
            "Failed with the following errors:\n\n{}",
            errors
                .iter()
                .map(|(success, err)| format!("* {}: {err:?}", success.to_string_lossy()))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    Ok(successes)
}

fn process_analyzed_file(analyzed: ExifToolFile) -> Result<TrackMetadata> {
    let format = match analyzed.FileType.as_str() {
        "FLAC" => AudioFormat::FLAC,
        "MP3" => AudioFormat::MP3,
        "WAV" => AudioFormat::WAV,
        "AAC" => AudioFormat::AAC,
        "OGG" => AudioFormat::OGG,
        "M4A" => AudioFormat::M4A,
        codec_name => bail!("Unknown codec name: {codec_name}"),
    };

    Ok(TrackMetadata {
        format,
        size: i32::try_from(analyzed.FileSize).with_context(|| {
            format!(
                "Size is too big to be returned to GraphQL: {}",
                analyzed.FileSize
            )
        })?,
        duration: analyzed.Duration as i32,
        bitrate: analyzed.AudioBitrate.map(|br| br as i32),
        tags: parse_exiftool_tags(analyzed.tags)?,
    })
}

fn parse_exiftool_tags(tags: ExifToolFileTags) -> Result<TrackTags> {
    Ok(TrackTags {
        title: tags.Title.context("Missing 'title' tag")?,
        artists: tags.Artist.map(parse_array_tag).unwrap_or_default(),
        composers: tags.Composer.map(parse_array_tag).unwrap_or_default(),
        album: tags.Album.context("Missing 'album' tag")?,
        album_artists: tags.Band.map(parse_array_tag).unwrap_or_default(),

        disc: tags
            .Discnumber
            .or(tags.PartOfSet)
            .map(|value| parse_set_number(&value, "disc number"))
            .transpose()?,

        track_no: tags
            .Track
            .or(tags.TrackNumber)
            .map(|value| parse_set_number(&value, "track number"))
            .transpose()?,

        date: tags.Year.map(|date| parse_date(&date)).transpose()?,

        genres: tags.Genre.map(parse_array_tag).unwrap_or_default(),
    })
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

    #[serde(default)]
    AudioBitrate: Option<f64>,

    #[serde(flatten)]
    tags: ExifToolFileTags,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct ExifToolFileTags {
    #[serde(default, deserialize_with = "ensure_string")]
    Album: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    Artist: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    Band: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    Composer: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    Discnumber: Option<String>,

    #[serde(default)]
    Genre: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    PartOfSet: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    Title: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    Track: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    TrackNumber: Option<String>,

    #[serde(default, deserialize_with = "ensure_string")]
    Year: Option<String>,
    // #[serde(default)]
    // Popularimeter: Option<String>,
}

fn ensure_string<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    let matched: Option<Value> = Deserialize::deserialize(deserializer)?;

    matched
        .map(decode_value_as_string)
        .transpose()
        .map_err(D::Error::custom)
}

fn decode_value_as_string(value: Value) -> Result<String, String> {
    match value {
        // NOTE: Approx. but no way to do otherwise :(
        Value::Bool(bool) => Ok(if bool { "True" } else { "False" }.to_string()),

        Value::Number(num) => {
            if num.is_u64() {
                Ok(num.to_string())
            } else {
                Err(format!("Invalid number type (expected u64): {num}"))
            }
        }

        Value::String(str) => Ok(str),

        Value::Array(values) => {
            let decoded = values
                .into_iter()
                .map(decode_value_as_string)
                .collect::<Result<Vec<_>, String>>()?;

            // NOTE: joined as ',' as this is the default separator used by ExifTool
            Ok(decoded.join(","))
        }

        invalid => Err(format!(
            "Invalid value type (expected string or integer): {}",
            invalid
        )),
    }
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

static PARSE_PROGRESS_LINE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(pomsky!(
        Start
            "======== " Codepoint+ " [" :current([digit]+) "/" [digit]+ "]"
        End
    ))
    .unwrap()
});
