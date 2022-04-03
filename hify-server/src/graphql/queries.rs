use async_graphql::{ComplexObject, Context, Enum, Object, Result};

use crate::{
    graphql_index,
    index::{AlbumID, AlbumInfos, ArtistID, ArtistInfos, SortedMap, Track, TrackID, TrackTags},
    transparent_cursor_type,
};

use super::{
    pagination::{paginate, Paginated, PaginationInput},
    GraphQLContext,
};

transparent_cursor_type!(TrackID, AlbumID, ArtistID);

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn fingerprint(&self, ctx: &Context<'_>) -> String {
        graphql_index!(ctx).fingerprint.clone()
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

    async fn tracks<'c>(
        &self,
        ctx: &Context<'_>,
        pagination: PaginationInput,
    ) -> Paginated<TrackID, Track> {
        let index = graphql_index!(ctx);
        paginate(pagination, &index.tracks, |track: &Track| track.id.clone())
    }

    async fn track(&self, ctx: &Context<'_>, id: String) -> Option<Track> {
        graphql_index!(ctx).tracks.get(&TrackID(id)).cloned()
    }
}

#[ComplexObject]
impl Track {
    async fn id(&self) -> &str {
        self.id.0.as_str()
    }
}

#[ComplexObject]
impl TrackTags {
    async fn album(&self) -> Option<AlbumInfos> {
        self.get_album_infos()
    }

    async fn artists(&self) -> Vec<ArtistInfos> {
        self.get_artists_infos().collect()
    }

    async fn album_artists(&self) -> Vec<ArtistInfos> {
        self.get_album_artists_infos().collect()
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

    async fn year(&self, ctx: &Context<'_>, strategy: AlbumYearStrategy) -> Option<i32> {
        let index = graphql_index!(ctx);
        let album_tracks = index.cache.albums_tracks.get(&self.get_id()).unwrap();
        let years: Vec<_> = album_tracks
            .iter()
            .filter_map(|track_id| index.tracks.get(track_id).unwrap().metadata.tags.date)
            .map(|date| date.year)
            .collect();

        let first_track_year = *years.get(0)?;

        if years.iter().all(|year| *year == first_track_year) {
            return Some(first_track_year);
        }

        match strategy {
            AlbumYearStrategy::IdenticalOnly => None,
            AlbumYearStrategy::IdenticalOrFirstTrack => Some(first_track_year),
            AlbumYearStrategy::IdenticalOrLowestYear => Some(*years.iter().min().unwrap()),
        }
    }

    async fn has_art_image(&self, ctx: &Context<'_>) -> bool {
        graphql_index!(ctx)
            .albums_arts
            .get(&self.get_id())
            .unwrap()
            .is_some()
    }
}

#[derive(Enum, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum AlbumYearStrategy {
    IdenticalOnly,
    IdenticalOrFirstTrack,
    IdenticalOrLowestYear,
}

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
                .albums_artists_albums
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
                .artists_albums
                .get(&self.get_id())
                .unwrap_or(&SortedMap::empty()),
            |album| album.get_id(),
        )
    }
}
