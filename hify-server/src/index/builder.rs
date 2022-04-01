use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator,
};
use std::{
    collections::{hash_map::DefaultHasher, BTreeSet, HashMap},
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU32, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};
use walkdir::WalkDir;

use super::{
    data::{AlbumID, AlbumInfos, ArtistID, ArtistInfos, Index, IndexCache, Track, TrackID},
    ffprobe,
};

fn log(time: SystemTime, message: &str) {
    let elapsed = match time.elapsed() {
        Ok(time) => time.as_secs().to_string(),
        Err(_) => "?".to_string(),
    };

    println!("[{: >4}s] {message}", elapsed);
}

pub fn build_index(from: PathBuf) -> Index {
    let started = SystemTime::now();

    log(started, "Starting index building...");

    let mut files = vec![];
    let mut observations = vec![];

    for file in build_files_list(&from) {
        match file {
            Ok(file) => files.push(file),
            Err(err) => observations.push(err),
        }
    }

    files.sort();

    log(
        started,
        &format!("Found {} files, analyzing with FFProbe...", files.len()),
    );

    let counter = AtomicU32::new(0);

    let analyzed = files
        .par_iter()
        .enumerate()
        .map(|(_, file)| ffprobe::run_on(&file.path))
        .inspect(|_| {
            let counter = counter.fetch_add(1, Ordering::SeqCst);
            if counter % 1000 == 0 {
                let progress_percent = f64::from(counter * 100) / files.len() as f64;
                log(
                    started,
                    &format!(
                        "Index building progress: {:.1}% ({counter}/{} files)",
                        progress_percent,
                        files.len(),
                    ),
                );
            }
        })
        .collect::<Vec<_>>();

    log(started, "Collecting tracks...");

    let mut tracks = vec![];
    let mut tracks_paths = HashMap::new();

    for (i, track_metadata) in analyzed.into_iter().enumerate() {
        let FoundFile { path, path_str } = &files.get(i).unwrap();

        let track_metadata = match track_metadata {
            Ok(None) => continue,
            Ok(Some(mt)) => mt,
            Err(err) => {
                let err = format!("Error while analyzing file '{path_str}': {err}");
                eprintln!("{err}");
                observations.push(err);
                continue;
            }
        };

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

    let cache = build_index_cache(&tracks, tracks_paths);

    log(started, "Index has been generated.");

    Index {
        fingerprint: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap() // cannot fail as it would imply SystemTime::now() returns a time *earlier* than UNIX_EPOCH
            .as_secs()
            .to_string(),
        from,
        tracks,
        cache,
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
    let mut tracks_formats = HashMap::new();
    let mut tracks_index = HashMap::new();

    let mut no_title_tracks = BTreeSet::new();
    let mut no_album_tracks = BTreeSet::new();
    let mut no_album_artist_tracks = BTreeSet::new();

    let mut artists_albums = HashMap::<ArtistID, BTreeSet<AlbumID>>::new();
    let mut artists_tracks = HashMap::<ArtistID, BTreeSet<TrackID>>::new();
    let mut albums_artists_albums = HashMap::<ArtistID, BTreeSet<AlbumID>>::new();
    let mut albums_tracks = HashMap::<AlbumID, BTreeSet<TrackID>>::new();

    let mut artists_infos = HashMap::<ArtistID, ArtistInfos>::new();
    let mut albums_artists_infos = HashMap::<ArtistID, ArtistInfos>::new();
    let mut albums_infos = HashMap::<AlbumID, AlbumInfos>::new();

    for (i, track) in tracks.iter().enumerate() {
        tracks_formats.insert(track.id.clone(), track.metadata.format);
        tracks_index.insert(track.id.clone(), i);

        let tags = &track.metadata.tags;

        if tags.title.is_none() {
            no_title_tracks.insert(track.id.clone());
        }

        if tags.album.is_none() {
            no_album_tracks.insert(track.id.clone());
        }

        if tags.album_artists.is_empty() {
            no_album_artist_tracks.insert(track.id.clone());
        }

        if let Some(album_infos) = tags.get_album_infos() {
            let album_id = album_infos.get_id();

            albums_infos.insert(album_id.clone(), album_infos);

            for album_artist_infos in tags.get_album_artists_infos() {
                albums_artists_infos
                    .insert(album_artist_infos.get_id(), album_artist_infos.clone());

                albums_artists_albums
                    .entry(album_artist_infos.get_id())
                    .or_default()
                    .insert(album_id.clone());
            }

            for artist_infos in tags
                .get_album_artists_infos()
                .chain(tags.get_artists_infos())
            {
                let artist_id = artist_infos.get_id();

                artists_infos.insert(artist_id.clone(), artist_infos.clone());

                artists_albums
                    .entry(artist_id.clone())
                    .or_default()
                    .insert(album_id.clone());

                artists_tracks
                    .entry(artist_id.clone())
                    .or_default()
                    .insert(track.id.clone());
            }

            albums_tracks
                .entry(album_id.clone())
                .or_default()
                .insert(track.id.clone());
        }
    }

    let mut ordered_artists = artists_infos.iter().collect::<Vec<_>>();
    ordered_artists.sort_by(|a, b| a.1.name.cmp(&b.1.name));
    let ordered_artists = ordered_artists
        .into_iter()
        .map(|(id, _)| id.clone())
        .collect();

    let mut ordered_albums_artists = albums_artists_infos.iter().collect::<Vec<_>>();
    ordered_albums_artists.sort_by(|a, b| a.1.name.cmp(&b.1.name));
    let ordered_albums_artists = ordered_albums_artists
        .into_iter()
        .map(|(id, _)| id.clone())
        .collect();

    let mut ordered_albums = albums_infos.iter().collect::<Vec<_>>();
    ordered_albums.sort_by(|a, b| a.1.name.cmp(&b.1.name));
    let ordered_albums = ordered_albums
        .into_iter()
        .map(|(id, _)| id.clone())
        .collect();

    IndexCache {
        tracks_paths,
        tracks_index,
        tracks_formats,

        no_title_tracks,
        no_album_tracks,
        no_album_artist_tracks,

        artists_albums,
        artists_tracks,

        albums_artists_albums,
        albums_tracks,

        artists_infos,
        albums_infos,

        ordered_artists,
        ordered_albums_artists,
        ordered_albums,
    }
}
