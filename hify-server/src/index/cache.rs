use std::{
    collections::{BTreeSet, HashMap, HashSet},
    path::PathBuf,
};

use super::{AlbumID, AlbumInfos, ArtistID, ArtistInfos, IndexCache, SortedMap, Track, TrackID};

// TODO: lots of optimization to perform here
pub fn build_index_cache(
    tracks: &SortedMap<TrackID, Track>,
    tracks_paths: HashMap<TrackID, PathBuf>,
) -> IndexCache {
    let mut tracks_formats = HashMap::new();

    let mut artists_albums = HashMap::<ArtistID, BTreeSet<AlbumInfos>>::new();
    let mut artists_tracks = HashMap::<ArtistID, Vec<TrackID>>::new();
    let mut albums_artists_albums = HashMap::<ArtistID, BTreeSet<AlbumInfos>>::new();
    let mut albums_tracks = HashMap::<AlbumID, Vec<TrackID>>::new();

    let mut artists_infos = HashMap::<ArtistID, ArtistInfos>::new();
    let mut albums_artists_infos = HashMap::<ArtistID, ArtistInfos>::new();
    let mut albums_infos = HashMap::<AlbumID, AlbumInfos>::new();

    let mut genres_tracks = HashMap::<String, Vec<TrackID>>::new();
    let mut no_genre_tracks = HashSet::new();

    for track in tracks.values() {
        tracks_formats.insert(track.id.clone(), track.metadata.format);

        let tags = &track.metadata.tags;

        let album_infos = tags.get_album_infos();
        let album_id = album_infos.get_id();

        albums_infos.insert(album_id.clone(), album_infos.clone());

        for album_artist_infos in tags.get_album_artists_infos() {
            albums_artists_infos.insert(album_artist_infos.get_id(), album_artist_infos.clone());

            albums_artists_albums
                .entry(album_artist_infos.get_id())
                .or_default()
                .insert(album_infos.clone());
        }

        for artist_infos in tags
            .get_album_artists_infos()
            .chain(tags.get_artists_infos())
        {
            let artist_id = artist_infos.get_id();

            artists_infos.insert(artist_id.clone(), artist_infos.clone());

            artists_albums
                .entry(artist_id.clone())
                .or_default()
                .insert(album_infos.clone());

            artists_tracks
                .entry(artist_id.clone())
                .or_default()
                .push(track.id.clone());
        }

        albums_tracks
            .entry(album_id.clone())
            .or_default()
            .push(track.id.clone());

        if track.metadata.tags.genres.is_empty() {
            no_genre_tracks.insert(track.id.clone());
        } else {
            for genre in &track.metadata.tags.genres {
                genres_tracks
                    .entry(genre.clone())
                    .or_default()
                    .push(track.id.clone());
            }
        }
    }

    let artists_albums = artists_albums
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                SortedMap::from_vec(v.into_iter().collect(), |album| album.get_id()),
            )
        })
        .collect();

    let albums_artists_albums = albums_artists_albums
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                SortedMap::from_vec(v.into_iter().collect(), |album| album.get_id()),
            )
        })
        .collect();

    let artists_tracks = artists_tracks
        .into_iter()
        .map(|(artist_id, mut artist_tracks)| {
            artist_tracks.sort_by(|a, b| tracks.get(a).unwrap().cmp(tracks.get(b).unwrap()));
            (artist_id, artist_tracks)
        })
        .collect();

    let albums_tracks = albums_tracks
        .into_iter()
        .map(|(album_id, mut album_tracks)| {
            album_tracks.sort_by(|a, b| tracks.get(a).unwrap().cmp(tracks.get(b).unwrap()));
            (album_id, album_tracks)
        })
        .collect();

    let genres_tracks = genres_tracks
        .into_iter()
        .map(|(genre_id, mut genre_tracks)| {
            genre_tracks.sort_by(|a, b| tracks.get(a).unwrap().cmp(tracks.get(b).unwrap()));
            (genre_id, genre_tracks)
        })
        .collect();

    IndexCache {
        tracks_paths,

        artists_albums,
        artists_tracks,

        albums_artists_albums,
        albums_tracks,

        artists_infos: SortedMap::from_hashmap(artists_infos),
        albums_artists_infos: SortedMap::from_hashmap(albums_artists_infos),
        albums_infos: SortedMap::from_hashmap(albums_infos),

        genres_tracks,
        no_genre_tracks,
    }
}
