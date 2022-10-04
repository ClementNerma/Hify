use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, AtomicUsize, Ordering},
        Mutex,
    },
    time::Instant,
};

use anyhow::{bail, Context, Result};
use color_thief::ColorFormat;
use image::EncodableLayout;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::utils::progress::display_progress;

use super::{
    blurhash, AlbumID, AlbumInfos, Art, ArtID, ArtRgb, ArtTarget, ArtistID, IndexCache, SortedMap,
    Track, TrackID,
};

static COVER_FILENAMES: &[&str] = &["cover", "Cover", "folder", "Folder"];
static COVER_EXTENSIONS: &[&str] = &["jpg", "JPG", "jpeg", "JPEG", "png", "PNG"];

pub fn generate_artist_art(
    artist_id: ArtistID,
    arts: &HashMap<ArtID, Art>,
    cache: &IndexCache,
) -> Option<Art> {
    let albums = cache.artists_albums.get(&artist_id)?;

    let first_album = albums.values().next()?;

    let album_art = arts.get(&ArtTarget::AlbumCover(first_album.get_id()).to_id())?;

    let target = ArtTarget::Artist(artist_id);

    Some(Art {
        id: target.to_id(),
        target,
        ..album_art.clone()
    })
}

pub fn find_albums_arts(
    albums: &[&AlbumInfos],
    base_dir: &Path,
    tracks: &SortedMap<TrackID, Track>,
    cache: &IndexCache,
) -> Vec<Art> {
    let started = Instant::now();

    let total = albums.len();
    let done = AtomicUsize::new(0);

    let errors = Mutex::new(vec![]);
    let errors_count = AtomicU64::new(0);

    print!("        Starting...");

    let result = albums
        .par_iter()
        .filter_map(|album| {
            let result = find_album_art(album.get_id(), base_dir, tracks, cache);

            let current = done.load(Ordering::Acquire) + 1;
            done.store(current, Ordering::Release);

            display_progress(
                started.elapsed().as_secs(),
                current,
                total,
                errors_count.load(Ordering::Acquire),
            );

            match result {
                Ok(None) => {}
                Ok(Some(art)) => return Some(art),
                Err(err) => {
                    errors.lock().unwrap().push(format!("{:?}", err));
                    errors_count.store(errors_count.load(Ordering::Acquire) + 1, Ordering::Release);
                    return None;
                }
            }

            eprintln!(
                "Warning: no album art found for album '{}' by '{}'",
                album.name,
                album
                    .album_artists
                    .iter()
                    .map(|artist| artist.name.clone())
                    .collect::<Vec<_>>()
                    .join(" / ")
            );

            None
        })
        .collect::<Vec<_>>();

    println!();

    let errors = errors.lock().unwrap();

    if !errors.is_empty() {
        for (i, err) in errors.iter().enumerate() {
            eprintln!(
                "| Art error {} / {}: {}",
                i + 1,
                errors.len(),
                err.lines().collect::<Vec<_>>().join("\\n")
            );
        }
    }

    result
}

fn find_album_art(
    album_id: AlbumID,
    base_dir: &Path,
    tracks: &SortedMap<TrackID, Track>,
    cache: &IndexCache,
) -> Result<Option<Art>> {
    let album_tracks_ids = cache.albums_tracks.get(&album_id).unwrap();

    // Cannot fail as albums need at least one track to be registered
    let first_track_id = album_tracks_ids.get(0).unwrap();

    let track_path = base_dir.join(&tracks.get(first_track_id).unwrap().relative_path);

    for dir in track_path.ancestors() {
        for filename in COVER_FILENAMES {
            for extension in COVER_EXTENSIONS {
                let mut art_file = PathBuf::new();
                art_file.set_file_name(filename);
                art_file.set_extension(extension);

                let mut art_path = dir.to_path_buf();
                art_path.push(art_file);

                if art_path.is_file() {
                    let art = make_album_art(&art_path, base_dir, album_id).with_context(|| {
                        format!(
                            "Failed to make art for album cover at: {}",
                            art_path.to_string_lossy()
                        )
                    })?;

                    return Ok(Some(art));
                }
            }
        }
    }

    Ok(None)
}

fn make_album_art(path: &Path, base_dir: &Path, album_id: AlbumID) -> Result<Art> {
    let mut img = image::open(path).context("Failed to open the image file")?;

    let img = img
        .as_mut_rgb8()
        .context("Failed to get an RGB8 image from the album cover")?;

    let bytes_count = img.as_bytes().len();
    let expected = usize::try_from(img.width() * img.height() * 3).unwrap();

    if bytes_count != expected {
        bail!("Invalid image bytes count (found {bytes_count} bytes, expected {expected} bytes)");
    }

    let blurhash = blurhash::encode(9, 9, img.width(), img.height(), img.as_bytes())?;

    let dominant_color = color_thief::get_palette(img.as_bytes(), ColorFormat::Rgb, 10, 2)?;

    if dominant_color.len() != 2 {
        bail!("Color Thief did not return exactly one color");
    }

    let dominant_color = dominant_color[0];

    let relative_path = path
        .strip_prefix(base_dir)
        .expect("Internal error: art path couldn't be stripped of the base directory")
        .to_path_buf();

    let target = ArtTarget::AlbumCover(album_id);

    Ok(Art {
        relative_path,
        target,

        id: target.to_id(),

        width: img.width(),
        height: img.height(),

        blurhash,
        dominant_color: ArtRgb {
            r: dominant_color.r,
            g: dominant_color.g,
            b: dominant_color.b,
        },
    })
}
