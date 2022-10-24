use anyhow::{bail, Context, Result};
use rayon::prelude::{ParallelBridge, ParallelIterator};

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    path::{Path, PathBuf},
    time::{Instant, SystemTime, UNIX_EPOCH},
};
use walkdir::WalkDir;

use super::{
    arts::{find_albums_arts, generate_artist_art},
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
        arts: HashMap::new(),
        cache: IndexCache {
            tracks_files_mtime: HashMap::new(),
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

    let files_mtime = files
        .into_iter()
        .filter(|(path, _)| exiftool::is_audio_file(path))
        .filter(|(path, mtime)| {
            match from
                .cache
                .tracks_files_mtime
                .get(path.strip_prefix(&dir).expect(
                    "Internal error: audio file path couldn't be stripped of the base directory",
                )) {
                None => true,
                Some(old_mtime) => old_mtime != mtime,
            }
        })
        .collect::<BTreeMap<_, _>>();

    log(started, &format!("Found {} files.", files_mtime.len()));
    log(started, "Extracting audio metadata...");

    // Run analysis tool on all new and modified files
    let analyzed = exiftool::run_on(files_mtime.keys().cloned().collect::<Vec<_>>().as_slice())?;

    // Turn the analyzed files into tracks
    let analyzed = analyzed
        .into_iter()
        .map(|(path, metadata)| {
            Track::new(
                path.strip_prefix(&dir)
                    .expect("Internal error: track path couldn't be stripped of the base directory")
                    .to_path_buf(),
                *files_mtime.get(&path).unwrap(),
                metadata,
            )
        })
        .collect::<Vec<_>>();

    // Remove previous versions of analyzed files
    let analyzed_ids = analyzed
        .iter()
        .map(|track| &track.id)
        .collect::<HashSet<_>>();

    let mut tracks = from
        .tracks
        .into_values()
        .into_iter()
        .filter(|track| !analyzed_ids.contains(&track.id))
        .collect::<Vec<_>>();

    // Add new (or modified) tracks
    tracks.extend(analyzed);

    log(
        started,
        &format!("Collected {} tracks, generating cache...", tracks.len()),
    );

    let tracks = SortedMap::from_vec(tracks, |track| track.id.clone());

    let cache = build_index_cache(&tracks);

    let new_albums = cache
        .albums_infos
        .values()
        .filter(|album| !from.cache.albums_infos.contains_key(&album.get_id()))
        .collect::<Vec<_>>();

    log(
        started,
        &format!("Searching for new albums' ({}) arts...", new_albums.len()),
    );

    let new_albums_arts = find_albums_arts(&new_albums, &dir, &tracks, &cache);

    let mut arts = from.arts;

    arts.extend(
        new_albums_arts
            .into_iter()
            .map(|art| (art.id, art))
            .collect::<HashMap<_, _>>(),
    );

    println!(
        "Generating artists' arts ({})...",
        cache.artists_infos.len()
    );

    arts.extend(
        cache
            .artists_infos
            .values()
            .filter_map(|artist| generate_artist_art(artist.get_id(), &arts, &cache))
            .map(|art| (art.id, art))
            .collect::<HashMap<_, _>>(),
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
        arts,
        cache,
    })
}

pub fn rebuild_cache(index: &mut Index) {
    index.cache = build_index_cache(&index.tracks);
}

pub fn rebuild_arts(index: &mut Index) {
    let arts = find_albums_arts(
        index
            .cache
            .albums_infos
            .values()
            .collect::<Vec<_>>()
            .as_slice(),
        &index.from,
        &index.tracks,
        &index.cache,
    );

    index.arts = arts.into_iter().map(|art| (art.id, art)).collect();
    index.arts.extend(
        index
            .cache
            .artists_infos
            .values()
            .filter_map(|artist| generate_artist_art(artist.get_id(), &index.arts, &index.cache))
            .map(|art| (art.id, art))
            .collect::<HashMap<_, _>>(),
    );
}

fn build_files_list(from: &Path) -> Result<HashMap<PathBuf, SystemTime>> {
    WalkDir::new(from)
        .min_depth(1)
        .into_iter()
        .par_bridge()
        .map(|item| {
            let item = item.context("Failed to read directory entry")?;

            if !item.path().is_file() {
                return Ok(None);
            }

            if item.path().to_str().is_none() {
                bail!(
                    "Item does not have a valid UTF-8 path: {}",
                    item.path().to_string_lossy()
                );
            }

            let metadata = item
                .metadata()
                .context("Failed to get the file's metadata")?;

            let mtime = metadata
                .modified()
                .context("Failed to get file's modification time")?;

            return Ok(Some((item.path().to_path_buf(), mtime)));
        })
        .filter_map(|entry| match entry {
            Ok(Some(entry)) => Some(Ok(entry)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        })
        .collect()
}
