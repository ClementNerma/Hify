use id3::{Tag as ID3Tag, TagLike};
use std::{
    collections::HashMap,
    error::Error,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
use walkdir::WalkDir;

use crate::index::{AudioFormat, Library, Track};

pub fn build_index(from: &Path) -> Library {
    let creation_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap() // cannot fail as it would imply SystemTime::now() returns a time *earlier* than UNIX_EPOCH
        .as_secs();

    let mut tracks = vec![];
    let mut invalid_files = vec![];
    let mut tracks_files = HashMap::new();
    let mut counter = 0;

    for item in WalkDir::new(from).min_depth(1) {
        match item.and_then(|item| item.metadata().map(|metadata| (item, metadata))) {
            Err(error) => eprintln!("Failed to read item: {}", error),
            Ok((item, metadata)) => {
                if metadata.is_file() {
                    let path = item.path();

                    match analyze_track(path, counter) {
                        Err(error) => {
                            let path = path.to_string_lossy();
                            eprintln!("Failed to analyze file at '{}': {}", path, error);
                            invalid_files.push(path.to_string());
                        }
                        Ok(Some(track)) => {
                            tracks_files.insert(track.id, path.to_string_lossy().to_string());
                            tracks.push(track);
                            counter += 1;
                        }
                        Ok(None) => {}
                    }
                }
            }
        }
    }

    Library {
        creation_time,
        tracks,
        invalid_files,
        tracks_files,
    }
}

fn analyze_track(from: &Path, id: u64) -> Result<Option<Track>, Box<dyn Error>> {
    match from.extension() {
        None => Err("File does not have an extension".into()),
        Some(ext) => {
            let ext = ext
                .to_str()
                .ok_or("File does not have a valid UTF-8 extension")?
                .to_ascii_lowercase();

            match ext.as_str() {
                "mp3" => analyze_mp3_track(from, id).map(Option::Some),

                "flac" => Err("This file format is not supported yet".into()),
                "wave" => Err("This file format is not supported yet".into()),
                "ogg" => Err("This file format is not supported yet".into()),
                "webm" => Err("This file format is not supported yet".into()),

                _ => Ok(None),
            }
        }
    }
}

fn analyze_mp3_track(file: &Path, id: u64) -> Result<Track, Box<dyn Error>> {
    let tag = ID3Tag::read_from_path(file)?;

    Ok(Track {
        id,
        format: AudioFormat::MP3,
        title: tag.title().map(ToString::to_string),

        artist: tag.artist().map(ToString::to_string),
        disc: tag.disc(),
        track_no: tag.track(),

        album: tag.album().map(ToString::to_string),
        album_artist: tag.album_artist().map(ToString::to_string),

        year: tag.year(),

        genre: tag.genre().map(ToString::to_string),

        duration: tag.duration(),
    })
}
