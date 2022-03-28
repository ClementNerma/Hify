use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator,
};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use walkdir::WalkDir;

use crate::ffprobe;
use crate::index::{Library, Track};

pub fn build_index(from: &Path) -> Library {
    let mut files = vec![];
    let mut observations = vec![];

    for file in build_files_list(from) {
        match file {
            Ok(file) => files.push(file),
            Err(err) => observations.push(err),
        }
    }

    files.sort();

    let analyzed = files
        .par_iter()
        .enumerate()
        .map(|(_, file)| ffprobe::run_on(&file.path))
        .collect::<Vec<_>>();

    let mut tracks = vec![];
    let mut tracks_files = HashMap::new();

    for (i, track_metadata) in analyzed.into_iter().enumerate() {
        let path_str = &files.get(i).unwrap().path_str;

        let track_metadata = match track_metadata {
            Ok(None) => continue,
            Ok(Some(mt)) => mt,
            Err(err) => {
                observations.push(format!("Error while analyzing file '{path_str}': {err}"));
                continue;
            }
        };

        let mut hasher = DefaultHasher::new();
        path_str.hash(&mut hasher);
        let id = hasher.finish();

        let format = track_metadata.format;

        tracks.push(Track {
            id,
            metadata: track_metadata,
        });

        tracks_files.insert(id, (path_str.clone(), format));
    }

    println!("Finished generating the new index.");

    Library {
        creation_time: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap() // cannot fail as it would imply SystemTime::now() returns a time *earlier* than UNIX_EPOCH
            .as_secs(),
        tracks,
        tracks_files,
        observations,
    }
}

fn build_files_list(from: &Path) -> Vec<Result<FoundFile, String>> {
    WalkDir::new(from)
        .min_depth(1)
        .into_iter()
        .par_bridge()
        .filter_map(|item| {
            let item = match item {
                Ok(item) => item,
                Err(err) => return Some(Err(format!("Failed to read directory entry: {err}"))),
            };

            if !item.path().is_file() {
                return None;
            }

            let result = item
                .path()
                .to_str()
                .map(|path| FoundFile {
                    path: item.path().to_path_buf(),
                    path_str: path.to_string(),
                })
                .ok_or_else(|| {
                    format!(
                        "Item does not have a valid UTF-8 path: {}",
                        item.path().to_string_lossy()
                    )
                });

            Some(result)
        })
        .collect()
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct FoundFile {
    path: PathBuf,
    path_str: String,
}
