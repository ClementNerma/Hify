use std::collections::{BTreeSet, HashMap, HashSet};

use log::debug;

use super::{
    AlbumID, AlbumInfos, ArtistID, ArtistInfos, GenreID, GenreInfos, Index, TrackID, TracksList,
    ValueOrdMap,
};

impl Index {
    // TODO: lots of optimization to perform here
    pub fn build(tracks: TracksList) -> Index {
        debug!("Building index cache...");

        let TracksList(tracks) = tracks;

        let tracks = ValueOrdMap::from_iter(tracks.into_iter().map(|track| (track.id, track)));

        let mut tracks_files_mtime = HashMap::new();

        let mut tracks_formats = HashMap::new();

        let mut artists_infos = HashMap::<ArtistID, ArtistInfos>::new();
        let mut album_artists_infos = HashMap::<ArtistID, ArtistInfos>::new();
        let mut artists_albums = HashMap::<ArtistID, BTreeSet<AlbumInfos>>::new();
        let mut artists_album_participations = HashMap::<ArtistID, BTreeSet<AlbumInfos>>::new();

        let mut artists_album_tracks = HashMap::<ArtistID, Vec<TrackID>>::new();
        let mut artists_track_participations = HashMap::<ArtistID, Vec<TrackID>>::new();
        let mut artists_tracks_and_participations = HashMap::<ArtistID, Vec<TrackID>>::new();

        let mut albums_infos = HashMap::<AlbumID, AlbumInfos>::new();
        let mut albums_tracks = HashMap::<AlbumID, Vec<TrackID>>::new();
        let mut albums_genres = HashMap::<AlbumID, Vec<GenreID>>::new();

        let mut genres_infos = HashMap::<GenreID, GenreInfos>::new();
        let mut genres_albums = HashMap::<GenreID, BTreeSet<AlbumInfos>>::new();
        let mut genres_tracks = HashMap::<GenreID, Vec<TrackID>>::new();
        let mut no_genre_tracks = vec![];

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

            for artist_infos in &album_artists {
                let artist_id = artist_infos.get_id();

                album_artists_infos.insert(artist_id, artist_infos.clone());

                artists_albums
                    .entry(artist_id)
                    .or_default()
                    .insert(album_infos.clone());

                artists_album_tracks
                    .entry(artist_id)
                    .or_default()
                    .push(track.id)
            }

            for artist_infos in &non_album_artists {
                let artist_id = artist_infos.get_id();

                artists_album_participations
                    .entry(artist_id)
                    .or_default()
                    .insert(album_infos.clone());

                artists_track_participations
                    .entry(artist_id)
                    .or_default()
                    .push(track.id)
            }

            for artist_infos in track_artists {
                let artist_id = artist_infos.get_id();

                artists_infos.insert(artist_id, artist_infos.clone());

                artists_tracks_and_participations
                    .entry(artist_id)
                    .or_default()
                    .push(track.id);
            }

            albums_tracks.entry(album_id).or_default().push(track.id);

            if track.metadata.tags.genres.is_empty() {
                no_genre_tracks.push(track.id);
            } else {
                for genre in tags.get_genres_infos() {
                    let genre_id = genre.get_id();

                    genres_infos.insert(genre_id, genre.clone());

                    genres_tracks.entry(genre_id).or_default().push(track.id);

                    genres_albums
                        .entry(genre_id)
                        .or_default()
                        .insert(album_infos.clone());

                    albums_genres.entry(album_id).or_default().push(genre_id);
                }
            }
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

        let artists_tracks_and_participations = artists_tracks_and_participations
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

        let albums_genres = albums_genres
            .into_iter()
            .map(|(album_id, mut genres)| {
                let mut seen = HashSet::new();
                genres.retain(|genre| seen.insert(*genre));
                (album_id, genres)
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

        debug!("| Building statistics...");

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

        Index {
            tracks,
            tracks_files_mtime,

            albums_infos: albums_infos.into_iter().collect(),
            albums_tracks,

            albums_genres,

            artists_infos: artists_infos.into_iter().collect(),
            album_artists_infos: album_artists_infos.into_iter().collect(),

            artists_albums,
            artists_album_participations,
            artists_albums_and_participations,

            artists_album_tracks,
            artists_track_participations,
            artists_tracks_and_participations,

            genres_infos: genres_infos.into_iter().collect(),
            genres_albums,
            genres_tracks,
            no_genre_tracks,

            most_recent_albums,
        }
    }
}
