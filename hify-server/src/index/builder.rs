use anyhow::{bail, Context, Result};

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

    let mut files = build_files_list(&from).context("Failed to build files list")?;
    files.sort();

    log(
        started,
        &format!("Found {} files, analyzing with ExifTool...", files.len()),
    );

    let analyzed = exiftool::run_on(files.as_slice())?;

    let mut tracks = vec![];
    let mut tracks_paths = HashMap::new();

    for (i, track_metadata) in analyzed.into_iter().enumerate() {
        let path = files.get(i).unwrap();
        let path_str = path.to_str().unwrap();

        let mut hasher = DefaultHasher::new();
        let id = TrackID(format!("{:x}", hasher.finish()));
        path_str.hash(&mut hasher);

        tracks_paths.insert(id.clone(), path.clone());

        tracks.push(Track {
            id,
            path: path_str.to_string(),
            metadata: track_metadata,
        });
    }

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
    })
}

fn build_files_list(from: &Path) -> Result<Vec<PathBuf>> {
    WalkDir::new(from)
        .min_depth(1)
        .into_iter()
        .map(|item| {
            let item = item.context("Failed to read directory entry")?;

            if !item.path().is_file() {
                return Ok(None);
            }

            match item.path().to_str() {
                None => bail!(
                    "Item does not have a valid UTF-8 path: {}",
                    item.path().to_string_lossy()
                ),
                Some(_) => Ok(Some(item.path().to_path_buf())),
            }
        })
        .filter_map(|entry| match entry {
            Ok(Some(decoded)) => Some(Ok(decoded)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        })
        .collect::<Result<Vec<_>>>()
}
