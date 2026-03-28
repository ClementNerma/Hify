use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::Hash,
    path::PathBuf,
};

use indexmap::{IndexMap, IndexSet};

use crate::utils;

use super::{cmp::CmpIndex, content::*};

/// Index cache, used to accelerate requests by pre-computing some results once after index generation.
#[derive(Default)]
pub struct IndexCache {
    //
    // === TRACKS ===
    //
    /// List of all tracks, by ID
    pub tracks: IndexMap<TrackID, Track>,

    //
    // === ALBUMS ===
    //
    /// Informations about albums
    pub albums: IndexMap<AlbumID, Album>,

    /// Tracks belonging to an album
    pub albums_tracks: HashMap<AlbumID, IndexSet<TrackID>>,

    /// Genres of each album
    pub albums_genres: HashMap<AlbumID, IndexSet<GenreID>>,

    /// Common ancestor path to all tracks in an album
    pub albums_tracks_relative_common_path: HashMap<AlbumID, PathBuf>,

    /// Minimum and maximum track release date for each album
    pub albums_min_max_date: HashMap<AlbumID, Option<(TrackDate, TrackDate)>>,

    /// Album IDs sorted by their most recent track file's timestamp
    pub latest_added_albums: IndexSet<AlbumID>,

    //
    // === ARTISTS ===
    //
    /// Informations about artists
    pub artists: IndexMap<ArtistID, Artist>,

    /// Albums where the artist is listed in the album artists tag
    pub artists_albums: HashMap<ArtistID, IndexSet<AlbumID>>,

    /// Albums where the artist is listed in one of the tracks but not in the album artists tag
    pub artists_album_participations: HashMap<ArtistID, IndexSet<AlbumID>>,

    /// Tracks where the artist is listed in but belonging to an album they're not an album artist of
    pub artists_track_participations: HashMap<ArtistID, IndexSet<TrackID>>,

    //
    // === GENRES ===
    //
    /// Informations about genres
    pub genres: IndexMap<GenreID, Genre>,

    /// List of album for each genre
    pub genres_albums: HashMap<GenreID, IndexSet<AlbumID>>,

    /// List of tracks for each genre
    pub genres_tracks: HashMap<GenreID, IndexSet<TrackID>>,
}

impl IndexCache {
    // TODO: move assertions to a health check method on `Index`
    #[allow(clippy::too_many_lines)]
    pub fn build(index: &Index) -> Self {
        let cmp_index = CmpIndex::build(index);

        let Index {
            tracks,
            albums,
            artists,
            genres,
        } = index;

        let tracks = build_sorted_map(
            tracks.iter().cloned(),
            |track| track.id,
            |a, b| cmp_index.cmp_tracks(a, b),
        );

        let albums = build_sorted_map(
            albums.iter().cloned(),
            |album| album.id,
            |a, b| cmp_index.cmp_albums(a, b),
        );

        let artists = build_sorted_map(
            artists.iter().cloned(),
            |artist| artist.id,
            CmpIndex::cmp_artists,
        );

        let genres = build_sorted_map(
            genres.iter().cloned(),
            |genre| genre.id,
            CmpIndex::cmp_genres,
        );

        let mut artists_albums = HashMap::<ArtistID, HashSet<AlbumID>>::new();
        let mut artists_album_participations = HashMap::<ArtistID, HashSet<AlbumID>>::new();

        let mut artists_album_tracks = HashMap::<ArtistID, HashSet<TrackID>>::new();
        let mut artists_track_participations = HashMap::<ArtistID, HashSet<TrackID>>::new();

        let mut albums_tracks = HashMap::<AlbumID, HashSet<TrackID>>::new();
        let mut albums_genres = HashMap::<AlbumID, HashSet<GenreID>>::new();

        let mut genres_albums = HashMap::<GenreID, HashSet<AlbumID>>::new();
        let mut genres_tracks = HashMap::<GenreID, HashSet<TrackID>>::new();

        for album in albums.values() {
            for artist_id in &album.artists_id {
                assert!(artists.contains_key(artist_id));
            }
        }

        for track in tracks.values() {
            let album = albums.get(&track.tags.album_id).unwrap();

            assert!(albums.contains_key(&album.id));

            for artist_id in &album.artists_id {
                artists_albums
                    .entry(*artist_id)
                    .or_default()
                    .insert(album.id);

                artists_album_tracks
                    .entry(*artist_id)
                    .or_default()
                    .insert(track.id);

                assert!(artists.contains_key(artist_id));
            }

            for artist_id in track
                .tags
                .artists_id
                .union(&track.tags.composers_id)
                .filter(|artist_id| !album.artists_id.contains(*artist_id))
            {
                artists_album_participations
                    .entry(*artist_id)
                    .or_default()
                    .insert(album.id);

                artists_track_participations
                    .entry(*artist_id)
                    .or_default()
                    .insert(track.id);

                assert!(artists.contains_key(artist_id));
            }

            albums_tracks.entry(album.id).or_default().insert(track.id);

            for genre_id in &track.tags.genres_id {
                genres_tracks.entry(*genre_id).or_default().insert(track.id);
                genres_albums.entry(*genre_id).or_default().insert(album.id);
                albums_genres.entry(album.id).or_default().insert(*genre_id);

                assert!(genres.contains_key(genre_id));
            }
        }

        let cmp_tracks_by_id = |a: &TrackID, b: &TrackID| cmp_index.cmp_tracks_by_id(*a, *b);
        let cmp_albums_by_id = |a: &AlbumID, b: &AlbumID| cmp_index.cmp_albums_by_id(*a, *b);
        let cmp_genres_by_id = |a: &GenreID, b: &GenreID| cmp_index.cmp_genres_by_id(*a, *b);

        Self {
            latest_added_albums: to_sorted_set(albums.values().map(|album| album.id), |a, b| {
                let get_latest_mtime = |album_id: &AlbumID| {
                    let album_tracks = albums_tracks.get(album_id).unwrap();

                    album_tracks
                        .iter()
                        .map(|track_id| {
                            let track = tracks.get(track_id).unwrap();
                            track.file_times.ctime.unwrap_or(track.file_times.mtime)
                        })
                        .max()
                        .unwrap()
                };

                get_latest_mtime(a).cmp(&get_latest_mtime(b)).reverse()
            }),

            albums_min_max_date: albums
                .keys()
                .map(|album_id| {
                    let album_tracks = albums_tracks.get(album_id).unwrap();

                    let iter = album_tracks.iter().filter_map(|track_id| {
                        let track = tracks.get(track_id).unwrap();
                        track.tags.date
                    });

                    let min = iter.clone().min();

                    (*album_id, min.map(|min| (min, iter.max().unwrap())))
                })
                .collect(),

            albums_tracks_relative_common_path: albums_tracks
                .iter()
                .map(|(album_id, album_tracks)| {
                    let tracks_path = album_tracks
                        .iter()
                        .map(|track_id| &tracks.get(track_id).unwrap().relative_path);

                    (
                        *album_id,
                        if tracks_path.len() == 1 {
                            tracks_path
                                .into_iter()
                                .next()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .to_owned()
                        } else {
                            utils::common_ancestor(tracks_path).unwrap()
                        },
                    )
                })
                .collect(),

            albums_tracks: to_map_of_sorted_sets(
                albums_tracks,
                cmp_tracks_by_id,
                albums.keys().copied(),
            ),

            albums_genres: to_map_of_sorted_sets(
                albums_genres,
                cmp_genres_by_id,
                albums.keys().copied(),
            ),

            artists_albums: to_map_of_sorted_sets(
                artists_albums,
                cmp_albums_by_id,
                artists.keys().copied(),
            ),

            artists_album_participations: to_map_of_sorted_sets(
                artists_album_participations,
                cmp_albums_by_id,
                artists.keys().copied(),
            ),

            artists_track_participations: to_map_of_sorted_sets(
                artists_track_participations,
                cmp_tracks_by_id,
                artists.keys().copied(),
            ),

            genres_albums: to_map_of_sorted_sets(
                genres_albums,
                cmp_albums_by_id,
                genres.keys().copied(),
            ),

            genres_tracks: to_map_of_sorted_sets(genres_tracks, cmp_tracks_by_id, []),

            tracks,
            albums,
            artists,
            genres,
        }
    }
}

fn build_sorted_map<K: Hash + Eq, V>(
    values: impl IntoIterator<Item = V>,
    map_key: impl Fn(&V) -> K,
    sort_by_value: impl Fn(&V, &V) -> Ordering,
) -> IndexMap<K, V> {
    let mut values = values
        .into_iter()
        .map(|v| (map_key(&v), v))
        .collect::<IndexMap<_, _>>();

    values.sort_by(|_, a, _, b| {
        sort_by_value(a, b).then_with(|| panic!("No value should be equal in build_sorted_map"))
    });

    values
}

fn to_sorted_set<T: Hash + Eq>(
    values: impl IntoIterator<Item = T>,
    sort: impl Fn(&T, &T) -> Ordering,
) -> IndexSet<T> {
    let mut values = values.into_iter().collect::<IndexSet<_>>();

    values.sort_by(|a, b| {
        sort(a, b).then_with(|| panic!("No value should be equal in to_sorted_set"))
    });

    values
}

fn to_map_of_sorted_sets<K: Hash + Eq, V: Hash + Eq>(
    values: HashMap<K, HashSet<V>>,
    sort: impl Fn(&V, &V) -> Ordering,
    fill_with: impl IntoIterator<Item = K>,
) -> HashMap<K, IndexSet<V>> {
    let mut map = values
        .into_iter()
        .map(|(k, v)| (k, to_sorted_set(v, &sort)))
        .collect::<HashMap<_, _>>();

    for maybe_missing_key in fill_with {
        map.entry(maybe_missing_key).or_default();
    }

    map
}
