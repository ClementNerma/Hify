use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    index::{Album, Artist, CmpIndex, Genre, IndexCache, Rating, Track},
    manager::Ratings,
    server::utils::{
        dtos::{AlbumCompleteInfos, ArtistCompleteInfos, TrackCompleteInfos},
        pagination::{Paginated, Pagination},
    },
};

use super::dtos::GenreCompleteInfos;

pub fn paginate_sort_artists(
    mut artists: Vec<&Artist>,
    sort: ArtistsSort,
    pagination: Pagination,
    index: &IndexCache,
    ratings: &Ratings,
) -> Paginated<ArtistCompleteInfos> {
    match sort {
        ArtistsSort::Name => {
            artists.sort_by(|a, b| a.name.cmp(&b.name));
        }

        ArtistsSort::AlbumsCount => {
            // TODO: cmp_index
            artists.sort_by_key(|artist| index.artists_albums.get(&artist.id).unwrap().len());
        }

        ArtistsSort::TracksCount => {
            // TODO: cmp index
            artists.sort_by_key(|artist| {
                index
                    .artists_track_participations
                    .get(&artist.id)
                    .unwrap()
                    .len()
            });
        }

        ArtistsSort::GreatTracksCount => {
            // TODO: cmp_index
            artists.sort_by_key(|artist| {
                let artist_tracks = index.artists_track_participations.get(&artist.id).unwrap();

                artist_tracks
                    .iter()
                    .filter(|track_id| {
                        ratings
                            .get(track_id)
                            .is_some_and(|rating| *rating >= Rating::Four)
                    })
                    .count()
            });
        }
    }

    Paginated::paginate(artists.into_iter(), pagination)
        .map(|artist| ArtistCompleteInfos::new(artist.clone(), index))
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ArtistsSort {
    Name,
    AlbumsCount,
    TracksCount,
    GreatTracksCount,
}

pub fn paginate_sort_albums(
    mut albums: Vec<&Album>,
    sort: AlbumsSort,
    pagination: Pagination,
    index: &IndexCache,
    ratings: &Ratings,
) -> Paginated<AlbumCompleteInfos> {
    let cmp_index = CmpIndex::new(index);

    match sort {
        AlbumsSort::Name => {
            albums.sort_by(|a, b| cmp_index.cmp_albums(a, b));
        }

        AlbumsSort::Added => {
            let album_recent_pos = index
                .latest_added_albums
                .iter()
                .enumerate()
                .map(|(pos, album_id)| (*album_id, pos))
                .collect::<HashMap<_, _>>();

            albums.sort_by_key(|album| album_recent_pos.get(&album.id).unwrap());
            albums.reverse();
        }

        AlbumsSort::Date => {
            albums.sort_by(|a, b| {
                index
                    .albums_min_max_date
                    .get(&a.id)
                    .unwrap()
                    .map(|date| date.0)
                    .cmp(
                        &index
                            .albums_min_max_date
                            .get(&b.id)
                            .unwrap()
                            .map(|date| date.0),
                    )
                    .then_with(|| cmp_index.cmp_albums(a, b))
            });
        }

        AlbumsSort::TracksCount => {
            albums.sort_by(|a, b| {
                index
                    .albums_tracks
                    .get(&a.id)
                    .unwrap()
                    .len()
                    .cmp(&index.albums_tracks.get(&b.id).unwrap().len())
            });
        }

        AlbumsSort::Duration => {
            albums.sort_by_key(|album| {
                index
                    .albums_tracks
                    .get(&album.id)
                    .unwrap()
                    .iter()
                    .map(|track_id| {
                        u64::from(index.tracks.get(track_id).unwrap().metadata.duration_s)
                    })
                    .sum::<u64>()
            });
        }

        AlbumsSort::UnratedFirst => {
            let rated_tracks_count = |album_id| {
                let album_tracks = index.albums_tracks.get(&album_id).unwrap();

                album_tracks
                    .iter()
                    .filter(|track_id| ratings.get(track_id).is_some())
                    .count()
            };

            let unrated_tracks_count = |album_id| {
                let album_tracks = index.albums_tracks.get(&album_id).unwrap();

                album_tracks
                    .iter()
                    .filter(|track_id| ratings.get(track_id).is_none())
                    .count()
            };

            albums.sort_by(|a, b| {
                rated_tracks_count(a.id)
                    .cmp(&rated_tracks_count(b.id))
                    .then_with(|| {
                        unrated_tracks_count(a.id)
                            .cmp(&unrated_tracks_count(b.id))
                            .reverse()
                            .then_with(|| cmp_index.cmp_albums(a, b))
                    })
            });
        }

        AlbumsSort::RatedTracksCount => {
            let rated_tracks_count = |album_id| {
                let album_tracks = index.albums_tracks.get(&album_id).unwrap();

                album_tracks
                    .iter()
                    .filter(|track_id| ratings.get(track_id).is_some())
                    .count()
            };

            albums.sort_by(|a, b| {
                rated_tracks_count(a.id)
                    .cmp(&rated_tracks_count(b.id))
                    // TODO: then sort by album tracks count
                    .then_with(|| cmp_index.cmp_albums(a, b))
            });
        }

        AlbumsSort::BestTracksCount => {
            albums.sort_by_key(|album| {
                let album_tracks = index.albums_tracks.get(&album.id).unwrap();

                album_tracks
                    .iter()
                    .filter(|track_id| {
                        ratings
                            .get(track_id)
                            .is_some_and(|rating| *rating >= Rating::Four)
                    })
                    .count()
            });
        }
    }

    Paginated::paginate(albums.into_iter(), pagination)
        .map(|album| AlbumCompleteInfos::new(album.clone(), index))
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlbumsSort {
    Name,
    Added,
    Date,
    TracksCount,
    Duration,
    UnratedFirst,
    RatedTracksCount,
    BestTracksCount,
}

pub fn paginate_sort_tracks(
    mut tracks: Vec<&Track>,
    sort: TracksSort,
    pagination: Pagination,
    index: &IndexCache,
    ratings: &Ratings,
) -> Paginated<TrackCompleteInfos> {
    match sort {
        TracksSort::Title => {
            assert!(tracks.is_sorted_by(|a, b| a.tags.title <= b.tags.title));
        }

        TracksSort::Date => {
            tracks.sort_by_key(|track| track.file_times.mtime);
        }

        TracksSort::Duration => {
            tracks.sort_by_key(|track| index.tracks.get(&track.id).unwrap().metadata.duration_s);
        }

        TracksSort::UserRating => {
            tracks.sort_by_key(|track| ratings.get(&track.id));
        }
    }

    Paginated::paginate(tracks.into_iter(), pagination)
        .map(|track| TrackCompleteInfos::new(track.clone(), index, ratings))
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TracksSort {
    Date,
    Title,
    Duration,
    UserRating,
}

pub fn paginate_sort_genres(
    mut genres: Vec<&Genre>,
    sort: GenresSort,
    pagination: Pagination,
    index: &IndexCache,
    ratings: &Ratings,
) -> Paginated<GenreCompleteInfos> {
    match sort {
        GenresSort::Name => {
            genres.sort_by(|a, b| a.name.cmp(&b.name));
        }

        GenresSort::AlbumsCount => {
            // TODO: cmp_index
            genres.sort_by_key(|genre| index.genres_albums.get(&genre.id).unwrap().len());
        }

        GenresSort::TracksCount => {
            // TODO: cmp_index
            genres.sort_by_key(|genre| index.genres_tracks.get(&genre.id).unwrap().len());
        }

        GenresSort::GreatTracksCount => {
            // TODO: cmp_index
            genres.sort_by_key(|genre| {
                let genre_tracks = index.genres_tracks.get(&genre.id).unwrap();

                genre_tracks
                    .iter()
                    .filter(|track_id| {
                        ratings
                            .get(track_id)
                            .is_some_and(|rating| *rating >= Rating::Four)
                    })
                    .count()
            });
        }
    }

    Paginated::paginate(genres.into_iter(), pagination)
        .map(|genre| GenreCompleteInfos::new(genre.clone(), index))
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GenresSort {
    Name,
    AlbumsCount,
    TracksCount,
    GreatTracksCount,
}
