use std::collections::BTreeSet;

use anyhow::{Context as _, Result};
use async_graphql::{ComplexObject, Context, Object, SimpleObject};

use crate::{
    graphql_ctx_member, graphql_index, graphql_user_data,
    index::{
        search_index, AlbumID, AlbumInfos, ArtistID, ArtistInfos, GenreID, GenreInfos,
        IndexSearchResults, SortedMap, Track, TrackID, TrackTags,
    },
    library::{
        feed::{self, Feed, FeedParams},
        mixer::{self, MixerParams},
    },
    transparent_cursor_type,
};

use super::{
    pagination::{paginate, paginate_mapped_slice, Paginated, PaginationInput},
    GraphQLContext,
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

    async fn history(&self, ctx: &Context<'_>) -> Vec<Track> {
        let index = graphql_index!(ctx);
        let user_data = graphql_user_data!(ctx);

        user_data
            .history()
            .iter()
            .filter_map(|id| index.tracks.get(id))
            .cloned()
            .collect()
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

    async fn album(&self, ctx: &Context<'_>, id: String) -> Option<AlbumInfos> {
        graphql_index!(ctx)
            .cache
            .albums_infos
            .get(&AlbumID(id))
            .cloned()
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

    async fn artist(&self, ctx: &Context<'_>, id: String) -> Option<ArtistInfos> {
        graphql_index!(ctx)
            .cache
            .artists_infos
            .get(&ArtistID(id))
            .cloned()
    }

    async fn genres(&self, ctx: &Context<'_>) -> Vec<GenreInfos> {
        graphql_index!(ctx)
            .cache
            .genre_infos
            .values()
            .cloned()
            .collect()
    }

    async fn genre(&self, ctx: &Context<'_>, id: String) -> Option<GenreInfos> {
        graphql_index!(ctx)
            .cache
            .genre_infos
            .get(&GenreID(id))
            .cloned()
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

    async fn select_tracks(&self, ctx: &Context<'_>, in_ids: Vec<String>) -> Result<Vec<Track>> {
        let index = graphql_index!(ctx);
        in_ids
            .into_iter()
            .map(|track_id| {
                index
                    .tracks
                    .get(&TrackID(track_id.clone()))
                    .cloned()
                    .with_context(|| format!("Track not found for ID: {}", track_id))
            })
            .collect::<Result<Vec<_>>>()
    }

    async fn track(&self, ctx: &Context<'_>, id: String) -> Option<Track> {
        graphql_index!(ctx).tracks.get(&TrackID(id)).cloned()
    }

    async fn search(
        &self,
        ctx: &Context<'_>,
        input: String,
        limit: usize,
    ) -> Result<IndexSearchResults> {
        let index = graphql_index!(ctx);
        let mut search_cache = graphql_ctx_member!(ctx, search_cache, write);

        Ok(search_index(&index, &mut search_cache, &input, limit))
    }

    async fn generate_feed(&self, ctx: &Context<'_>, input: FeedParams) -> Feed {
        feed::generate_feed(&*graphql_index!(ctx), &*graphql_user_data!(ctx), input)
    }

    async fn generate_mix(&self, ctx: &Context<'_>, input: MixerParams) -> Vec<Track> {
        mixer::generate_mix(&*graphql_index!(ctx), input)
    }
}

#[derive(SimpleObject)]
pub struct IndexInfos {
    fingerprint: String,
    albums_count: usize,
    artists_count: usize,
    albums_artists_count: usize,
    tracks_count: usize,
}

#[ComplexObject]
impl Track {
    async fn id(&self) -> &str {
        self.id.0.as_str()
    }
}

#[ComplexObject]
impl TrackTags {
    async fn album(&self) -> AlbumInfos {
        self.get_album_infos()
    }

    async fn artists(&self) -> Vec<ArtistInfos> {
        self.get_artists_infos().collect()
    }

    async fn album_artists(&self) -> Vec<ArtistInfos> {
        self.get_album_artists_infos().collect()
    }

    async fn genres(&self) -> Vec<GenreInfos> {
        self.get_genres_infos().collect()
    }
}

#[Object]
impl AlbumInfos {
    async fn id(&self) -> String {
        self.get_id().0
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn album_artists(&self) -> Vec<ArtistInfos> {
        self.album_artists.clone()
    }

    async fn tracks(&self, ctx: &Context<'_>) -> Vec<Track> {
        let index = graphql_index!(ctx);
        let album_tracks = index.cache.albums_tracks.get(&self.get_id()).unwrap();
        album_tracks
            .iter()
            .map(|track_id| index.tracks.get(track_id).unwrap().clone())
            .collect()
    }

    async fn year(&self, ctx: &Context<'_>) -> Option<u32> {
        let index = graphql_index!(ctx);
        let album_tracks = index.cache.albums_tracks.get(&self.get_id()).unwrap();
        let years: Vec<_> = album_tracks
            .iter()
            .filter_map(|track_id| index.tracks.get(track_id).unwrap().metadata.tags.date)
            .map(|date| date.year)
            .collect();

        let first_track_year = *years.first()?;

        Some(first_track_year)

        // TODO: make it configurable with *global* options
        //
        // if years.iter().all(|year| *year == first_track_year) {
        //     return Some(first_track_year);
        // }
        //
        // match strategy {
        //     AlbumYearStrategy::IdenticalOnly => None,
        //     AlbumYearStrategy::IdenticalOrFirstTrack => Some(first_track_year),
        //     AlbumYearStrategy::IdenticalOrLowestYear => Some(*years.iter().min().unwrap()),
        // }
    }

    async fn genres(&self, ctx: &Context<'_>) -> BTreeSet<GenreInfos> {
        let index = graphql_index!(ctx);
        let album_tracks = index.cache.albums_tracks.get(&self.get_id()).unwrap();
        album_tracks
            .iter()
            .flat_map(|track_id| {
                index
                    .tracks
                    .get(track_id)
                    .unwrap()
                    .metadata
                    .tags
                    .get_genres_infos()
            })
            .collect()
    }

    async fn has_art_image(&self, ctx: &Context<'_>) -> bool {
        graphql_index!(ctx)
            .albums_arts
            .get(&self.get_id())
            .unwrap()
            .is_some()
    }
}

// #[derive(Enum, Clone, Copy, PartialEq, Eq)]
// #[allow(clippy::enum_variant_names)]
// pub enum AlbumYearStrategy {
//     IdenticalOnly,
//     IdenticalOrFirstTrack,
//     IdenticalOrLowestYear,
// }

#[Object]
impl ArtistInfos {
    async fn id(&self) -> String {
        self.get_id().0
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn albums(
        &self,
        ctx: &Context<'_>,
        pagination: PaginationInput,
    ) -> Paginated<AlbumID, AlbumInfos> {
        let index = graphql_index!(ctx);
        paginate(
            pagination,
            index
                .cache
                .artists_albums
                .get(&self.get_id())
                .unwrap_or(&SortedMap::empty()),
            |album| album.get_id(),
        )
    }

    async fn album_participations(
        &self,
        ctx: &Context<'_>,
        pagination: PaginationInput,
    ) -> Paginated<AlbumID, AlbumInfos> {
        let index = graphql_index!(ctx);
        paginate(
            pagination,
            index
                .cache
                .artists_album_participations
                .get(&self.get_id())
                .unwrap_or(&SortedMap::empty()),
            |album| album.get_id(),
        )
    }

    async fn track_participations(
        &self,
        ctx: &Context<'_>,
        pagination: PaginationInput,
    ) -> Paginated<usize, Track> {
        let index = graphql_index!(ctx);

        let track_ids = index
            .cache
            .artists_track_participations
            .get(&self.get_id())
            .map(Vec::as_slice)
            .unwrap_or(&[]);

        paginate_mapped_slice(pagination, track_ids, |track| {
            index.tracks.get(track).unwrap().clone()
        })
    }
}

#[Object]
impl GenreInfos {
    async fn id(&self) -> String {
        self.get_id().0
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn albums(
        &self,
        ctx: &Context<'_>,
        pagination: PaginationInput,
    ) -> Paginated<AlbumID, AlbumInfos> {
        let index = graphql_index!(ctx);
        paginate(
            pagination,
            index
                .cache
                .genres_albums
                .get(&self.get_id())
                .unwrap_or(&SortedMap::empty()),
            |genre| genre.get_id(),
        )
    }

    async fn albums_count(&self, ctx: &Context<'_>) -> usize {
        graphql_index!(ctx)
            .cache
            .genres_albums
            .get(&self.get_id())
            .unwrap()
            .len()
    }
}

#[derive(SimpleObject)]
pub struct SearchResults {
    tracks: Vec<Track>,
    albums: Vec<AlbumInfos>,
    artists: Vec<ArtistInfos>,
}
