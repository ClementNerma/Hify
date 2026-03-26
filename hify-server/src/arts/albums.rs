use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use anyhow::{Context, Result, bail};
use colored::Colorize;
use log::{debug, info};
use walkdir::WalkDir;

use crate::{
    index::{AlbumID, IndexCache},
    stable_hash,
    utils::TaskRunner,
};

use super::ArtsManager;

static COVER_FILE_STEMS: &[&str] = &["cover", "folder"];
static COVER_EXTENSIONS: &[&str] = &["jpg", "jpeg", "jfif", "png"];

pub fn generate_album_arts(
    index_cache: &IndexCache,
    music_dir: &Path,
    album_arts: &ArtsManager<AlbumID>,
) -> Result<()> {
    debug!(
        "-> Looking for album arts for {} albums...",
        index_cache.albums.len().to_string().bright_yellow()
    );

    let album_covers = find_album_arts(music_dir, index_cache)?;
    assert_eq!(album_covers.len(), index_cache.albums.len());

    debug!(
        "-> Found {} potential album arts in total",
        album_covers.len().to_string().bright_yellow()
    );

    info!("-> Generating miniatures for album arts...");

    let mut album_arts_tasks = TaskRunner::new();
    let total = Arc::new(AtomicUsize::new(0));

    for (album_id, art_path) in album_covers {
        let Some(art_path) = art_path else {
            if album_arts.has(album_id) {
                album_arts.delete(album_id)?;
            }

            continue;
        };

        let album_arts = album_arts.clone();

        let music_dir = music_dir.to_owned();
        let total = Arc::clone(&total);

        album_arts_tasks.spawn(move || {
            let mt = std::fs::metadata(&art_path).with_context(|| {
                format!(
                    "Failed to get metadata for image file: {}",
                    art_path.display()
                )
            })?;

            let mtime = mt.modified().with_context(|| {
                format!(
                    "Failed to get modification time for image file: {}",
                    art_path.display()
                )
            })?;

            let hash = stable_hash!(art_path.strip_prefix(music_dir).unwrap(), mtime);

            if album_arts.has_with_source_data(album_id, hash) {
                return Ok(());
            }

            let img = image::open(&art_path).with_context(|| {
                format!(
                    "Failed to open image file for album art: {}",
                    art_path.display()
                )
            })?;

            assert!(album_arts.register(album_id, hash, &img.into_rgb8())?);

            let curr = total.fetch_add(1, Ordering::SeqCst) + 1;

            if curr.is_multiple_of(100) {
                debug!(
                    "--> Generated miniatures for {} album arts so far...",
                    curr.to_string().bright_yellow()
                );
            }

            Ok(())
        });
    }

    album_arts_tasks
        .join_all()
        .context("Failed to register some album arts")?;

    let total = total.load(Ordering::SeqCst);

    if total > 0 {
        info!(
            "--> Successfully generated miniatures for {} album arts",
            total.to_string().bright_yellow()
        );
    }

    Ok(())
}

fn find_album_arts(dir: &Path, index: &IndexCache) -> Result<HashMap<AlbumID, Option<PathBuf>>> {
    let mut img_files = WalkDir::new(dir)
        .min_depth(1)
        .into_iter()
        .filter(|entry| match entry {
            Ok(entry) => is_cover_file_path(entry.path()),
            Err(_) => true,
        })
        .collect::<Result<Vec<_>, _>>()?;

    img_files.sort_by(|a, b| a.path().cmp(b.path()));

    let mut arts = HashMap::new();

    for album_id in index.albums.keys() {
        let album_root_path = index
            .albums_tracks_relative_common_path
            .get(album_id)
            .unwrap();

        let art_path = img_files.iter().find(|entry| {
            entry
                .path()
                .strip_prefix(dir)
                .unwrap()
                .strip_prefix(album_root_path)
                .is_ok()
        });

        arts.insert(*album_id, art_path.map(|p| p.path().to_owned()));

        if art_path.is_none() {
            bail!(
                "No art found for album at path: {}",
                album_root_path.display()
            );
        }
    }

    Ok(arts)
}

fn is_cover_file_path(path: &Path) -> bool {
    let Some(file_stem) = path.file_stem().and_then(OsStr::to_str) else {
        return false;
    };

    let Some(ext) = path.extension().and_then(OsStr::to_str) else {
        return false;
    };

    COVER_FILE_STEMS
        .iter()
        .any(|valid_file_stem| file_stem.eq_ignore_ascii_case(valid_file_stem))
        && COVER_EXTENSIONS
            .iter()
            .any(|valid_ext| valid_ext.eq_ignore_ascii_case(ext))
}
