use anyhow::{bail, Context, Result};
use rayon::prelude::{ParallelBridge, ParallelIterator};

use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    time::{Instant, SystemTime, UNIX_EPOCH},
};
use walkdir::WalkDir;

use super::{
    arts::find_albums_arts,
    cache::build_index_cache,
    data::{Index, Track},
    exiftool,
    sorted_map::SortedMap,
    IndexCache,
};

pub fn log(time: Instant, message: &str) {
    println!("[{: >4}s] {message}", time.elapsed().as_secs());
}

pub fn build_index(dir: PathBuf, from: Option<Index>) -> Result<Index> {
    let from = from.unwrap_or_else(|| Index {
        from: dir.clone(),
        fingerprint: String::new(),
        tracks: SortedMap::empty(),
        albums_arts: HashMap::new(),
        cache: IndexCache {
            tracks_paths: HashMap::new(),
            tracks_all_artists: HashMap::new(),

            artists_albums: HashMap::new(),
            artists_album_participations: HashMap::new(),
            artists_tracks: HashMap::new(),
            artists_track_participations: HashMap::new(),

            artists_mean_score: HashMap::new(),
            albums_artists_mean_score: HashMap::new(),

            albums_tracks: HashMap::new(),
            albums_mean_score: HashMap::new(),

            artists_infos: SortedMap::empty(),
            albums_artists_infos: SortedMap::empty(),
            albums_infos: SortedMap::empty(),
            genre_infos: SortedMap::empty(),

            genres_albums: HashMap::new(),
            genres_tracks: HashMap::new(),
            no_genre_tracks: HashSet::new(),
        },
    });

    let started = Instant::now();

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

        let track = Track::new(path_str.to_string(), track_metadata);

        tracks_paths.insert(track.id, path.to_path_buf());

        tracks.push(track);
    }

    log(
        started,
        &format!("Collected {} tracks, generating cache...", tracks.len()),
    );

    let tracks = SortedMap::from_vec(tracks, |track| track.id.clone());

    let cache = build_index_cache(&tracks, tracks_paths);

    let new_albums = cache
        .albums_infos
        .keys()
        .filter(|key| !from.cache.albums_infos.contains_key(key))
        .collect::<Vec<_>>();

    log(
        started,
        &format!("Searching for new albums' ({}) arts...", new_albums.len()),
    );

    let mut albums_arts = from.albums_arts;
    albums_arts.extend(find_albums_arts(&new_albums, &cache));

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

pub fn rebuild_cache(index: &mut Index) {
    index.cache = build_index_cache(&index.tracks, index.cache.tracks_paths.clone());
}

pub fn rebuild_arts(index: &mut Index) {
    index.albums_arts = find_albums_arts(
        index
            .cache
            .albums_infos
            .keys()
            .collect::<Vec<_>>()
            .as_slice(),
        &index.cache,
    );
}

fn build_files_list(from: &Path) -> Result<HashSet<PathBuf>> {
    WalkDir::new(from)
        .min_depth(1)
        .into_iter()
        .par_bridge()
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
