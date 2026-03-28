use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use colored::Colorize;
use log::{debug, info, warn};
use walkdir::WalkDir;

use crate::{
    index::{Album, Artist, FileTimes, Genre, Index, IndexCache, Track, TrackID, TrackTags},
    utils::TaskRunner,
};

use self::{
    tags::TrackStrTags,
    walker::{analyze_audio_files, may_be_audio_file},
};

mod analyzer;
mod tags;
mod walker;

#[allow(clippy::too_many_lines)]
pub fn analyze_tracks_in(dir: &Path, prev_index: Option<&IndexCache>) -> Result<Option<Index>> {
    debug!("-> Building files list...");

    let files = build_files_list(dir)?;

    let empty_cache = IndexCache::default();
    let prev_index = prev_index.unwrap_or(&empty_cache);

    let TracksChanges {
        new: new_tracks,
        modified: modified_tracks,
        deleted: deleted_tracks,
        unchanged: unchanged_tracks,
    } = compute_changes_in(&files, prev_index);

    let total_tracks = new_tracks.len() + modified_tracks.len() + unchanged_tracks.len();

    if new_tracks.is_empty() && modified_tracks.is_empty() && deleted_tracks.is_empty() {
        info!("-> No changes detected, skipping analysis.");
        return Ok(None);
    }

    info!(
        "-> Found a total of {} tracks...",
        total_tracks.to_string().bright_yellow()
    );

    if !new_tracks.is_empty() {
        info!(
            "--> ...of which {} are new",
            new_tracks.len().to_string().bright_green()
        );
    }

    if !modified_tracks.is_empty() {
        info!(
            "--> ...of which {} have been modified",
            modified_tracks.len().to_string().bright_yellow()
        );
    }

    if !deleted_tracks.is_empty() {
        info!(
            "--> ...plus {} deleted tracks",
            deleted_tracks.len().to_string().bright_red()
        );
    }

    info!(
        "-> Analyzing {} audio files...",
        (new_tracks.len() + modified_tracks.len())
            .to_string()
            .bright_yellow()
    );

    let analyzed =
        analyze_audio_files(new_tracks.into_iter().chain(modified_tracks).cloned(), dir)?;

    info!("--> Building new index...");

    let prev_tracks_by_path = prev_index
        .tracks
        .values()
        .map(|track| (&track.relative_path, track))
        .collect::<HashMap<_, _>>();

    let mut index_tracks = unchanged_tracks
        .iter()
        .map(|path| (*prev_tracks_by_path.get(path).unwrap()).clone())
        .collect::<Vec<_>>();

    let mut index_albums = index_tracks
        .iter()
        .map(|track| {
            let album = prev_index.albums.get(&track.tags.album_id).unwrap();
            (album.id, album.clone())
        })
        .collect::<HashMap<_, _>>();

    let mut index_artists = HashMap::new();

    for track_path in unchanged_tracks {
        let track = prev_tracks_by_path.get(track_path).unwrap();
        let album = prev_index.albums.get(&track.tags.album_id).unwrap();

        for artist_id in track
            .tags
            .artists_id
            .iter()
            .chain(track.tags.composers_id.iter())
            .chain(album.artists_id.iter())
        {
            let artist = prev_index.artists.get(artist_id).unwrap();

            if !index_artists.contains_key(&artist.name) {
                index_artists.insert(&artist.name, artist.clone());
            }
        }
    }

    for artist_id in index_tracks.iter().flat_map(|track| {
        track
            .tags
            .artists_id
            .iter()
            .chain(track.tags.composers_id.iter())
    }) {
        let artist = prev_index.artists.get(artist_id).unwrap();

        if !index_artists.contains_key(&artist.name) {
            index_artists.insert(&artist.name, artist.clone());
        }
    }

    let mut index_genres = HashMap::new();

    for genre_id in index_tracks
        .iter()
        .flat_map(|track| track.tags.genres_id.iter())
    {
        let genre = prev_index.genres.get(genre_id).unwrap();

        if !index_genres.contains_key(&genre.name) {
            index_genres.insert(&genre.name, genre.clone());
        }
    }

    for (relative_path, (metadata, str_tags)) in &analyzed {
        debug_assert!(
            !index_tracks
                .iter()
                .any(|track| &track.relative_path == relative_path)
        );

        let FileTimesWithSize {
            file_times,
            file_size_bytes,
        } = *files.get(relative_path).unwrap();

        let TrackStrTags {
            title,
            artists,
            composers,
            album,
            album_artists,
            disc,
            track_no,
            date,
            genres,
        } = str_tags;

        for artist_name in artists
            .iter()
            .chain(album_artists.iter())
            .chain(composers.iter())
        {
            let artist = Artist::new(artist_name.clone());

            index_artists.entry(artist_name).or_insert(artist);
        }

        for genre in genres {
            if !index_genres.contains_key(genre) {
                index_genres.insert(genre, Genre::new(genre.clone()));
            }
        }

        let album = Album::new(
            album.clone(),
            album_artists
                .iter()
                .map(|name| index_artists.get(name).unwrap().id)
                .collect(),
        );

        index_tracks.push(Track {
            id: TrackID::compute(relative_path),
            relative_path: relative_path.to_owned(),
            file_size_bytes,
            file_times,
            metadata: *metadata,
            tags: TrackTags {
                title: title.clone(),
                artists_id: artists
                    .iter()
                    .map(|name| index_artists.get(name).unwrap().id)
                    .collect(),
                composers_id: composers
                    .iter()
                    .map(|name| index_artists.get(name).unwrap().id)
                    .collect(),
                album_id: album.id,
                disc_number: *disc,
                track_number: *track_no,
                date: *date,
                genres_id: genres
                    .iter()
                    .map(|name| Genre::new(name.clone()).id)
                    .collect(),
            },
        });

        index_albums.insert(album.id, album);
    }

    assert_eq!(index_tracks.len(), total_tracks);

    Ok(Some(Index {
        tracks: index_tracks,
        albums: index_albums.into_values().collect(),
        artists: index_artists.into_values().collect(),
        genres: index_genres.into_values().collect(),
    }))
}

fn build_files_list(dir: &Path) -> Result<HashMap<PathBuf, FileTimesWithSize>> {
    let dir_bis = dir.to_owned();

    let mut tasks = TaskRunner::<Option<(PathBuf, FileTimesWithSize)>>::new();

    for item in WalkDir::new(&dir_bis).min_depth(1) {
        let item = item.context("Failed to read music directory entry")?;
        let item = item.path().to_owned();

        if !may_be_audio_file(&item) {
            continue;
        }

        let dir = dir.to_owned();

        tasks.spawn(move || {
            let mt = fs::metadata(&item)
                .with_context(|| format!("Failed to get metadata for path: {}", item.display()))?;

            if mt.is_dir() {
                return Ok(None);
            } else if !mt.is_file() {
                bail!("Unsupported filesystem item: {}", item.display());
            }

            let file_times = FileTimes {
                ctime: mt.created().ok(),
                mtime: mt.modified().with_context(|| {
                    format!("Failed to get file's modification time: {}", item.display())
                })?,
            };

            Ok(Some((
                item.strip_prefix(&dir).unwrap().to_owned(),
                FileTimesWithSize {
                    file_times,
                    file_size_bytes: mt.len(),
                },
            )))
        });
    }

    let files = tasks.join_all()?;

    Ok(files.into_iter().flatten().collect())
}

#[derive(Debug, Clone, Copy)]
struct FileTimesWithSize {
    file_times: FileTimes,
    file_size_bytes: u64,
}

fn compute_changes_in<'a>(
    files: &'a HashMap<PathBuf, FileTimesWithSize>,
    prev: &'a IndexCache,
) -> TracksChanges<'a> {
    let prev_tracks_by_path = prev
        .tracks
        .values()
        .map(|track| (&track.relative_path, track))
        .collect::<HashMap<_, _>>();

    let mut new_tracks = vec![];
    let mut modified_tracks = vec![];
    let mut unchanged_tracks = vec![];

    for (path, times) in files {
        match prev_tracks_by_path.get(&path) {
            Some(prev_track) => {
                if prev_track.file_times.mtime != times.file_times.mtime {
                    // Modified track
                    modified_tracks.push(path);
                } else if prev_track.file_times.ctime != times.file_times.ctime {
                    warn!(
                        "Creation time changed for track with same modification time: {}",
                        path.display()
                    );

                    modified_tracks.push(path);
                } else {
                    unchanged_tracks.push(path);
                }
            }

            None => {
                // New track
                new_tracks.push(path);
            }
        }
    }

    let mut deleted_tracks = prev_tracks_by_path
        .values()
        .map(|track| &track.relative_path)
        .filter(|path| !files.contains_key(*path))
        .collect::<Vec<_>>();

    new_tracks.sort();
    modified_tracks.sort();
    unchanged_tracks.sort();
    deleted_tracks.sort();

    TracksChanges {
        new: new_tracks,
        modified: modified_tracks,
        deleted: deleted_tracks,
        unchanged: unchanged_tracks,
    }
}

struct TracksChanges<'a> {
    new: Vec<&'a PathBuf>,
    modified: Vec<&'a PathBuf>,
    deleted: Vec<&'a PathBuf>,
    unchanged: Vec<&'a PathBuf>,
}
