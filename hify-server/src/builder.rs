use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator,
};
use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use walkdir::WalkDir;

use crate::index::{AlbumID, Index, Track, TrackID};
use crate::{ffprobe, index::IndexCache};

pub fn build_index(from: &Path) -> Index {
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
    let mut tracks_paths = HashMap::new();

    for (i, track_metadata) in analyzed.into_iter().enumerate() {
        let FoundFile { path, path_str } = &files.get(i).unwrap();

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
        let id = TrackID(hasher.finish().to_string());

        tracks_paths.insert(id.clone(), path.clone());

        tracks.push(Track {
            id,
            path: path_str.clone(),
            metadata: track_metadata,
        });
    }

    Index {
        fingerprint: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap() // cannot fail as it would imply SystemTime::now() returns a time *earlier* than UNIX_EPOCH
            .as_secs()
            .to_string(),
        cache: build_index_cache(&tracks, tracks_paths),
        tracks,
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

// TODO: lots of optimization to perform here
fn build_index_cache(tracks: &[Track], tracks_paths: HashMap<TrackID, PathBuf>) -> IndexCache {
    println!("Generating cache...");
    let mut no_title_tracks = HashSet::new();
    let mut no_album_tracks = HashSet::new();
    let mut no_album_artist_tracks = HashSet::new();

    let mut artists_albums = HashMap::<String, HashSet<AlbumID>>::new();
    let mut artists_tracks = HashMap::<String, HashSet<TrackID>>::new();
    let mut album_artists_albums = HashMap::<String, HashSet<AlbumID>>::new();
    let mut album_tracks = HashMap::<AlbumID, HashSet<TrackID>>::new();

    for track in tracks {
        let tags = &track.metadata.tags;

        if tags.title.is_none() {
            no_title_tracks.insert(track.id.clone());
        }

        if tags.album.is_none() {
            no_album_tracks.insert(track.id.clone());
        }

        if tags.album_artist.is_none() {
            no_album_artist_tracks.insert(track.id.clone());
        }

        if let Some(album_id) = tags.get_album_id() {
            if let Some(artist) = tags.artist.as_ref().or_else(|| tags.album_artist.as_ref()) {
                artists_albums
                    .entry(artist.clone())
                    .or_default()
                    .insert(album_id);

                artists_tracks
                    .entry(artist.clone())
                    .or_default()
                    .insert(track.id.clone());
            }

            if let Some(ref album_artist) = tags.album_artist {
                if let Some(album_id) = tags.get_album_id() {
                    album_artists_albums
                        .entry(album_artist.clone())
                        .or_default()
                        .insert(album_id.clone());
                }
            }

            if let Some(album_id) = tags.get_album_id() {
                album_tracks
                    .entry(album_id.clone())
                    .or_default()
                    .insert(track.id.clone());
            }
        }
    }

    IndexCache {
        tracks_paths,
        no_title_tracks,
        no_album_tracks,
        no_album_artist_tracks,
        artists_albums,
        artists_tracks,
        album_artists_albums,
        album_tracks,
    }
}
