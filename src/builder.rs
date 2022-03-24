use id3::{Tag, TagLike};
use std::{
    collections::HashMap,
    error::Error,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
use walkdir::WalkDir;

use crate::index::{Library, Track};

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
                        Ok(track) => {
                            tracks_files.insert(track.id, path.to_string_lossy().to_string());
                            tracks.push(track);
                            counter += 1;
                        }
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

fn analyze_track(from: &Path, id: u64) -> Result<Track, Box<dyn Error>> {
    let tag = Tag::read_from_path(from)?;

    Ok(Track {
        id,
        title: tag.title().map(ToString::to_string),

        artist: tag.artist().map(ToString::to_string),
        disc: tag.disc(),
        track_no: tag.track(),

        album: tag.album().map(ToString::to_string),

        year: tag.year(),

        genre: tag.genre().map(ToString::to_string),

        duration: tag.duration().ok_or("File does not have a duration")?,
    })
}
