use anyhow::Context;
use axum::{
    Router,
    extract::{Path, Query, State},
    routing::get,
};
use serde::Deserialize;

use crate::{
    index::{AlbumID, ArtistID, GenreID, TrackID},
    server::{
        HttpState,
        utils::{
            dtos::{
                AlbumCompleteInfos, ArtistCompleteInfos, GenreCompleteInfos, TrackCompleteInfos,
            },
            pagination::{Paginated, Pagination, PaginationDir},
            response::{ApiError, ApiResponse, ApiResult},
            sorting::{
                AlbumsSort, ArtistsSort, GenresSort, TracksSort, paginate_sort_albums,
                paginate_sort_artists, paginate_sort_genres, paginate_sort_tracks,
            },
        },
    },
};

#[rustfmt::skip]
pub fn router() -> Router<HttpState> {
    Router::new()
        .route("/ping", get(ping))
        .route("/albums", get(albums))
        .route("/album/{id}", get(album))
        .route("/album/{id}/tracks", get(album_tracks))
        .route("/artists", get(artists))
        // TODO: rename as it is inconsistent with "/album/{id}/with-tracks" which returns albums with their tracks
        .route("/artists/with-albums", get(artists_with_albums))
        .route("/artist/{id}", get(artist))
        .route("/artist/{id}/albums", get(artist_albums))
        .route("/artist/{id}/album-participations", get(artist_album_participations))
        .route("/artist/{id}/track-participations", get(artist_track_participations))
        .route("/tracks", get(tracks))
        .route("/tracks/multi", get(multi_tracks))
        .route("/track/{id}", get(track))
        .route("/genres", get(genres))
        .route("/genre/{id}", get(genre))
        .route("/genre/{id}/albums", get(genre_albums))
}

async fn ping(State(_): State<HttpState>) -> ApiResponse<&'static str> {
    ApiResponse("pong")
}

async fn artists(
    State(state): State<HttpState>,
    Query(query): Query<ArtistsQuery>,
) -> ApiResponse<Paginated<ArtistCompleteInfos>> {
    let ArtistsQuery {
        sort_by,
        limit,
        offset,
        dir,
    } = query;

    let index = state.index().await;

    ApiResponse(paginate_sort_artists(
        index.artists.values().collect(),
        sort_by,
        Pagination { limit, offset, dir },
        &index,
        &*state.ratings().await,
    ))
}

async fn artists_with_albums(
    State(state): State<HttpState>,
    Query(query): Query<ArtistsQuery>,
) -> ApiResponse<Paginated<ArtistCompleteInfos>> {
    let ArtistsQuery {
        sort_by,
        limit,
        offset,
        dir,
    } = query;

    let index = state.index().await;

    ApiResponse(paginate_sort_artists(
        index
            .artists_albums
            .iter()
            .filter(|(_, albums)| !albums.is_empty())
            .map(|(artist_id, _)| index.artists.get(artist_id).unwrap())
            .collect(),
        sort_by,
        Pagination { limit, offset, dir },
        &index,
        &*state.ratings().await,
    ))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ArtistsQuery {
    sort_by: ArtistsSort,

    // Cannot flatten [`Pagination`] because of axum limitations
    // See https://github.com/serde-rs/serde/issues/1183
    limit: usize,
    offset: Option<usize>,
    dir: PaginationDir,
}

async fn artist(
    State(state): State<HttpState>,
    Path(artist_id): Path<ArtistID>,
) -> ApiResult<ArtistCompleteInfos> {
    let index = state.index().await;

    let artist = index
        .artists
        .get(&artist_id)
        .context("Provided artist ID was not found")?;

    Ok(ApiResponse(ArtistCompleteInfos::new(
        artist.clone(),
        &index,
    )))
}

async fn artist_albums(
    State(state): State<HttpState>,
    Path(artist_id): Path<ArtistID>,
    Query(query): Query<ArtistAlbumsQuery>,
) -> ApiResult<Paginated<AlbumCompleteInfos>> {
    let ArtistAlbumsQuery {
        sort_by,
        limit,
        offset,
        dir,
    } = query;

    let index = state.index().await;

    let artist_albums = index
        .artists_albums
        .get(&artist_id)
        .context("Provided artist ID was not found")?;

    Ok(ApiResponse(paginate_sort_albums(
        artist_albums
            .iter()
            .map(|album_id| index.albums.get(album_id).unwrap())
            .collect(),
        sort_by,
        Pagination { limit, offset, dir },
        &index,
        &*state.ratings().await,
    )))
}

async fn artist_album_participations(
    State(state): State<HttpState>,
    Path(artist_id): Path<ArtistID>,
    Query(query): Query<ArtistAlbumsQuery>,
) -> ApiResult<Paginated<AlbumCompleteInfos>> {
    let ArtistAlbumsQuery {
        sort_by,
        limit,
        offset,
        dir,
    } = query;

    let index = state.index().await;

    let artist_album_participations = index
        .artists_album_participations
        .get(&artist_id)
        .context("Provided artist ID was not found")?;

    Ok(ApiResponse(paginate_sort_albums(
        artist_album_participations
            .iter()
            .map(|album_id| index.albums.get(album_id).unwrap())
            .collect(),
        sort_by,
        Pagination { limit, offset, dir },
        &index,
        &*state.ratings().await,
    )))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ArtistAlbumsQuery {
    sort_by: AlbumsSort,

    // Cannot flatten [`Pagination`] because of axum limitations
    // See https://github.com/serde-rs/serde/issues/1183
    limit: usize,
    offset: Option<usize>,
    dir: PaginationDir,
}

async fn artist_track_participations(
    State(state): State<HttpState>,
    Path(artist_id): Path<ArtistID>,
    Query(query): Query<ArtistTracksQuery>,
) -> ApiResult<Paginated<TrackCompleteInfos>> {
    let ArtistTracksQuery {
        sort_by,
        limit,
        offset,
        dir,
    } = query;

    let index = state.index().await;

    let artist_track_participations = index
        .artists_track_participations
        .get(&artist_id)
        .context("Provided artist ID was not found")?;

    Ok(ApiResponse(paginate_sort_tracks(
        artist_track_participations
            .iter()
            .map(|track_id| index.tracks.get(track_id).unwrap())
            .collect(),
        sort_by,
        Pagination { limit, offset, dir },
        &index,
        &*state.ratings().await,
    )))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ArtistTracksQuery {
    sort_by: TracksSort,

    // Cannot flatten [`Pagination`] because of axum limitations
    // See https://github.com/serde-rs/serde/issues/1183
    limit: usize,
    offset: Option<usize>,
    dir: PaginationDir,
}

async fn albums(
    State(state): State<HttpState>,
    Query(query): Query<AlbumsQuery>,
) -> ApiResponse<Paginated<AlbumCompleteInfos>> {
    let AlbumsQuery {
        sort_by,
        limit,
        offset,
        dir,
    } = query;

    let index = state.index().await;

    ApiResponse(paginate_sort_albums(
        index.albums.values().collect(),
        sort_by,
        Pagination { limit, offset, dir },
        &index,
        &*state.ratings().await,
    ))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AlbumsQuery {
    sort_by: AlbumsSort,

    // Cannot flatten [`Pagination`] because of axum limitations
    // See https://github.com/serde-rs/serde/issues/1183
    limit: usize,
    offset: Option<usize>,
    dir: PaginationDir,
}

async fn album(
    State(state): State<HttpState>,
    Path(album_id): Path<AlbumID>,
) -> ApiResult<AlbumCompleteInfos> {
    let index = state.index().await;

    let album = index
        .albums
        .get(&album_id)
        .context("Provided album ID was not found")?;

    Ok(ApiResponse(AlbumCompleteInfos::new(album.clone(), &index)))
}

async fn album_tracks(
    State(state): State<HttpState>,
    Path(album_id): Path<AlbumID>,
) -> ApiResult<Vec<TrackCompleteInfos>> {
    let index = state.index().await;

    if !index.albums.contains_key(&album_id) {
        return Err(ApiError::new("Provided album ID was not found"));
    }

    let ratings = state.ratings().await;

    Ok(ApiResponse(
        index
            .albums_tracks
            .get(&album_id)
            .unwrap()
            .iter()
            .map(|track_id| index.tracks.get(track_id).unwrap())
            .map(|track| TrackCompleteInfos::new(track.clone(), &index, &ratings))
            .collect(),
    ))
}

async fn tracks(
    State(state): State<HttpState>,
    Query(query): Query<TracksQuery>,
) -> ApiResponse<Paginated<TrackCompleteInfos>> {
    let TracksQuery {
        sort_by,
        limit,
        offset,
        dir,
    } = query;

    let index = state.index().await;
    let ratings = state.ratings().await;

    ApiResponse(paginate_sort_tracks(
        index.tracks.values().collect(),
        sort_by,
        Pagination { limit, offset, dir },
        &index,
        &ratings,
    ))
}

async fn multi_tracks(
    State(state): State<HttpState>,
    Query(query): Query<MultiTracksQuery>,
) -> ApiResult<Vec<TrackCompleteInfos>> {
    let MultiTracksQuery { ids } = query;

    // NOTE: this is necessary as Axum doesn't support decoding lists directly from query params
    let ids = serde_json::from_str::<Vec<TrackID>>(&ids)
        .context("Failed to decode the provided track IDs")?;

    let index = state.index().await;
    let ratings = state.ratings().await;

    let mut tracks = vec![];

    for track_id in ids {
        let track = index
            .tracks
            .get(&track_id)
            .with_context(|| format!("Provided track ID '{track_id:?}' was not found"))?;

        tracks.push(TrackCompleteInfos::new(track.clone(), &index, &ratings));
    }

    Ok(ApiResponse(tracks))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct MultiTracksQuery {
    ids: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct TracksQuery {
    sort_by: TracksSort,

    // Cannot flatten [`Pagination`] because of axum limitations
    // See https://github.com/serde-rs/serde/issues/1183
    limit: usize,
    offset: Option<usize>,
    dir: PaginationDir,
}

async fn track(
    State(state): State<HttpState>,
    Path(track_id): Path<TrackID>,
) -> ApiResult<TrackCompleteInfos> {
    let index = state.index().await;
    let ratings = state.ratings().await;

    let track = index
        .tracks
        .get(&track_id)
        .context("Provided track ID was not found")?;

    Ok(ApiResponse(TrackCompleteInfos::new(
        track.clone(),
        &index,
        &ratings,
    )))
}

async fn genres(
    State(state): State<HttpState>,
    Query(query): Query<GenresQuery>,
) -> ApiResponse<Paginated<GenreCompleteInfos>> {
    let GenresQuery {
        sort_by,
        limit,
        offset,
        dir,
    } = query;

    let index = state.index().await;
    let ratings = state.ratings().await;

    ApiResponse(paginate_sort_genres(
        index.genres.values().collect(),
        sort_by,
        Pagination { limit, offset, dir },
        &index,
        &ratings,
    ))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GenresQuery {
    sort_by: GenresSort,

    // Cannot flatten [`Pagination`] because of axum limitations
    // See https://github.com/serde-rs/serde/issues/1183
    limit: usize,
    offset: Option<usize>,
    dir: PaginationDir,
}

async fn genre(
    State(state): State<HttpState>,
    Path(genre_id): Path<GenreID>,
) -> ApiResult<GenreCompleteInfos> {
    let index = state.index().await;

    let genre = index
        .genres
        .get(&genre_id)
        .context("Provided genre ID was not found")?;

    Ok(ApiResponse(GenreCompleteInfos::new(genre.clone(), &index)))
}

async fn genre_albums(
    State(state): State<HttpState>,
    Path(genre_id): Path<GenreID>,
    Query(query): Query<GenreAlbumsQuery>,
) -> ApiResult<Paginated<AlbumCompleteInfos>> {
    let GenreAlbumsQuery {
        sort_by,
        limit,
        offset,
        dir,
    } = query;

    let index = state.index().await;

    let genre_albums = index
        .genres_albums
        .get(&genre_id)
        .context("Provided genre ID was not found")?;

    Ok(ApiResponse(paginate_sort_albums(
        genre_albums
            .iter()
            .map(|album_id| index.albums.get(album_id).unwrap())
            .collect(),
        sort_by,
        Pagination { limit, offset, dir },
        &index,
        &*state.ratings().await,
    )))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GenreAlbumsQuery {
    sort_by: AlbumsSort,

    // Cannot flatten [`Pagination`] because of axum limitations
    // See https://github.com/serde-rs/serde/issues/1183
    limit: usize,
    offset: Option<usize>,
    dir: PaginationDir,
}
