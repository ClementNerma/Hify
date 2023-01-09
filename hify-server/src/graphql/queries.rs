use anyhow::{Context as _, Result};
use async_graphql::{Context, Object};

use crate::{
    graphql_ctx_member, graphql_index, graphql_user_data,
    index::{
        search_index, AlbumID, AlbumInfos, ArtistID, ArtistInfos, GenreID, GenreInfos,
        IndexSearchResults, Track, TrackID,
    },
    library::{
        feed::{self, Feed, FeedParams},
        mixer::{self, MixParams},
    },
    transparent_cursor_type,
};

use super::{
    pagination::{paginate, paginate_mapped_slice, Paginated, PaginationInput},
    queries_types::*,
};

transparent_cursor_type!(TrackID, AlbumID, ArtistID, GenreID);

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn index_infos(&self, ctx: &Context<'_>) -> IndexInfos {
        let index = graphql_index!(ctx);

        IndexInfos {
            fingerprint: index.fingerprint.clone(),
            albums_count: index.cache.albums_infos.len(),
            albums_artists_count: index.cache.albums_artists_infos.len(),
            artists_count: index.cache.artists_infos.len(),
            tracks_count: index.tracks.len(),
        }
    }

    async fn history(
        &self,
        ctx: &Context<'_>,
        pagination: PaginationInput,
    ) -> Paginated<usize, Track> {
        let index = graphql_index!(ctx);
        let user_data = graphql_user_data!(ctx);

        paginate_mapped_slice(pagination, user_data.cache().dedup_history(), |entry| {
            index.tracks.get(&entry.track_id).unwrap().clone()
        })
    }

    async fn albums(
        &self,
        ctx: &Context<'_>,
        pagination: PaginationInput,
    ) -> Paginated<AlbumID, AlbumInfos> {
        let index = graphql_index!(ctx);
        paginate(
            pagination,
            &index.cache.albums_infos,
            |album: &AlbumInfos| album.get_id(),
        )
    }

    async fn album(&self, ctx: &Context<'_>, id: AlbumID) -> Option<AlbumInfos> {
        graphql_index!(ctx).cache.albums_infos.get(&id).cloned()
    }

    async fn artists(
        &self,
        ctx: &Context<'_>,
        pagination: PaginationInput,
    ) -> Paginated<ArtistID, ArtistInfos> {
        let index = graphql_index!(ctx);
        paginate(
            pagination,
            &index.cache.artists_infos,
            |artist: &ArtistInfos| artist.get_id(),
        )
    }

    async fn albums_artists(
        &self,
        ctx: &Context<'_>,
        pagination: PaginationInput,
    ) -> Paginated<ArtistID, ArtistInfos> {
        let index = graphql_index!(ctx);
        paginate(
            pagination,
            &index.cache.albums_artists_infos,
            |artist: &ArtistInfos| artist.get_id(),
        )
    }

    async fn artist(&self, ctx: &Context<'_>, id: ArtistID) -> Option<ArtistInfos> {
        graphql_index!(ctx).cache.artists_infos.get(&id).cloned()
    }

    async fn genres(&self, ctx: &Context<'_>) -> Vec<GenreInfos> {
        graphql_index!(ctx)
            .cache
            .genre_infos
            .values()
            .cloned()
            .collect()
    }

    async fn genre(&self, ctx: &Context<'_>, id: GenreID) -> Option<GenreInfos> {
        graphql_index!(ctx).cache.genre_infos.get(&id).cloned()
    }

    // Slow Waiting for answers on https://github.com/async-graphql/async-graphql/issues/1090
    // This will allow to use a `Paginated<TrackID, Track>` alongside the current `Paginated<usize, Track>`
    //
    // async fn tracks<'c>(
    //     &self,
    //     ctx: &Context<'_>,
    //     pagination: PaginationInput,
    // ) -> Paginated<TrackID, Track> {
    //     let index = graphql_index!(ctx);
    //     paginate(pagination, &index.tracks, |track: &Track| track.id.clone())
    // }

    async fn select_tracks(&self, ctx: &Context<'_>, in_ids: Vec<TrackID>) -> Result<Vec<Track>> {
        let index = graphql_index!(ctx);
        in_ids
            .into_iter()
            .map(|track_id| {
                index
                    .tracks
                    .get(&track_id)
                    .cloned()
                    .with_context(|| format!("Track not found for ID: {:?}", track_id))
            })
            .collect::<Result<Vec<_>>>()
    }

    async fn track(&self, ctx: &Context<'_>, id: TrackID) -> Option<Track> {
        graphql_index!(ctx).tracks.get(&id).cloned()
    }

    async fn search(
        &self,
        ctx: &Context<'_>,
        input: String,
        limit: usize,
    ) -> Result<IndexSearchResults> {
        let index = graphql_index!(ctx);
        let mut search_cache = graphql_ctx_member!(ctx, app_state.search_cache, write);

        Ok(search_index(&index, &mut search_cache, &input, limit))
    }

    async fn generate_feed(&self, ctx: &Context<'_>, input: FeedParams) -> Feed {
        feed::generate_feed(&*graphql_index!(ctx), &*graphql_user_data!(ctx), input)
    }

    async fn generate_mix(&self, ctx: &Context<'_>, input: MixParams) -> Vec<Track> {
        mixer::generate_mix(&*graphql_index!(ctx), input)
    }
}
