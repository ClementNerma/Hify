use std::collections::{BTreeSet, HashMap, HashSet};

use log::debug;

use super::{
    AlbumID, AlbumInfos, ArtistID, ArtistInfos, GenreID, GenreInfos, IndexCache, Track, TrackID,
    ValueOrdMap,
};

// TODO: lots of optimization to perform here
pub fn build_index_cache(tracks: &ValueOrdMap<TrackID, Track>) -> IndexCache {
    debug!("Building index cache...");

    let mut tracks_files_mtime = HashMap::new();

    let mut tracks_formats = HashMap::new();
    let mut tracks_all_artists = HashMap::new();

    let mut artists_albums = HashMap::<ArtistID, BTreeSet<AlbumInfos>>::new();
    let mut artists_album_participations = HashMap::<ArtistID, BTreeSet<AlbumInfos>>::new();
    let mut artists_track_participations = HashMap::<ArtistID, Vec<TrackID>>::new();
    let mut artists_tracks = HashMap::<ArtistID, Vec<TrackID>>::new();
    let mut albums_tracks = HashMap::<AlbumID, Vec<TrackID>>::new();

    let mut artists_infos = HashMap::<ArtistID, ArtistInfos>::new();
    let mut album_artists_infos = HashMap::<ArtistID, ArtistInfos>::new();
    let mut albums_infos = HashMap::<AlbumID, AlbumInfos>::new();

    let mut genres_infos = HashMap::<GenreID, GenreInfos>::new();
    let mut genres_albums = HashMap::<GenreID, BTreeSet<AlbumInfos>>::new();
    let mut genres_tracks = HashMap::<GenreID, Vec<TrackID>>::new();
    let mut no_genre_tracks = HashSet::new();

    debug!("| Building tracks cache...");

    for track in tracks.values() {
        tracks_files_mtime.insert(track.relative_path.clone(), track.mtime);
        tracks_formats.insert(track.id, track.metadata.codec);

        let tags = &track.metadata.tags;

        let album_infos = tags.get_album_infos();
        let album_id = album_infos.get_id();

        albums_infos.insert(album_id, album_infos.clone());

        let album_artists: HashSet<_> = tags.get_album_artists_infos().collect();

        let track_artists = tags.get_artists_infos().collect::<HashSet<_>>();
        let non_album_artists = &track_artists - &album_artists;

        for album_artist_infos in &album_artists {
            album_artists_infos.insert(album_artist_infos.get_id(), album_artist_infos.clone());

            artists_albums
                .entry(album_artist_infos.get_id())
                .or_default()
                .insert(album_infos.clone());
        }

        for non_album_artist_infos in &non_album_artists {
            let artist_id = non_album_artist_infos.get_id();

            artists_album_participations
                .entry(artist_id)
                .or_default()
                .insert(album_infos.clone());

            artists_track_participations
                .entry(artist_id)
                .or_default()
                .push(track.id)
        }

        for artist_infos in album_artists.iter().chain(non_album_artists.iter()) {
            let artist_id = artist_infos.get_id();

            artists_infos.insert(artist_id, artist_infos.clone());

            artists_tracks.entry(artist_id).or_default().push(track.id);
        }

        albums_tracks.entry(album_id).or_default().push(track.id);

        if track.metadata.tags.genres.is_empty() {
            no_genre_tracks.insert(track.id);
        } else {
            for genre in tags.get_genres_infos() {
                let genre_id = genre.get_id();

                genres_infos.insert(genre_id, genre.clone());

                genres_tracks.entry(genre_id).or_default().push(track.id);

                genres_albums
                    .entry(genre_id)
                    .or_default()
                    .insert(album_infos.clone());
            }
        }

        tracks_all_artists.insert(
            track.id,
            album_artists
                .iter()
                .chain(non_album_artists.iter())
                .map(ArtistInfos::get_id)
                .collect(),
        );
    }

    debug!("| Building maps...");

    let artists_album_participations = artists_album_participations
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                v.into_iter()
                    .map(|album| (album.get_id(), album))
                    .collect::<ValueOrdMap<_, _>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    let artists_albums = artists_albums
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                v.into_iter()
                    .map(|album| (album.get_id(), album))
                    .collect::<ValueOrdMap<_, _>>(),
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
                v.into_iter().map(|album| (album.get_id(), album)).collect(),
            )
        })
        .collect();

    let artists_albums_and_participations = artists_infos
        .keys()
        .map(|artist_id| {
            (
                *artist_id,
                artists_albums
                    .get(artist_id)
                    .unwrap_or(&ValueOrdMap::empty())
                    .iter()
                    .chain(
                        artists_album_participations
                            .get(artist_id)
                            .unwrap_or(&ValueOrdMap::empty())
                            .iter(),
                    )
                    .map(|(album_id, album_infos)| (*album_id, album_infos.clone()))
                    .collect::<ValueOrdMap<_, _>>(),
            )
        })
        .collect();

    let artists_tracks_and_participations = artists_infos
        .keys()
        .map(|artist_id| {
            let mut all_tracks = match artists_tracks.get(artist_id) {
                Some(tracks) => tracks.clone(),
                None => vec![],
            };

            if let Some(tracks) = artists_track_participations.get(artist_id) {
                all_tracks.extend(tracks.iter().copied());
            }

            (*artist_id, all_tracks)
        })
        .collect();

    debug!("| Building statistics...");

    debug!("| > Albums' mean score...");

    let albums_mean_score = albums_tracks
        .iter()
        .filter_map(|(album_id, album_tracks)| {
            let rated_tracks: Vec<_> = album_tracks
                .iter()
                .filter_map(|track_id| tracks.get(track_id).unwrap().metadata.tags.rating)
                .map(|rating| rating.value() as f64)
                .collect();

            if rated_tracks.is_empty() {
                return None;
            }

            let mean = rated_tracks.iter().sum::<f64>() / (rated_tracks.len() as f64);
            Some((*album_id, mean))
        })
        .collect();

    debug!("| > Artists' mean score...");

    let artists_mean_score = artists_tracks
        .iter()
        .filter_map(|(artist_id, artist_tracks)| {
            let rated_tracks: Vec<_> = artist_tracks
                .iter()
                .filter_map(|track_id| tracks.get(track_id).unwrap().metadata.tags.rating)
                .map(|rating| rating.value() as f64)
                .collect();

            if rated_tracks.is_empty() {
                return None;
            }

            let mean = rated_tracks.iter().sum::<f64>() / (rated_tracks.len() as f64);
            Some((*artist_id, mean))
        })
        .collect();

    debug!("| > Album artists' mean score...");

    let album_artists_mean_score = artists_albums
        .iter()
        .filter_map(|(artist_id, artist_albums)| {
            let rated_tracks: Vec<_> = artist_albums
                .keys()
                .flat_map(|album_id| albums_tracks.get(album_id).unwrap())
                .filter_map(|track_id| tracks.get(track_id).unwrap().metadata.tags.rating)
                .map(|rating| rating.value() as f64)
                .collect();

            if rated_tracks.is_empty() {
                return None;
            }

            let mean = rated_tracks.iter().sum::<f64>() / (rated_tracks.len() as f64);
            Some((*artist_id, mean))
        })
        .collect();

    debug!("| > Most recent albums...");

    let mut most_recent_albums = albums_infos.keys().cloned().collect::<Vec<_>>();

    most_recent_albums.sort_by_key(|album_id| {
        let most_recent_track = albums_tracks
            .get(album_id)
            .unwrap()
            .iter()
            .map(|track_id| tracks.get(track_id).unwrap())
            // NOTE: We use the minimum here to avoid a typical problem where
            //       some tracks are updated manually and mess with the whole ordering.
            // So instead the lowest (usually common) time of all tracks to determine
            // the album's one.
            .min_by_key(|track| track.ctime.unwrap_or(track.mtime))
            .unwrap();

        most_recent_track.ctime.unwrap_or(most_recent_track.mtime)
    });

    // Reverse to get the most recent albums first
    most_recent_albums.reverse();

    debug!("| Done.");

    IndexCache {
        tracks_files_mtime,
        tracks_all_artists,

        artists_albums,
        artists_album_participations,
        artists_albums_and_participations,

        artists_tracks,
        artists_track_participations,
        artists_tracks_and_participations,

        albums_tracks,

        albums_mean_score,
        artists_mean_score,
        album_artists_mean_score,

        artists_infos: artists_infos.into_iter().collect(),
        album_artists_infos: album_artists_infos.into_iter().collect(),
        albums_infos: albums_infos.into_iter().collect(),
        genre_infos: genres_infos.into_iter().collect(),

        genres_albums,
        genres_tracks,
        no_genre_tracks,

        most_recent_albums,
    }
}
