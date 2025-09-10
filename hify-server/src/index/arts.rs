use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use anyhow::{Context, Result, anyhow, ensure};
use image::{
    GenericImage, ImageBuffer, Rgba,
    imageops::{FilterType, resize},
};
use log::{debug, error, warn};
use rayon::iter::{ParallelBridge, ParallelIterator};
use tokio::{fs, sync::Mutex, task::JoinSet};

use crate::{logging::progress_bar, resources::ResourceManager};

use super::{AlbumID, AlbumInfos, ArtistInfos, IndexCache, Track, TrackID, ValueOrdMap};

static COVER_FILENAMES: &[&str] = &["cover", "folder"];
static COVER_EXTENSIONS: &[&str] = &["jpg", "jpeg", "jfif", "png"];

pub async fn find_albums_arts(
    albums: impl ExactSizeIterator<Item = AlbumInfos>,
    base_dir: &Path,
    tracks: ValueOrdMap<TrackID, Track>,
    cache: IndexCache,
) -> Result<Vec<(AlbumID, PathBuf)>> {
    let mut set = JoinSet::<Result<Option<(AlbumID, PathBuf)>>>::new();
    let tracks = Arc::new(tracks);
    let cache = Arc::new(cache);

    let pb = progress_bar(albums.len());

    for album in albums {
        let base_dir = base_dir.to_path_buf();
        let pb = pb.clone();
        let tracks = Arc::clone(&tracks);
        let cache = Arc::clone(&cache);

        set.spawn(async move {
            let album_id = album.get_id();

            let result = find_album_art(album_id, &base_dir, &tracks, &cache).await?;

            if result.is_none() {
                pb.suspend(|| {
                    warn!(
                        "Warning: no album art found for album '{}' by '{}'",
                        album.name,
                        album
                            .album_artists
                            .iter()
                            .map(|artist| artist.name.clone())
                            .collect::<Vec<_>>()
                            .join(" / ")
                    );
                });
            }

            pb.inc(1);

            Ok(result.map(|relative_path| (album_id, relative_path)))
        });
    }

    let mut arts = vec![];
    let mut errors = 0;

    while let Some(res) = set.join_next().await {
        match res? {
            Ok(art) => {
                if let Some(art) = art {
                    arts.push(art)
                }
            }

            Err(err) => {
                pb.suspend(|| error!("Error: {err}"));
                errors += 1;
            }
        }
    }

    pb.finish();

    ensure!(errors == 0, "Encountered {errors} error(s)");

    Ok(arts)
}

async fn find_album_art(
    album_id: AlbumID,
    base_dir: &Path,
    tracks: &ValueOrdMap<TrackID, Track>,
    cache: &IndexCache,
) -> Result<Option<PathBuf>> {
    let album_tracks_ids = cache.albums_tracks.get(&album_id).unwrap();

    // Cannot fail as albums need at least one track to be registered
    let first_track_id = album_tracks_ids.first().unwrap();

    let track_path = base_dir.join(&tracks.get(first_track_id).unwrap().relative_path);

    for dir in track_path.ancestors().skip(1) {
        let mut dir_iter = fs::read_dir(dir).await.with_context(|| {
            format!(
                "Failed to read directory during art discovery: {}",
                dir.display()
            )
        })?;

        loop {
            let Some(item) = dir_iter.next_entry().await? else {
                break;
            };

            for filename in COVER_FILENAMES {
                for extension in COVER_EXTENSIONS {
                    if item.file_name().to_string_lossy().to_ascii_lowercase()
                        == format!("{filename}.{extension}")
                    {
                        let relative_path = item.path()
                            .strip_prefix(base_dir)
                            .expect("Internal error: art path couldn't be stripped of the base directory")
                            .to_path_buf();

                        return Ok(Some(relative_path));
                    }
                }
            }
        }
    }

    Ok(None)
}

pub fn generate_artist_arts<'a>(
    artists: impl ExactSizeIterator<Item = &'a ArtistInfos> + ParallelBridge + Send,
    base_dir: &Path,
    album_arts: &HashMap<AlbumID, PathBuf>,
    cache: IndexCache,
    res_manager: ResourceManager,
) -> Result<()> {
    let pb = progress_bar(artists.len());

    let errors = AtomicUsize::new(0);

    artists.par_bridge().for_each(|artist| {
        let res_manager = res_manager.clone();
        let artist_id = artist.get_id();

        debug!("Generating art for artist '{}'...", artist.name);

        match generate_artist_art(
            base_dir,
            cache
                .artists_albums_and_participations
                .get(&artist_id)
                .unwrap()
                .keys()
                .copied(),
            album_arts,
        ) {
            Err(err) => {
                pb.suspend(|| {
                    error!(
                        "Failed to generate cover art for artist '{}': {}",
                        artist.name,
                        format!("{err:?}")
                            .lines()
                            .map(str::trim)
                            .filter(|line| !line.is_empty())
                            .collect::<Vec<_>>()
                            .join(" > ")
                    );
                });

                errors.fetch_add(1, Ordering::SeqCst);
            }

            Ok(img) => {
                if let Some(img) = img
                    && let Err(err) = res_manager.save_artist_art(artist_id, img)
                {
                    pb.suspend(|| {
                        error!(
                            "Failed to save cover art for artist '{}' to disk: {err}",
                            artist.name
                        );
                    });

                    errors.fetch_add(1, Ordering::SeqCst);
                }
            }
        }

        pb.inc(1);
    });

    pb.finish();

    let errors = errors.load(Ordering::SeqCst);

    ensure!(errors == 0, "Failed with {errors} error(s)");

    Ok(())
}

fn generate_artist_art(
    base_dir: &Path,
    albums_and_participations: impl Iterator<Item = AlbumID>,
    album_arts: &HashMap<AlbumID, PathBuf>,
) -> Result<Option<ImageBuffer<Rgba<u8>, Vec<u8>>>> {
    // Only 4 arts are needed but we analyze them all to ensure every image is correct on disk
    let album_arts = albums_and_participations
        .filter_map(|album_id| album_arts.get(&album_id))
        .map(|relative_path| {
            let path = base_dir.join(relative_path);

            image::open(&path)
                .with_context(|| format!("Failed to open image file at path '{}'", path.display()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    if album_arts.is_empty() {
        return Ok(None);
    }

    let image = match album_arts.len() {
        1 => {
            // TODO: dynamic dimensions (choose what width and height artists' arts should be and apply it here as well)

            let art = resize(&album_arts[0], 500, 500, FilterType::Lanczos3);

            let mut image = ImageBuffer::new(art.width(), art.height());

            image
                .copy_from(&art, 0, 0)
                .context("Failed to copy single cover art into artist's one")?;

            image
        }

        _ => {
            let (top_left, top_right, bottom_left, bottom_right) = match album_arts.as_slice() {
                [left, right] => (left, right, right, left),

                [top_left, top_right, bottom_left] => (top_left, top_right, bottom_left, top_left),

                [top_left, top_right, bottom_left, bottom_right, ..] => {
                    (top_left, top_right, bottom_left, bottom_right)
                }

                _ => unreachable!(),
            };

            // TODO: dynamic dimensions
            // TODO: choose how to handle images with different aspect ratios
            let mut image = ImageBuffer::new(1000, 1000);

            let resize = |image| resize(image, 500, 500, FilterType::Lanczos3);

            image
                .copy_from(&resize(top_left), 0, 0)
                .and_then(|()| image.copy_from(&resize(top_right), 500, 0))
                .and_then(|()| image.copy_from(&resize(bottom_left), 0, 500))
                .and_then(|()| image.copy_from(&resize(bottom_right), 500, 500))
                .map_err(|err| {
                    anyhow!("Failed to copy album cover arts into the artist's one: {err}")
                })?;

            image
        }
    };

    Ok(Some(image))
}

pub async fn detect_deleted_album_arts(
    base_dir: &Path,
    album_arts: &HashMap<AlbumID, PathBuf>,
) -> Result<Vec<AlbumID>> {
    let mut arts_file_checker = JoinSet::new();
    let deleted_arts = Arc::new(Mutex::new(vec![]));

    for (album_id, path) in album_arts {
        let album_id = *album_id;
        let path = path.clone();
        let deleted_arts = Arc::clone(&deleted_arts);
        let base_dir = base_dir.to_owned(); // TODO: wrap in an Arc<> instead

        arts_file_checker.spawn(async move {
            if !fs::try_exists(base_dir.join(path))
                .await
                .is_ok_and(|exists| exists)
            {
                deleted_arts.lock().await.push(album_id);
            }
        });
    }

    arts_file_checker.join_all().await;

    Ok(Arc::into_inner(deleted_arts).unwrap().into_inner())
}
