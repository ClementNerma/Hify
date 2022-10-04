use std::{
    collections::HashMap,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

use anyhow::{bail, Context, Result};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::utils::progress::display_progress;

use super::{blurhash, AlbumID, IndexCache};

static COVER_FILENAMES: &[&str] = &["cover", "Cover", "folder", "Folder"];
static COVER_EXTENSIONS: &[&str] = &["jpg", "JPG", "jpeg", "JPEG", "png", "PNG"];

#[derive(Clone, Serialize, Deserialize)]
pub struct Art {
    pub path: PathBuf,
    pub blurhash: String,
}

pub fn find_albums_arts(
    album_ids: &[&AlbumID],
    cache: &IndexCache,
) -> Result<HashMap<AlbumID, Option<Art>>> {
    let started = Instant::now();

    let total = album_ids.len();
    let done = AtomicUsize::new(0);

    print!("        Starting...");

    album_ids
        .par_iter()
        .map(|id| find_album_art(id, cache).map(|art| (**id, art)))
        .inspect(|result| {
            let current = done.load(Ordering::Acquire) + 1;
            done.store(current, Ordering::Release);

            display_progress(started.elapsed().as_secs(), current, total);

            let album_id = match result {
                Ok((album_id, album_art)) if album_art.is_none() => album_id,
                _ => return,
            };

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
        })
        .collect()
}

fn find_album_art(album_id: &AlbumID, cache: &IndexCache) -> Result<Option<Art>> {
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
                    let art = make_art(art_path)?;
                    return Ok(Some(art));
                }
            }
        }
    }

    Ok(None)
}

fn make_art(path: PathBuf) -> Result<Art> {
    let img = image::open(&path).with_context(|| {
        format!(
            "Failed to open the image file at: {}",
            path.to_string_lossy()
        )
    })?;

    let bytes_count = img.as_bytes().len();
    let expected = usize::try_from(img.width() * img.height() * 3).unwrap();

    if bytes_count != expected {
        bail!("Invalid image bytes count (found {bytes_count} bytes, expected {expected} bytes)");
    }

    let blurhash = blurhash::encode(3, 3, img.width(), img.height(), img.as_bytes())?;

    Ok(Art { path, blurhash })
}
