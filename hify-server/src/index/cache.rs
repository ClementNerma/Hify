use std::{
    collections::{BTreeSet, HashMap, HashSet},
    path::PathBuf,
};

use super::{
    AlbumID, AlbumInfos, ArtistID, ArtistInfos, GenreID, GenreInfos, IndexCache, SortedMap, Track,
    TrackID,
};

// TODO: lots of optimization to perform here
pub fn build_index_cache(
    tracks: &SortedMap<TrackID, Track>,
    tracks_paths: HashMap<TrackID, PathBuf>,
) -> IndexCache {
    let mut tracks_formats = HashMap::new();

    let mut artists_albums = HashMap::<ArtistID, BTreeSet<AlbumInfos>>::new();
    let mut artists_album_participations = HashMap::<ArtistID, BTreeSet<AlbumInfos>>::new();
    let mut artists_track_participations = HashMap::<ArtistID, Vec<TrackID>>::new();
    let mut artists_tracks = HashMap::<ArtistID, Vec<TrackID>>::new();
    let mut albums_tracks = HashMap::<AlbumID, Vec<TrackID>>::new();

    let mut artists_infos = HashMap::<ArtistID, ArtistInfos>::new();
    let mut albums_artists_infos = HashMap::<ArtistID, ArtistInfos>::new();
    let mut albums_infos = HashMap::<AlbumID, AlbumInfos>::new();

    let mut genres_infos = HashMap::<GenreID, GenreInfos>::new();
    let mut genres_albums = HashMap::<GenreID, BTreeSet<AlbumInfos>>::new();
    let mut genres_tracks = HashMap::<GenreID, Vec<TrackID>>::new();
    let mut no_genre_tracks = HashSet::new();

    for track in tracks.values() {
        tracks_formats.insert(track.id.clone(), track.metadata.format);

        let tags = &track.metadata.tags;

        let album_infos = tags.get_album_infos();
        let album_id = album_infos.get_id();

        albums_infos.insert(album_id.clone(), album_infos.clone());

        let album_artists: HashSet<_> = tags.get_album_artists_infos().collect();

        let track_artists = tags.get_artists_infos().collect::<HashSet<_>>();
        let non_album_artists = &album_artists - &track_artists;

        for album_artist_infos in &album_artists {
            albums_artists_infos.insert(album_artist_infos.get_id(), album_artist_infos.clone());

            artists_albums
                .entry(album_artist_infos.get_id())
                .or_default()
                .insert(album_infos.clone());
        }

        for artist_infos in album_artists.iter().chain(non_album_artists.iter()) {
            let artist_id = artist_infos.get_id();

            artists_infos.insert(artist_id.clone(), artist_infos.clone());

            artists_tracks
                .entry(artist_id)
                .or_default()
                .push(track.id.clone());
        }

        for non_album_artist_infos in &non_album_artists {
            let artist_id = non_album_artist_infos.get_id();

            artists_album_participations
                .entry(artist_id.clone())
                .or_default()
                .insert(album_infos.clone());

            artists_track_participations
                .entry(artist_id)
                .or_default()
                .push(track.id.clone())
        }

        albums_tracks
            .entry(album_id.clone())
            .or_default()
            .push(track.id.clone());

        if track.metadata.tags.genres.is_empty() {
            no_genre_tracks.insert(track.id.clone());
        } else {
            for genre in tags.get_genres_infos() {
                let genre_id = genre.get_id();

                genres_infos.insert(genre_id.clone(), genre.clone());

                genres_tracks
                    .entry(genre_id.clone())
                    .or_default()
                    .push(track.id.clone());

                genres_albums
                    .entry(genre_id)
                    .or_default()
                    .insert(album_infos.clone());
            }
        }
    }

    let artists_album_participations = artists_album_participations
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                SortedMap::from_vec(v.into_iter().collect(), |album| album.get_id()),
            )
        })
        .collect();

    let artists_albums = artists_albums
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                SortedMap::from_vec(v.into_iter().collect(), |album| album.get_id()),
            )
        })
        .collect::<HashMap<_, _>>();

    let artists_tracks = artists_tracks
        .into_iter()
        .map(|(artist_id, mut artist_tracks)| {
            artist_tracks.sort_by(|a, b| tracks.get(a).unwrap().cmp(tracks.get(b).unwrap()));
            (artist_id, artist_tracks)
        })
        .collect::<HashMap<_, _>>();

    let albums_tracks = albums_tracks
        .into_iter()
        .map(|(album_id, mut album_tracks)| {
            album_tracks.sort_by(|a, b| tracks.get(a).unwrap().cmp(tracks.get(b).unwrap()));
            (album_id, album_tracks)
        })
        .collect::<HashMap<_, _>>();

    let genres_tracks = genres_tracks
        .into_iter()
        .map(|(genre_id, mut genre_tracks)| {
            genre_tracks.sort_by(|a, b| tracks.get(a).unwrap().cmp(tracks.get(b).unwrap()));
            (genre_id, genre_tracks)
        })
        .collect();

    let genres_albums = genres_albums
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                SortedMap::from_vec(v.into_iter().collect(), |album| album.get_id()),
            )
        })
        .collect();

    let albums_mean_score = albums_tracks
        .iter()
        .filter_map(|(album_id, album_tracks)| {
            let rated_tracks: Vec<_> = album_tracks
                .iter()
                .filter_map(|track_id| tracks.get(track_id).unwrap().metadata.tags.rating)
                .map(|rating| rating as f64)
                .collect();

            if rated_tracks.is_empty() {
                return None;
            }

            let mean = rated_tracks.iter().sum::<f64>() / (rated_tracks.len() as f64);
            Some((album_id.clone(), mean))
        })
        .collect();

    let artists_mean_score = artists_tracks
        .iter()
        .filter_map(|(artist_id, artist_tracks)| {
            let rated_tracks: Vec<_> = artist_tracks
                .iter()
                .filter_map(|track_id| tracks.get(track_id).unwrap().metadata.tags.rating)
                .map(|rating| rating as f64)
                .collect();

            if rated_tracks.is_empty() {
                return None;
            }

            let mean = rated_tracks.iter().sum::<f64>() / (rated_tracks.len() as f64);
            Some((artist_id.clone(), mean))
        })
        .collect();

    let albums_artists_mean_score = artists_albums
        .iter()
        .filter_map(|(artist_id, artist_albums)| {
            let rated_tracks: Vec<_> = artist_albums
                .keys()
                .flat_map(|album_id| albums_tracks.get(album_id).unwrap())
                .filter_map(|track_id| tracks.get(track_id).unwrap().metadata.tags.rating)
                .map(|rating| rating as f64)
                .collect();

            if rated_tracks.is_empty() {
                return None;
            }

            let mean = rated_tracks.iter().sum::<f64>() / (rated_tracks.len() as f64);
            Some((artist_id.clone(), mean))
        })
        .collect();

    IndexCache {
        tracks_paths,

        artists_albums,
        artists_album_participations,
        artists_tracks,
        artists_track_participations,

        albums_tracks,

        albums_mean_score,
        artists_mean_score,
        albums_artists_mean_score,

        artists_infos: SortedMap::from_hashmap(artists_infos),
        albums_artists_infos: SortedMap::from_hashmap(albums_artists_infos),
        albums_infos: SortedMap::from_hashmap(albums_infos),
        genre_infos: SortedMap::from_hashmap(genres_infos),

        genres_albums,
        genres_tracks,
        no_genre_tracks,
    }
}
