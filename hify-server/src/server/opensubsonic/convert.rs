use std::{collections::BTreeSet, time::SystemTime};

use indexmap::IndexSet;
use jiff::fmt::temporal::DateTimePrinter;

use crate::{
    index::{Album, Artist, IdType, IndexCache, Rating, Track},
    manager::Ratings,
};

use super::types::{AlbumID3WithSongs, ArtistID3, Child, CoverArtId, ItemGenre};

pub fn track_to_child(track: &Track, index: &IndexCache, ratings: &Ratings) -> Child {
    let tags = &track.tags;

    let rating = ratings.get(&track.id).map(Rating::get_zero_to_five);

    let album = index.albums.get(&tags.album_id).unwrap();

    Child {
        id: track.id.encode(),
        parent: Some(tags.album_id.encode()),
        is_dir: false,
        title: tags.title.clone(),
        album_name: Some(album.name.clone()),
        artist_name: tags
            .artists_id
            .first()
            .map(|artist_id| index.artists.get(artist_id).unwrap().name.clone()), // OK?
        track_number: tags.track_number,
        year: tags.date.map(|date| u32::from(date.year)),
        genre: tags
            .genres_id
            .first()
            .map(|genre_id| index.genres.get(genre_id).unwrap().name.clone()), // OK?
        covert_art_id: None, // TODO
        size_bytes: Some(track.file_size_bytes),
        mime_type: None,      // TODO
        file_extension: None, // TODO
        duration_s: Some(track.metadata.duration_s),
        bit_rate: None,      // TODO
        bit_depth: None,     // TODO
        sampling_rate: None, // TODO
        channel_count: None, // TODO
        path: None,          // TODO
        is_video: Some(false),
        user_rating_1_to_5: rating,
        average_rating_1_to_5: rating.map(f32::from),
        play_count: None, // TODO (requires caching)
        disc_number: tags.disc_number,
        created_iso_8601: Some(to_iso_8601(
            track.file_times.ctime.unwrap_or(track.file_times.mtime),
        )),
        starred_iso_8601: None, // TODO?
        album_id: Some(tags.album_id.encode()),
        artist_id: tags.artists_id.first().map(IdType::encode), // OK?
        typ: Some("music"),
        last_played_iso_8601: None, // TODO: (requires caching)
        bpm: None,
        comment: None,
        sort_name: None,
        music_brainz_id: None,
        isrc: None,
        display_artist_name: None,
        display_album_artist_name: None,
        explicit_status: None,
        genres: Some(
            tags.genres_id
                .iter()
                .map(|genre_id| ItemGenre {
                    name: index.genres.get(genre_id).unwrap().name.clone(),
                })
                .collect(),
        ),
        artists: Some(
            tags.artists_id
                .iter()
                .map(|artist_id| artist_to_id3(index.artists.get(artist_id).unwrap(), index))
                .collect(),
        ),
        album_artists: Some(
            album
                .artists_id
                .iter()
                .map(|artist_id| artist_to_id3(index.artists.get(artist_id).unwrap(), index))
                .collect(),
        ),
        contributors: None, // TODO?
    }
}

pub fn album_to_id3_with_songs(
    album: &Album,
    index: &IndexCache,
    ratings: &Ratings,
) -> AlbumID3WithSongs {
    let album_tracks = index.albums_tracks.get(&album.id).unwrap();
    let album_tracks = album_tracks
        .iter()
        .map(|track| index.tracks.get(track).unwrap())
        .collect::<Vec<_>>();

    // TODO: good idea?
    let first_artist = index
        .artists
        .get(album.artists_id.first().unwrap())
        .unwrap();

    AlbumID3WithSongs {
        id: album.id,
        name: album.name.clone(),
        version: None,
        artist_name: Some(first_artist.name.clone()),
        artist_id: Some(first_artist.id),
        cover_art_id: Some(CoverArtId::Album(album.id)),
        song_count: album_tracks.len(),
        duration_s: album_tracks
            .iter()
            .map(|track| track.metadata.duration_s)
            .sum(),
        play_count: None, // TODO (possible to compute?)
        created_iso_8601: to_iso_8601(
            album_tracks
                .iter()
                .map(|track| track.file_times.ctime.unwrap_or(track.file_times.mtime))
                .min()
                .unwrap(),
        ),
        starred_iso_8601: None,
        year: None,
        genre: None,
        last_played_iso_8601: None, // TODO (requires caching)
        user_rating_1_to_5: None,
        genres: Some(
            album_tracks
                .iter()
                .flat_map(|track| &track.tags.genres_id)
                .map(|genre_id| index.genres.get(genre_id).unwrap().name.as_str())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .map(|genre| ItemGenre {
                    name: genre.to_owned(),
                })
                .collect(),
        ),
        artists: Some(
            album
                .artists_id
                .iter()
                .map(|artist| artist_to_id3(index.artists.get(artist).unwrap(), index))
                .collect(),
        ),
        display_artist_name: None,
        sort_name: None,
        original_release_date: None,
        release_date: None, // TODO?
        is_compilation: None,
        explicit_status: None,
        tracks: Some(
            album_tracks
                .iter()
                .map(|track| track_to_child(track, index, ratings))
                .collect(),
        ),
    }
}

pub fn album_to_child(album: &Album, index: &IndexCache) -> Child {
    let album_tracks = index.albums_tracks.get(&album.id).unwrap();

    let album_tracks = album_tracks
        .iter()
        .map(|track| index.tracks.get(track).unwrap())
        .collect::<Vec<_>>();

    // TODO: good idea?
    let first_artist = index
        .artists
        .get(album.artists_id.first().unwrap())
        .unwrap();

    Child {
        id: album.id.encode(),
        parent: Some(first_artist.id.encode()),
        is_dir: false,
        title: album.name.clone(),
        album_name: Some(album.name.clone()),
        artist_name: Some(first_artist.name.clone()),
        track_number: None,
        year: None,  // TODO
        genre: None, // TODO?
        covert_art_id: Some(CoverArtId::Album(album.id)),
        size_bytes: None, // TODO
        mime_type: None,
        file_extension: None,
        duration_s: Some(
            album_tracks
                .iter()
                .map(|track| track.metadata.duration_s)
                .sum(),
        ),
        bit_rate: None,
        bit_depth: None,
        sampling_rate: None,
        channel_count: None,
        path: None, // TODO?
        is_video: Some(false),
        user_rating_1_to_5: None,
        average_rating_1_to_5: None, // TODO (requires caching)
        play_count: None,            // TODO (possible to compute?)
        disc_number: None,
        // TODO: optimize
        created_iso_8601: Some(to_iso_8601(
            album_tracks
                .iter()
                .map(|track| track.file_times.ctime.unwrap_or(track.file_times.mtime))
                .min()
                .unwrap(),
        )),
        starred_iso_8601: None,
        album_id: Some(album.id.encode()),
        artist_id: Some(first_artist.id.encode()),
        typ: Some("music"),
        last_played_iso_8601: None, // TODO (requires caching)
        bpm: None,
        comment: None,
        sort_name: None,
        music_brainz_id: None,
        isrc: None,

        genres: Some(
            album_tracks
                .iter()
                .flat_map(|track| &track.tags.genres_id)
                .map(|genre_id| index.genres.get(genre_id).unwrap().name.as_str())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .map(|genre| ItemGenre {
                    name: genre.to_owned(),
                })
                .collect(),
        ),

        display_artist_name: None,

        artists: Some(
            album
                .artists_id
                .iter()
                .map(|artist| artist_to_id3(index.artists.get(artist).unwrap(), index))
                .collect(),
        ),

        album_artists: Some(
            album
                .artists_id
                .iter()
                .map(|artist| artist_to_id3(index.artists.get(artist).unwrap(), index))
                .collect(),
        ),

        display_album_artist_name: None,
        contributors: None, // TODO?
        explicit_status: None,
    }
}

pub fn artist_to_id3(artist: &Artist, index: &IndexCache) -> ArtistID3 {
    ArtistID3 {
        id: artist.id,
        name: artist.name.clone(),
        covert_art_id: Some(CoverArtId::Artist(artist.id)),
        artist_image_url: None, // TODO
        album_count: index.artists_albums.get(&artist.id).map(IndexSet::len),
        starred_iso_8601: None,
        music_brainz_id: None,
        sort_name: None,
    }
}

// TODO: optimize
pub fn to_iso_8601(st: SystemTime) -> String {
    let printer = DateTimePrinter::new();
    let mut buf = String::new();
    printer
        .print_zoned(&jiff::Zoned::try_from(st).unwrap(), &mut buf)
        .unwrap();
    buf
}
