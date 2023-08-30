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

use crate::utils::logging::{progress_bar, spinner};

use super::{
    arts::{find_albums_arts, generate_artist_art},
    cache::build_index_cache,
    data::{Index, Track},
    metadata,
    sorted_map::SortedMap,
    IndexCache,
};

pub fn log(time: Instant, message: &str) {
    info!("[{: >4}s] {message}", time.elapsed().as_secs());
}

pub async fn build_index(dir: PathBuf, from: Option<Index>) -> Result<Index> {
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

            most_recent_albums: vec![],
        },
    });

    let started = Instant::now();

    log(started, "Looking for audio files...");

    let files = build_files_list(&dir)
        .await
        .context("Failed to build files list")?;

    log(started, &format!("Found a total of {} files.", files.len()));

    let files = files
        .into_iter()
        .filter(|(path, _)| metadata::is_audio_file(path))
        .collect::<Vec<_>>();

    log(
        started,
        &format!("...of which {} are audio files.", files.len()),
    );

    let file_times = files
        .into_iter()
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
    let analyzed = metadata::run_on(file_times.keys().cloned().collect::<Vec<_>>()).await?;

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

    let mut tracks = filter_out_deleted_tracks(tracks, &dir).await?;

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

    let tracks = SortedMap::from_vec(tracks, |track| track.id);

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

    let new_albums_arts =
        find_albums_arts(&new_albums, &dir, tracks.clone(), cache.clone()).await?;

    let mut arts = from.arts;

    arts.extend(
        new_albums_arts
            .into_iter()
            .map(|art| (art.id, art))
            .collect::<HashMap<_, _>>(),
    );

    info!(
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

pub async fn rebuild_arts(index: &mut Index) -> Result<()> {
    let arts = find_albums_arts(
        index
            .cache
            .albums_infos
            .values()
            .collect::<Vec<_>>()
            .as_slice(),
        &index.from,
        index.tracks.clone(),
        index.cache.clone(),
    )
    .await?;

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
    let spinner = spinner("Analyzed {pos} files");

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

async fn filter_out_deleted_tracks(tracks: Vec<Track>, base_dir: &Path) -> Result<Vec<Track>> {
    let pb = progress_bar(tracks.len());

    let mut set: JoinSet<Result<Option<Track>>> = JoinSet::new();

    for track in tracks {
        let track = track.clone();
        let base_dir = base_dir.to_path_buf();
        let pb = pb.clone();

        set.spawn(async move {
            let mt = base_dir
                .join(&track.relative_path)
                .metadata()
                .with_context(|| {
                    format!(
                        "Failed to get metadata for file at path: {}",
                        track.relative_path.display()
                    )
                })?;

            pb.inc(1);

            if mt.is_file() {
                Ok(Some(track))
            } else {
                Ok(None)
            }
        });
    }

    let mut out = vec![];

    while let Some(res) = set.join_next().await {
        if let Some(res) = res?? {
            out.push(res);
        }
    }

    Ok(out)
}
