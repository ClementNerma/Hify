use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context, Result};
use log::warn;

use crate::{
    arts::{RegisterableArtType, hash_for_file},
    runner::{TaskSet, TaskSetOptions},
};

use crate::index::{AlbumID, Index};

static COVER_FILENAMES: &[&str] = &["cover", "folder"];
static COVER_EXTENSIONS: &[&str] = &["jpg", "jpeg", "jfif", "png"];

pub fn find_album_covers(
    base_dir: &Path,
    index: Index,
) -> Result<Vec<(AlbumID, u64, RegisterableArtType)>> {
    let index = Arc::new(index);

    let albums = index.albums_infos.keys().copied();

    let mut runner = TaskSet::new();

    for album_id in albums {
        let base_dir = base_dir.to_path_buf();
        let index = Arc::clone(&index);

        runner.add(
            move || -> Result<Option<(AlbumID, u64, RegisterableArtType)>> {
                let Some(album_cover) = find_album_cover(album_id, &base_dir, &index)? else {
                    let album = index.albums_infos.get(&album_id).unwrap();

                    // TODO: pb.suspend()
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

                    return Ok(None);
                };

                let ret = (
                    album_id,
                    hash_for_file(&album_cover)?,
                    RegisterableArtType::File(album_cover),
                );

                Ok(Some(ret))
            },
        );
    }

    let results = runner.run(TaskSetOptions::with_progress_bar());

    results
        .into_iter()
        .filter_map(|result| result.flatten().transpose())
        .collect::<Result<Vec<_>, _>>()
}

fn find_album_cover(album_id: AlbumID, base_dir: &Path, index: &Index) -> Result<Option<PathBuf>> {
    let album_tracks_ids = index.albums_tracks.get(&album_id).unwrap();

    // Cannot fail as albums need at least one track to be registered
    let first_track_id = album_tracks_ids.first().unwrap();

    let track_path = base_dir.join(&index.tracks.get(first_track_id).unwrap().relative_path);

    for dir in track_path.ancestors().skip(1) {
        for entry in fs::read_dir(dir).with_context(|| {
            format!(
                "Failed to read directory during art discovery: {}",
                dir.display()
            )
        })? {
            let entry = entry?;

            for filename in COVER_FILENAMES {
                for extension in COVER_EXTENSIONS {
                    if entry.file_name().to_string_lossy().to_ascii_lowercase()
                        == format!("{filename}.{extension}")
                    {
                        return Ok(Some(entry.path().to_owned()));
                    }
                }
            }
        }
    }

    Ok(None)
}
