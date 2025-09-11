use std::{collections::BTreeSet, time::SystemTime};

use jiff::fmt::temporal::DateTimePrinter;

use crate::{
    index::{AlbumInfos, ArtistInfos, IdType, Index, Rating, Track},
    userdata::UserDataWrapper,
};

use super::types::{AlbumID3WithSongs, ArtistID3, Child, CoverArtId, ItemGenre};

pub fn track_to_child(track: &Track, index: &Index, user_data: &UserDataWrapper) -> Child {
    let tags = &track.metadata.tags;

    let album_id = tags.get_album_infos().get_id();

    let rating = user_data
        .track_ratings()
        .get(&track.id)
        .map(|rating| match rating {
            Rating::Zero | Rating::One => 0,
            Rating::Two | Rating::Three => 1,
            Rating::Four | Rating::Five => 2,
            Rating::Six | Rating::Seven => 3,
            Rating::Eight | Rating::Nine => 4,
            Rating::Ten => 5,
        });

    Child {
        id: track.id.encode(),
        parent: Some(album_id.encode()),
        is_dir: false,
        title: tags.title.clone(),
        album_name: Some(tags.album.clone()),
        artist_name: tags.artists.first().cloned(), // OK?
        track_number: tags.track_no,
        year: tags.date.map(|date| date.year),
        genre: tags.genres.first().cloned(), // OK?
        covert_art_id: None,                 // TODO
        size_bytes: Some(track.metadata.file_size),
        mime_type: None,      // TODO
        file_extension: None, // TODO
        duration_s: Some(track.metadata.duration),
        bit_rate: None,      // TODO
        bit_depth: None,     // TODO
        sampling_rate: None, // TODO
        channel_count: None, // TODO
        path: None,          // TODO
        is_video: Some(false),
        user_rating_1_to_5: rating,
        average_rating_1_to_5: rating.map(f32::from),
        play_count: None, // TODO (requires caching)
        disc_number: tags.disc,
        created_iso_8601: Some(to_iso_8601(track.mtime)),
        starred_iso_8601: None, // TODO?
        album_id: Some(album_id.encode()),
        artist_id: tags
            .get_artists_infos()
            .next()
            .map(|artist| artist.get_id().encode()),
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
            tags.get_genres_infos()
                .map(|genre| ItemGenre { name: genre.name })
                .collect(),
        ),
        artists: Some(
            tags.get_artists_infos()
                .map(|artist| artist_to_id3(&artist, index))
                .collect(),
        ),
        album_artists: Some(
            tags.get_album_artists_infos()
                .map(|artist| artist_to_id3(&artist, index))
                .collect(),
        ),
        contributors: None, // TODO?
    }
}

pub fn album_to_id3_with_songs(
    album: &AlbumInfos,
    index: &Index,
    user_data: Option<&UserDataWrapper>,
) -> AlbumID3WithSongs {
    let album_tracks = index.cache.albums_tracks.get(&album.get_id()).unwrap();
    let album_tracks = album_tracks
        .iter()
        .map(|track| index.tracks.get(track).unwrap())
        .collect::<Vec<_>>();

    // TODO: good idea?
    let first_artist = album.album_artists.first().unwrap();

    AlbumID3WithSongs {
        id: album.get_id(),
        name: album.name.clone(),
        version: None,
        artist_name: Some(first_artist.name.clone()),
        artist_id: Some(first_artist.get_id()),
        cover_art_id: Some(CoverArtId::Album(album.get_id())),
        song_count: album_tracks.len(),
        duration_s: album_tracks
            .iter()
            .map(|track| track.metadata.duration)
            .sum(),
        play_count: None, // TODO (possible to compute?)
        created_iso_8601: to_iso_8601(album_tracks.iter().map(|track| track.mtime).min().unwrap()),
        starred_iso_8601: None,
        year: None,
        genre: None,
        last_played_iso_8601: None, // TODO (requires caching)
        user_rating_1_to_5: None,
        genres: Some(
            album_tracks
                .iter()
                .flat_map(|track| &track.metadata.tags.genres)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .map(|genre| ItemGenre {
                    name: genre.to_owned(),
                })
                .collect(),
        ),
        artists: Some(
            album
                .album_artists
                .iter()
                .map(|artist| artist_to_id3(artist, index))
                .collect(),
        ),
        display_artist_name: None,
        sort_name: None,
        original_release_date: None,
        release_date: None, // TODO?
        is_compilation: None,
        explicit_status: None,
        tracks: user_data.map(|user_data| {
            album_tracks
                .iter()
                .map(|track| track_to_child(track, index, user_data))
                .collect()
        }),
    }
}

pub fn album_to_child(album: &AlbumInfos, index: &Index) -> Child {
    let album_tracks = index.cache.albums_tracks.get(&album.get_id()).unwrap();
    let album_tracks = album_tracks
        .iter()
        .map(|track| index.tracks.get(track).unwrap())
        .collect::<Vec<_>>();

    // TODO: good idea?
    let first_artist = album.album_artists.first().unwrap();

    Child {
        id: album.get_id().encode(),
        parent: Some(first_artist.get_id().encode()),
        is_dir: false,
        title: album.name.clone(),
        album_name: Some(album.name.clone()),
        artist_name: Some(first_artist.name.clone()),
        track_number: None,
        year: None,  // TODO
        genre: None, // TODO?
        covert_art_id: Some(CoverArtId::Album(album.get_id())),
        size_bytes: None, // TODO
        mime_type: None,
        file_extension: None,
        duration_s: Some(
            album_tracks
                .iter()
                .map(|track| track.metadata.duration)
                .sum(),
        ),
        bit_rate: None,
        bit_depth: None,
        sampling_rate: None,
        channel_count: None,
        path: None, // TODO?
        is_video: Some(false),
        user_rating_1_to_5: None,
        average_rating_1_to_5: index
            .cache
            .albums_mean_score
            .get(&album.get_id())
            .map(|rating| (*rating / 2.0) as f32),
        play_count: None, // TODO (possible to compute?)
        disc_number: None,
        // TODO: optimize
        created_iso_8601: Some(to_iso_8601(
            album_tracks.iter().map(|track| track.mtime).min().unwrap(),
        )),
        starred_iso_8601: None,
        album_id: Some(album.get_id().encode()),
        artist_id: Some(first_artist.get_id().encode()),
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
                .flat_map(|track| &track.metadata.tags.genres)
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
                .album_artists
                .iter()
                .map(|artist| artist_to_id3(artist, index))
                .collect(),
        ),

        album_artists: Some(
            album
                .album_artists
                .iter()
                .map(|artist| artist_to_id3(artist, index))
                .collect(),
        ),

        display_album_artist_name: None,
        contributors: None, // TODO?
        explicit_status: None,
    }
}

pub fn artist_to_id3(artist: &ArtistInfos, index: &Index) -> ArtistID3 {
    let artist_id = artist.get_id();

    ArtistID3 {
        id: artist_id,
        name: artist.name.clone(),
        covert_art_id: Some(CoverArtId::Artist(artist_id)),
        artist_image_url: None, // TODO
        album_count: index
            .cache
            .artists_albums
            .get(&artist_id)
            .map(|albums| albums.len()),
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
