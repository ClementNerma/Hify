use std::collections::BTreeSet;

use async_graphql::{ComplexObject, Context, Object, SimpleObject};

use crate::{
    graphql_ctx, graphql_index, graphql_user_data,
    index::{
        AlbumID, AlbumInfos, ArtistID, ArtistInfos, GenreID, GenreInfos, Rating, Track, TrackTags,
        ValueOrdMap,
    },
    resources::ArtistArt,
    userdata::{Playlist, PlaylistEntry},
};

use super::{
    super::pagination::{paginate, paginate_mapped_slice, Paginated, PaginationInput},
    TrackUsizeConnection, TrackUsizeEdge,
};

#[derive(SimpleObject)]
pub struct IndexInfos {
    pub fingerprint: String,
    pub albums_count: usize,
    pub artists_count: usize,
    pub album_artists_count: usize,
    pub tracks_count: usize,
}

#[ComplexObject]
impl Track {
    async fn app_only_rating(&self, ctx: &Context<'_>) -> Option<Rating> {
        graphql_user_data!(ctx)
            .track_ratings()
            .get(&self.id)
            .copied()
    }

    async fn computed_rating(&self, ctx: &Context<'_>) -> Option<Rating> {
        graphql_user_data!(ctx)
            .track_ratings()
            .get(&self.id)
            .copied()
            .or(self.metadata.tags.rating)
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
    async fn id(&self) -> AlbumID {
        self.get_id()
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

    async fn has_art(&self, ctx: &Context<'_>) -> bool {
        graphql_index!(ctx).album_arts.contains_key(&self.get_id())
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
    async fn id(&self) -> ArtistID {
        self.get_id()
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
                .unwrap_or(&ValueOrdMap::empty()),
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
                .unwrap_or(&ValueOrdMap::empty()),
            |album| album.get_id(),
        )
    }

    async fn track_participations(
        &self,
        ctx: &Context<'_>,
        pagination: PaginationInput,
    ) -> Paginated<usize, Track, TrackUsizeConnection, TrackUsizeEdge> {
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

    async fn all_tracks(
        &self,
        ctx: &Context<'_>,
        pagination: PaginationInput,
    ) -> Paginated<usize, Track, TrackUsizeConnection, TrackUsizeEdge> {
        let index = graphql_index!(ctx);

        let track_ids = index
            .cache
            .artists_tracks_and_participations
            .get(&self.get_id())
            .map(Vec::as_slice)
            .unwrap_or(&[]);

        paginate_mapped_slice(pagination, track_ids, |track| {
            index.tracks.get(track).unwrap().clone()
        })
    }

    async fn has_art(&self, ctx: &Context<'_>) -> bool {
        graphql_ctx!(ctx)
            .app_state
            .resource_manager
            .has::<ArtistArt>(self.get_id())
    }
}

#[Object]
impl GenreInfos {
    async fn id(&self) -> GenreID {
        self.get_id()
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
                .unwrap_or(&ValueOrdMap::empty()),
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

#[ComplexObject]
impl Playlist {
    async fn entries(&self, pagination: PaginationInput) -> Paginated<usize, PlaylistEntry> {
        paginate_mapped_slice(pagination, &self.entries, |entry| *entry)
    }

    async fn entries_count(&self) -> usize {
        self.entries.len()
    }
}

#[ComplexObject]
impl PlaylistEntry {
    async fn track(&self, ctx: &Context<'_>) -> Track {
        graphql_index!(ctx)
            .tracks
            .get(&self.track_id)
            .unwrap()
            .clone()
    }
}

#[derive(SimpleObject)]
pub struct SearchResults {
    tracks: Vec<Track>,
    albums: Vec<AlbumInfos>,
    artists: Vec<ArtistInfos>,
}
