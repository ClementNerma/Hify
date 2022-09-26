use std::{collections::HashMap, path::PathBuf};

use super::{AlbumID, IndexCache};

static COVER_FILENAMES: &[&str] = &["cover", "Cover", "folder", "Folder"];
static COVER_EXTENSIONS: &[&str] = &["jpg", "JPG", "jpeg", "JPEG", "png", "PNG"];

pub fn find_albums_arts(
    album_ids: &[&AlbumID],
    cache: &IndexCache,
) -> HashMap<AlbumID, Option<PathBuf>> {
    album_ids
        .iter()
        .map(|id| ((*id).clone(), find_album_art(id, cache)))
        .inspect(|(album_id, art_path)| {
            if art_path.is_some() {
                return;
            }

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
