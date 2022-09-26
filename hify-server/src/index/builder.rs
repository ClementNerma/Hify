use anyhow::{anyhow, Context, Result};

use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};
use walkdir::WalkDir;

use super::{
    arts::find_albums_arts,
    cache::build_index_cache,
    data::{Index, Track, TrackID},
    exiftool,
    sorted_map::SortedMap,
};

pub fn log(time: SystemTime, message: &str) {
    let elapsed = match time.elapsed() {
        Ok(time) => time.as_secs().to_string(),
        Err(_) => "?".to_string(),
    };

    println!("[{: >4}s] {message}", elapsed);
}

pub fn build_index(from: PathBuf) -> Result<Index> {
    let started = SystemTime::now();

    log(started, "Looking for audio files...");

    let mut files = vec![];
    let mut observations = vec![];

    for file in build_files_list(&from) {
        match file {
            Ok(file) => files.push(file),
            Err(err) => observations.push(format!("{err:?}")),
        }
    }

    files.sort();

    log(
        started,
        &format!("Found {} files, analyzing with ExifTool...", files.len()),
    );

    let analyzed = exiftool::run_on(
        files
            .iter()
            .map(|file| &file.path)
            .collect::<Vec<_>>()
            .as_slice(),
    )?;

    let mut tracks = vec![];
    let mut tracks_paths = HashMap::new();

    for (i, track_metadata) in analyzed.into_iter().enumerate() {
        let FoundFile { path, path_str } = &files.get(i).unwrap();

        let mut hasher = DefaultHasher::new();
        path_str.hash(&mut hasher);
        let id = TrackID(format!("{:x}", hasher.finish()));

        tracks_paths.insert(id.clone(), path.clone());

        tracks.push(Track {
            id,
            path: path_str.clone(),
            metadata: track_metadata,
        });
    }

    log(
        started,
        &format!("Emitted {} observations.", observations.len()),
    );

    log(
        started,
        &format!("Collected {} tracks, generating cache...", tracks.len()),
    );

    let tracks = SortedMap::from_vec(tracks, |track| track.id.clone());

    let cache = build_index_cache(&tracks, tracks_paths);

    log(started, "Searching for album arts...");

    let found_album_arts = AtomicUsize::new(0);

    let albums_arts = find_albums_arts(
        cache.albums_infos.keys().collect::<Vec<_>>().as_slice(),
        &cache,
    );

    log(
        started,
        &format!(
            "Found {}/{} album arts.",
            found_album_arts.load(Ordering::SeqCst),
            cache.albums_infos.len()
        ),
    );

    log(started, "Index has been generated.");

    let fingerprint = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap() // cannot fail as it would imply SystemTime::now() returns a time *earlier* than UNIX_EPOCH
        .as_secs()
        .to_string();

    Ok(Index {
        fingerprint,
        from,
        tracks,
        albums_arts,
        cache,
        observations,
    })
}

fn build_files_list(from: &Path) -> Vec<Result<FoundFile>> {
    WalkDir::new(from)
        .min_depth(1)
        .into_iter()
        .filter_map(|item| {
            let item = match item {
                Ok(item) => item,
                Err(err) => return Some(Err(anyhow!("Failed to read directory entry: {err}"))),
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
                .with_context(|| {
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
