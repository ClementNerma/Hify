use anyhow::{Context, Result};
use log::info;
use tokio::task::JoinSet;

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::Metadata,
    path::{Path, PathBuf},
    time::{Instant, SystemTime, UNIX_EPOCH},
};
use walkdir::WalkDir;

use crate::{
    helpers::logging::spinner,
    index::arts::{find_albums_arts, generate_artists_art},
    resources::ResourceManager,
};

use super::{
    arts::detect_deleted_arts,
    cache::build_index_cache,
    data::{Index, Track},
    metadata,
    sorted_map::SortedMap,
    IndexCache,
};

pub fn log(time: Instant, message: &str) {
    info!("[{: >4}s] {message}", time.elapsed().as_secs());
}

pub async fn build_index(
    dir: PathBuf,
    from: Option<Index>,
    res_manager: &ResourceManager,
) -> Result<Index> {
    let from = from.unwrap_or_else(|| Index {
        from: dir.clone(),

        fingerprint: String::new(),
        tracks: SortedMap::empty(),

        album_arts: HashMap::new(),

        cache: IndexCache {
            tracks_files_mtime: HashMap::new(),
            tracks_all_artists: HashMap::new(),

            artists_albums: HashMap::new(),
            artists_album_participations: HashMap::new(),
            artists_albums_and_participations: HashMap::new(),
            artists_tracks: HashMap::new(),
            artists_track_participations: HashMap::new(),

            artists_mean_score: HashMap::new(),
            album_artists_mean_score: HashMap::new(),

            albums_tracks: HashMap::new(),
            albums_mean_score: HashMap::new(),

            artists_infos: SortedMap::empty(),
            album_artists_infos: SortedMap::empty(),
            albums_infos: SortedMap::empty(),
            genre_infos: SortedMap::empty(),

            genres_albums: HashMap::new(),
            genres_tracks: HashMap::new(),
            no_genre_tracks: HashSet::new(),

            most_recent_albums: vec![],
        },
    });

    let started = Instant::now();

    log(started, "Looking for audio files...");

    let files = build_files_list(&dir)
        .await
        .context("Failed to build files list")?;

    log(started, &format!("Found a total of {} files", files.len()));

    let files = files
        .into_iter()
        .filter(|(path, _)| metadata::is_audio_file(path))
        .collect::<HashMap<_, _>>();

    log(
        started,
        &format!("...of which {} are audio files.", files.len()),
    );

    let file_times = files
        .iter()
        .map(|(path, times)| (path.clone(), *times))
        .filter(|(path, _)| metadata::is_audio_file(path))
        .filter(|(path, times)| {
            match from
                .cache
                .tracks_files_mtime
                .get(path.strip_prefix(&dir).expect(
                    "Internal error: audio file path couldn't be stripped of the base directory",
                )) {
                None => true,
                Some(old_mtime) => old_mtime != &times.mtime,
            }
        })
        .collect::<BTreeMap<_, _>>();

    log(
        started,
        &format!("...of which {} are new or modified.", file_times.len()),
    );

    log(started, "Extracting audio metadata...");

    // Run analysis tool on all new and modified files
    let analyzed =
        metadata::analyze_audio_files(file_times.keys().cloned().collect::<Vec<_>>()).await?;

    // Turn the analyzed files into tracks
    let analyzed = analyzed
        .into_iter()
        .map(|(path, metadata)| {
            Track::new(
                path.strip_prefix(&dir)
                    .expect("Internal error: track path couldn't be stripped of the base directory")
                    .to_path_buf(),
                *file_times.get(&path).unwrap(),
                metadata,
            )
        })
        .collect::<Vec<_>>();

    // Remove previous versions of analyzed files
    let analyzed_ids = analyzed
        .iter()
        .map(|track| &track.id)
        .collect::<HashSet<_>>();

    let tracks = from
        .tracks
        .into_values()
        .into_iter()
        .filter(|track| !analyzed_ids.contains(&track.id))
        .collect::<Vec<_>>();

    // Remove deleted tracks
    let before_deletion_count = tracks.len();

    log(started, "Looking for deleted track(s)...");

    let mut tracks = tracks
        .into_iter()
        .filter(|track| files.contains_key(&dir.join(&track.relative_path)))
        .collect::<Vec<_>>();

    let deleted_count = before_deletion_count - tracks.len();

    if deleted_count > 0 {
        log(
            started,
            &format!("Detected {deleted_count} deleted track(s)."),
        );
    }

    // Add new (or modified) tracks
    tracks.extend(analyzed);

    log(
        started,
        &format!("Collected {} tracks, generating cache...", tracks.len()),
    );

    let tracks = tracks.into_iter().map(|track| (track.id, track)).collect();

    let cache = build_index_cache(&tracks);

    let new_albums = cache
        .albums_infos
        .values()
        .filter(|album| !from.cache.albums_infos.contains_key(&album.get_id()))
        .cloned()
        .collect::<Vec<_>>();

    let mut album_arts = from
        .album_arts
        .into_iter()
        .filter(|(id, _)| cache.albums_infos.contains_key(id))
        .collect::<HashMap<_, _>>();

    log(
        started,
        &format!(
            "Searching for existing albums' ({}) arts...",
            album_arts.len()
        ),
    );

    // TODO: remove deleted albums from this
    let deleted_arts = detect_deleted_arts(&dir, &album_arts).await?;

    if !deleted_arts.is_empty() {
        log(
            started,
            &format!("> Detected {} deleted arts", deleted_arts.len()),
        );
    }

    let mut search_for_album_arts = new_albums;
    search_for_album_arts.extend(
        deleted_arts
            .iter()
            .map(|id| from.cache.albums_infos.get(id).unwrap())
            .cloned(),
    );

    if !search_for_album_arts.is_empty() {
        log(
            started,
            &format!(
                "Searching for {} albums' art...",
                search_for_album_arts.len()
            ),
        );

        // Cleanup deleted arts
        album_arts.retain(|album_id, _| cache.albums_infos.contains_key(album_id));

        // Detect art for new albums
        let new_albums_arts = find_albums_arts(
            search_for_album_arts.iter().cloned(),
            &dir,
            tracks.clone(),
            cache.clone(),
        )
        .await?;

        album_arts.extend(new_albums_arts);
    }

    // Generate art for artists
    let artists_new_or_diff_albums = cache
        .artists_albums_and_participations
        .iter()
        .filter(|(artist_id, albums)| {
            from.cache
                .artists_albums_and_participations
                .get(artist_id)
                .is_none_or(|old_albums| {
                    // NOTE: we don't check if albums have been removed at the end, as only the first ones are necessary for generation
                    let mut old_albums = old_albums.keys();
                    albums.keys().any(|id| old_albums.next() != Some(id))
                })
        })
        .map(|(artist_id, _)| cache.artists_infos.get(artist_id).unwrap())
        .collect::<Vec<_>>();

    if !artists_new_or_diff_albums.is_empty() {
        info!(
            "Generating art for {} artist(s)...",
            artists_new_or_diff_albums.len(),
        );

        generate_artists_art(
            artists_new_or_diff_albums.into_iter(),
            &dir,
            &album_arts,
            cache.clone(),
            res_manager.clone(),
        )?;
    }

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
        album_arts,
        cache,
    })
}

pub fn rebuild_cache(index: &mut Index) {
    index.cache = build_index_cache(&index.tracks);
}

pub async fn rebuild_resources(index: &mut Index, res_manager: &ResourceManager) -> Result<()> {
    info!(
        "Detecting cover art for {} albums...",
        index.cache.albums_infos.len()
    );

    let album_arts = find_albums_arts(
        index.cache.albums_infos.values().cloned(),
        &index.from,
        index.tracks.clone(),
        index.cache.clone(),
    )
    .await?;

    index.album_arts = album_arts.into_iter().collect();

    info!(
        "Generating cover art for {} artists...",
        index.cache.artists_infos.len()
    );

    generate_artists_art(
        index.cache.artists_infos.values(),
        &index.from,
        &index.album_arts,
        index.cache.clone(),
        res_manager.clone(),
    )?;

    Ok(())
}

pub fn refetch_file_times(index: &mut Index) -> Result<()> {
    for track in index.tracks.values_mut() {
        let mt = index
            .from
            .join(&track.relative_path)
            .metadata()
            .with_context(|| {
                format!(
                    "Failed to get file metadata from track path: {}",
                    track.relative_path.display()
                )
            })?;

        let FileTimes { ctime, mtime } = get_file_times(&mt)?;

        track.ctime = ctime;
        track.mtime = mtime;
    }

    Ok(())
}

#[derive(Clone, Copy)]
pub struct FileTimes {
    pub ctime: Option<SystemTime>,
    pub mtime: SystemTime,
}

async fn build_files_list(from: &Path) -> Result<HashMap<PathBuf, FileTimes>> {
    let spinner = spinner("[{elapsed_precise}] Found {pos} files");

    let mut set: JoinSet<Result<Option<(PathBuf, FileTimes)>>> = JoinSet::new();

    for item in WalkDir::new(from).min_depth(1) {
        let spinner = spinner.clone();

        set.spawn(async move {
            let item = item.context("Failed to read directory entry")?;
            let mt = item.metadata().with_context(|| {
                format!("Failed to get metadata for path: {}", item.path().display())
            })?;

            if !mt.is_file() {
                return Ok(None);
            }

            spinner.inc(1);
            Ok(Some((item.path().to_path_buf(), get_file_times(&mt)?)))
        });
    }

    let mut files = HashMap::new();

    while let Some(res) = set.join_next().await {
        if let Some((path, times)) = res?? {
            files.insert(path, times);
        }
    }

    spinner.finish_and_clear();

    Ok(files)
}

fn get_file_times(mt: &Metadata) -> Result<FileTimes> {
    let ctime = mt.created().ok();

    let mtime = mt
        .modified()
        .context("Failed to get the file's modification time")?;

    Ok(FileTimes { ctime, mtime })
}
