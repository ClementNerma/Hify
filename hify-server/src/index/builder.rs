use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator,
};
use std::{
    collections::{hash_map::DefaultHasher, BTreeSet, HashMap, HashSet},
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};
use walkdir::WalkDir;

use super::{
    data::{AlbumID, AlbumInfos, ArtistID, ArtistInfos, Index, IndexCache, Track, TrackID},
    ffprobe,
    sorted_map::SortedMap,
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

    let counter = AtomicUsize::new(0);

    let analyzed = files
        .par_iter()
        .enumerate()
        .map(|(_, file)| ffprobe::run_on(&file.path))
        .inspect(|_| {
            let counter = counter.fetch_add(1, Ordering::SeqCst) + 1;
            if counter % 1000 == 0 || counter == files.len() {
                let progress_percent = counter as f64 * 100.0 / files.len() as f64;
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

    let tracks = SortedMap::from_vec(tracks, |track| track.id.clone());

    let cache = build_index_cache(&tracks, tracks_paths);

    log(started, "Searching for album arts...");

    let found_album_arts = AtomicUsize::new(0);

    let album_ids: Vec<_> = cache.albums_infos.keys().collect();
    let albums_arts: HashMap<_, _> = album_ids
        .par_iter()
        .map(|id| ((*id).clone(), find_album_art(id, &cache)))
        .inspect(|(album_id, art_path)| {
            if art_path.is_some() {
                found_album_arts.fetch_add(1, Ordering::SeqCst);
            } else {
                let album_infos = cache.albums_infos.get(album_id).unwrap();
                eprintln!(
                    "Warning: no album art found for album '{}' by '{}'",
                    album_infos.name,
                    album_infos
                        .album_artists
                        .iter()
                        .map(|artist| artist.name.clone())
                        .collect::<Vec<_>>()
                        .join(" / ")
                );
            }
        })
        .collect();

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

    Index {
        fingerprint,
        from,
        tracks,
        albums_arts,
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

static COVER_FILENAMES: &[&str] = &["cover", "Cover", "folder", "Folder"];
static COVER_EXTENSIONS: &[&str] = &["jpg", "JPG", "jpeg", "JPEG", "png", "PNG"];

fn find_album_art(album_id: &AlbumID, cache: &IndexCache) -> Option<PathBuf> {
    let album_tracks_ids = cache.albums_tracks.get(album_id).unwrap();

    // Cannot fail as albums need at least one track to be registered
    let first_track_id = album_tracks_ids.get(0).unwrap();

    let track_path = cache.tracks_paths.get(first_track_id).unwrap();

    for dir in track_path.ancestors() {
        for filename in COVER_FILENAMES {
            for extension in COVER_EXTENSIONS {
                let mut art_file = PathBuf::new();
                art_file.set_file_name(filename);
                art_file.set_extension(extension);

                let mut art_path = dir.to_path_buf();
                art_path.push(art_file);

                if art_path.is_file() {
                    return Some(art_path);
                }
            }
        }
    }

    None
}

// TODO: lots of optimization to perform here
fn build_index_cache(
    tracks: &SortedMap<TrackID, Track>,
    tracks_paths: HashMap<TrackID, PathBuf>,
) -> IndexCache {
    let mut tracks_formats = HashMap::new();

    let mut artists_albums = HashMap::<ArtistID, BTreeSet<AlbumInfos>>::new();
    let mut artists_tracks = HashMap::<ArtistID, Vec<TrackID>>::new();
    let mut albums_artists_albums = HashMap::<ArtistID, BTreeSet<AlbumInfos>>::new();
    let mut albums_tracks = HashMap::<AlbumID, Vec<TrackID>>::new();

    let mut artists_infos = HashMap::<ArtistID, ArtistInfos>::new();
    let mut albums_artists_infos = HashMap::<ArtistID, ArtistInfos>::new();
    let mut albums_infos = HashMap::<AlbumID, AlbumInfos>::new();

    let mut genres_tracks = HashMap::<String, Vec<TrackID>>::new();
    let mut no_genre_tracks = HashSet::new();

    for track in tracks.values() {
        tracks_formats.insert(track.id.clone(), track.metadata.format);

        let tags = &track.metadata.tags;

        let album_infos = tags.get_album_infos();
        let album_id = album_infos.get_id();

        albums_infos.insert(album_id.clone(), album_infos.clone());

        for album_artist_infos in tags.get_album_artists_infos() {
            albums_artists_infos.insert(album_artist_infos.get_id(), album_artist_infos.clone());

            albums_artists_albums
                .entry(album_artist_infos.get_id())
                .or_default()
                .insert(album_infos.clone());
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
                .insert(album_infos.clone());

            artists_tracks
                .entry(artist_id.clone())
                .or_default()
                .push(track.id.clone());
        }

        albums_tracks
            .entry(album_id.clone())
            .or_default()
            .push(track.id.clone());

        if track.metadata.tags.genres.is_empty() {
            no_genre_tracks.insert(track.id.clone());
        } else {
            for genre in &track.metadata.tags.genres {
                genres_tracks
                    .entry(genre.clone())
                    .or_default()
                    .push(track.id.clone());
            }
        }
    }

    let artists_albums = artists_albums
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                SortedMap::from_vec(v.into_iter().collect(), |album| album.get_id()),
            )
        })
        .collect();

    let albums_artists_albums = albums_artists_albums
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                SortedMap::from_vec(v.into_iter().collect(), |album| album.get_id()),
            )
        })
        .collect();

    let artists_tracks = artists_tracks
        .into_iter()
        .map(|(artist_id, mut artist_tracks)| {
            artist_tracks.sort_by(|a, b| tracks.get(a).unwrap().cmp(tracks.get(b).unwrap()));
            (artist_id, artist_tracks)
        })
        .collect();

    let albums_tracks = albums_tracks
        .into_iter()
        .map(|(album_id, mut album_tracks)| {
            album_tracks.sort_by(|a, b| tracks.get(a).unwrap().cmp(tracks.get(b).unwrap()));
            (album_id, album_tracks)
        })
        .collect();

    let genres_tracks = genres_tracks
        .into_iter()
        .map(|(genre_id, mut genre_tracks)| {
            genre_tracks.sort_by(|a, b| tracks.get(a).unwrap().cmp(tracks.get(b).unwrap()));
            (genre_id, genre_tracks)
        })
        .collect();

    IndexCache {
        tracks_paths,

        artists_albums,
        artists_tracks,

        albums_artists_albums,
        albums_tracks,

        artists_infos: SortedMap::from_hashmap(artists_infos),
        albums_artists_infos: SortedMap::from_hashmap(albums_artists_infos),
        albums_infos: SortedMap::from_hashmap(albums_infos),

        genres_tracks,
        no_genre_tracks,
    }
}
