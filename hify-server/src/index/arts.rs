use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
};

use anyhow::{Context, Result};
use log::error;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::utils::logging::files_progress_bar;

use super::{
    AlbumID, AlbumInfos, Art, ArtID, ArtTarget, ArtistID, IndexCache, SortedMap, Track, TrackID,
};

static COVER_FILENAMES: &[&str] = &["cover", "folder"];
static COVER_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png"];

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
    let done = AtomicUsize::new(0);

    let errors = Mutex::new(vec![]);

    let pb = files_progress_bar(albums.len());

    let result = albums
        .par_iter()
        .filter_map(|album| {
            let result = find_album_art(album.get_id(), base_dir, tracks, cache);

            let current = done.load(Ordering::Acquire) + 1;
            done.store(current, Ordering::Release);

            pb.inc(1);

            match result {
                Ok(None) => {}
                Ok(Some(art)) => return Some(art),
                Err(err) => {
                    errors.lock().unwrap().push(format!("{:?}", err));
                    return None;
                }
            }

            pb.suspend(|| {
                error!(
                    "Warning: no album art found for album '{}' by '{}'",
                    album.name,
                    album
                        .album_artists
                        .iter()
                        .map(|artist| artist.name.clone())
                        .collect::<Vec<_>>()
                        .join(" / ")
                )
            });

            None
        })
        .collect::<Vec<_>>();

    pb.finish();

    let errors = errors.lock().unwrap();

    if !errors.is_empty() {
        for (i, err) in errors.iter().enumerate() {
            error!(
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

    for dir in track_path.ancestors().skip(1) {
        let items: Vec<_> = fs::read_dir(dir)
            .with_context(|| {
                format!(
                    "Failed to read directory during art discovery: {}",
                    dir.display()
                )
            })?
            .collect::<Result<_, _>>()
            .context("Failed during art discovery")?;

        for filename in COVER_FILENAMES {
            for extension in COVER_EXTENSIONS {
                let first_match = items.iter().find(|item| {
                    item.file_name().to_string_lossy().to_ascii_lowercase()
                        == format!("{filename}.{extension}")
                });

                if let Some(item) = first_match {
                    let art =
                        make_album_art(&item.path(), base_dir, album_id).with_context(|| {
                            format!(
                                "Failed to make art for album cover at: {}",
                                item.path().to_string_lossy()
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
    // let img = image::open(path).context("Failed to open the image file")?;

    let relative_path = path
        .strip_prefix(base_dir)
        .expect("Internal error: art path couldn't be stripped of the base directory")
        .to_path_buf();

    let target = ArtTarget::AlbumCover(album_id);

    Ok(Art {
        relative_path,
        target,

        id: target.to_id(),
        // width: img.width(),
        // height: img.height(),
        // blurhash: generate_blurhash(&img, MAX_BLURHASH_COMPONENTS_X, MAX_BLURHASH_COMPONENTS_Y)?,
        // dominant_color: get_dominant_color(&img)?,
    })
}

// fn get_dominant_color(img: &DynamicImage) -> Result<Option<ArtRgb>> {
//     let img = match img.as_rgb8() {
//         Some(img) => img,
//         None => return Ok(None),
//     };

//     let bytes_count = img.as_bytes().len();
//     let expected = usize::try_from(img.width() * img.height() * 3).unwrap();

//     if bytes_count != expected {
//         bail!("Invalid image bytes count (found {bytes_count} bytes, expected {expected} bytes)");
//     }

//     let dominant_color = color_thief::get_palette(img.as_bytes(), ColorFormat::Rgb, 10, 2)?;

//     if dominant_color.len() != 2 {
//         bail!("Color Thief did not return exactly one color");
//     }

//     let dominant_color = dominant_color[0];

//     Ok(Some(ArtRgb {
//         r: dominant_color.r,
//         g: dominant_color.g,
//         b: dominant_color.b,
//     }))
// }
