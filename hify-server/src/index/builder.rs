use anyhow::{bail, Context, Result};

use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use walkdir::WalkDir;

use super::{
    arts::find_albums_arts,
    cache::build_index_cache,
    data::{Index, Track, TrackID},
    exiftool,
    sorted_map::SortedMap,
    IndexCache,
};

pub fn log(time: SystemTime, message: &str) {
    let elapsed = match time.elapsed() {
        Ok(time) => time.as_secs().to_string(),
        Err(_) => "?".to_string(),
    };

    println!("[{: >4}s] {message}", elapsed);
}

pub fn build_index(dir: PathBuf, from: Option<Index>) -> Result<Index> {
    let from = from.unwrap_or_else(|| Index {
        from: dir.clone(),
        fingerprint: String::new(),
        tracks: SortedMap::empty(),
        albums_arts: HashMap::new(),
        cache: IndexCache {
            tracks_paths: HashMap::new(),
            artists_albums: HashMap::new(),
            artists_tracks: HashMap::new(),
            albums_artists_albums: HashMap::new(),
            albums_tracks: HashMap::new(),
            artists_infos: SortedMap::empty(),
            albums_artists_infos: SortedMap::empty(),
            albums_infos: SortedMap::empty(),
            genres_tracks: HashMap::new(),
            no_genre_tracks: HashSet::new(),
            genres_albums: HashMap::new(),
        },
    });

    let started = SystemTime::now();

    log(started, "Looking for audio files...");

    let files = build_files_list(&dir).context("Failed to build files list")?;

    let existing = &from
        .cache
        .tracks_paths
        .values()
        .cloned()
        .collect::<HashSet<_>>();

    let mut files = files
        .difference(existing)
        .filter(|path| exiftool::is_audio_file(path))
        .cloned()
        .collect::<Vec<_>>();

    files.sort();

    log(started, &format!("Found {} files.", files.len(),));
    log(started, "Extracting audio metadata...");

    let analyzed = exiftool::run_on(files.as_slice())?;

    let mut tracks = from.tracks.into_values();
    let mut tracks_paths = from.cache.tracks_paths;

    for (path, track_metadata) in analyzed.into_iter() {
        let path_str = path.to_str().unwrap();

        let id = get_track_id(path_str);

        tracks_paths.insert(id.clone(), path.to_path_buf());

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

    let albums_arts = find_albums_arts(
        cache.albums_infos.keys().collect::<Vec<_>>().as_slice(),
        &cache,
    );

    log(started, "Index has been generated.");

    let fingerprint = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap() // cannot fail as it would imply SystemTime::now() returns a time *earlier* than UNIX_EPOCH
        .as_secs()
        .to_string();

    Ok(Index {
        fingerprint,
        from: dir,
        tracks,
        albums_arts,
        cache,
    })
}

fn build_files_list(from: &Path) -> Result<HashSet<PathBuf>> {
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
        .collect()
}

fn get_track_id(track_path_str: &str) -> TrackID {
    let mut hasher = DefaultHasher::new();
    track_path_str.hash(&mut hasher);
    TrackID(format!("{:x}", hasher.finish()))
}
