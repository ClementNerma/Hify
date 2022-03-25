use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fs,
    hash::{Hash, Hasher},
    path::Path,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};
use walkdir::WalkDir;

use crate::index::{AudioFormat, Library, Track, TrackDuration, TrackMetadata};

pub fn build_index(from: &Path) -> Result<Library, ()> {
    let creation_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap() // cannot fail as it would imply SystemTime::now() returns a time *earlier* than UNIX_EPOCH
        .as_secs();

    let files = WalkDir::new(from)
        .min_depth(1)
        .into_iter()
        .filter_map(|item| match item {
            Ok(item) if item.path().is_file() => {
                item.path().to_str().map(ToString::to_string).or_else(|| {
                    eprintln!(
                        "Item does not have a valid UTF-8 path: {}",
                        item.path().to_string_lossy()
                    );
                    None
                })
            }
            Ok(_) => None,
            Err(err) => {
                eprintln!("Failed to read item: {}", err);
                None
            }
        })
        .collect::<Vec<_>>();

    println!("Found {} audio files to analyze.", files.len());

    let files_list_file = Path::new("/tmp/test-to-generate-randomly.txt");

    fs::write(files_list_file, files.join("\n")).unwrap();

    let cmd = Command::new("exiftool")
        .args(&[
            "-json",
            "-@",
            files_list_file.to_str().unwrap(),
            // List of fields to get
            "-sourcefile",
            "-filetype",
            "-title",
            "-artist",
            "-composer",
            "-album",
            "-albumartist",
            "-discnumber",
            "-partofset",
            "-track",
            "-tracknumber",
            "-year",
            "-date",
            "-genre",
            "-duration",
            "-samplerate",
            "-bitspersample",
        ])
        .output()
        .map_err(|err| eprintln!("Failed to run ExifTool: {}", err))?;

    println!("Finished running ExifTool on all files.");

    let stdout = std::str::from_utf8(&cmd.stdout).expect("ExifTool returned invalid UTF-8 data");

    let out = serde_json::from_str::<ExifToolOutput>(stdout)
        .map_err(|err| eprintln!("Failed to deserialize ExifTool response: {}", err))?;

    if out.0.len() != files.len() {
        eprintln!(
            "ExifTool didn't return the same number of items ({}) than the number of analyzed audio files ({})",
            out.0.len(),
            files.len()
        );
        return Err(());
    }

    println!("Finished parsing and validating ExifTool's JSON output");

    let mut tracks = vec![];
    let mut tracks_files = HashMap::new();

    for (i, track) in out.0.into_iter().enumerate() {
        let file = files.get(i).unwrap();

        let format = match track.FileType.as_deref() {
            Some("MP3") => AudioFormat::MP3,
            Some("FLAC") => AudioFormat::FLAC,
            _ => {
                println!("{:#?}", track);
                println!("Ignored non-audio file: {}", file);
                continue;
            }
        };

        let mut hasher = DefaultHasher::new();
        file.hash(&mut hasher);

        let id = hasher.finish();

        tracks.push(Track {
            id,
            format,
            metadata: parse_exif_tool_file(track),
        });

        tracks_files.insert(id, file.clone());
    }

    println!("Finished generating the new index.");

    Ok(Library {
        creation_time,
        tracks,
        tracks_files,
    })
}

#[derive(Debug, Deserialize)]
struct ExifToolOutput(Vec<ExifToolFile>);

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct ExifToolFile {
    FileType: Option<String>,

    Title: Option<String>,

    Artist: Option<String>,
    Composer: Option<String>,

    Album: Option<String>,
    Albumartist: Option<String>,

    Discnumber: Option<String>,
    PartOfSet: Option<String>,
    Track: Option<u32>,
    Tracknumber: Option<u32>,

    Year: Option<u32>,
    Date: Option<u32>,

    Genre: Option<String>,

    Duration: Option<String>,

    SampleRate: Option<u32>,
    BitsPerSample: Option<u8>,
}

lazy_static! {
    static ref PARSE_DISC_NUMBER: Regex = Regex::new(r"^(\d+)(/\d+)?$").unwrap();
    static ref PARSE_TRACK_DURATION: Regex =
        Regex::new(r"^(\d+):(\d+):(\d+)(\s\(approx\))?$").unwrap();
    static ref PARSE_ALT_TRACK_DURATION: Regex =
        Regex::new(r"^(\d+)\.\d+\ss(\s\(approx\))?$").unwrap();
}

// TODO: Return reports for invalid fields value etc.
fn parse_exif_tool_file(input: ExifToolFile) -> TrackMetadata {
    TrackMetadata {
        title: input.Title,
        artist: input.Artist,
        composer: input.Composer,
        album: input.Album,
        album_artist: input.Albumartist,
        disc: input.Discnumber.or(input.PartOfSet).and_then(|disc| {
            PARSE_DISC_NUMBER
                .captures(&disc)
                .map(|disc| disc.get(1).unwrap())
                .and_then(|disc| disc.as_str().parse::<u32>().ok())
        }),
        track_no: input.Track.or(input.Tracknumber),
        year: input.Year.or(input.Date),
        genre: input.Genre,
        duration: input.Duration.and_then(|duration| {
            PARSE_TRACK_DURATION
                .captures(&duration)
                .and_then(|duration| {
                    Some(TrackDuration {
                        hours: duration.get(1).unwrap().as_str().parse::<u8>().ok()?,
                        minutes: duration.get(2).unwrap().as_str().parse::<u8>().ok()?,
                        seconds: duration.get(3).unwrap().as_str().parse::<u8>().ok()?,
                        approx: duration.get(4).is_some(),
                    })
                })
                .or_else(|| {
                    PARSE_ALT_TRACK_DURATION
                        .captures(&duration)
                        .and_then(|duration| {
                            let seconds = duration.get(1).unwrap().as_str().parse::<u32>().ok()?;

                            Some(TrackDuration {
                                hours: (seconds / 3600) as u8,
                                minutes: (seconds / 60) as u8,
                                seconds: (seconds % 60) as u8,
                                approx: duration.get(2).is_some(),
                            })
                        })
                })
        }),
        bitrate: input.SampleRate,
        resolution: input.BitsPerSample,
    }
}
