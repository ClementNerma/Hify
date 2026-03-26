mod cache;
mod cmp;
mod content;

use std::collections::{HashMap, HashSet};

pub use self::{cache::IndexCache, cmp::CmpIndex, content::*};

pub fn assert_index_correctness(index: &Index) {
    let Index {
        tracks,
        albums,
        artists,
        genres,
    } = index;

    let albums_map = albums
        .iter()
        .map(|album| (&album.id, album))
        .collect::<HashMap<_, _>>();

    let artists_map = artists
        .iter()
        .map(|artist| (&artist.id, artist))
        .collect::<HashMap<_, _>>();

    let genres_map = genres
        .iter()
        .map(|genre| (&genre.id, genre))
        .collect::<HashMap<_, _>>();

    let mut detected_albums = HashSet::new();

    for track in tracks {
        let Track {
            id,
            relative_path,
            file_size_bytes,
            file_times: _,
            metadata: _,
            tags,
        } = track;

        assert_eq!(TrackID::compute(relative_path), *id);

        assert!(relative_path.is_relative());
        assert!(*file_size_bytes > 0);

        let TrackTags {
            title: _,
            artists_id,
            composers_id,
            album_id,
            disc_number: _,
            track_number: _,
            date: _,
            genres_id,
        } = tags;

        assert!(albums_map.contains_key(album_id));
        detected_albums.insert(album_id);

        for artist_id in artists_id.iter().chain(composers_id) {
            assert!(artists_map.contains_key(artist_id));
        }

        for genre_id in genres_id {
            assert!(genres_map.contains_key(genre_id));
        }
    }

    for album in albums {
        let Album {
            id: _,
            name: _,
            artists_id,
        } = album;

        assert!(detected_albums.contains(&album.id));

        for artist_id in artists_id {
            assert!(artists_map.contains_key(artist_id));
        }
    }

    let mut artist_names = HashSet::new();

    for artist in artists {
        let Artist { id: _, name } = artist;
        assert!(artist_names.insert(name));
    }

    let mut genre_names = HashSet::new();

    for genre in genres {
        let Genre { id: _, name } = genre;
        assert!(genre_names.insert(name));
    }
}
