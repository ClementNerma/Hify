use std::collections::BTreeSet;

use async_graphql::{ComplexObject, Context, Object, SimpleObject};

use crate::{
    graphql_index,
    index::{
        AlbumID, AlbumInfos, Art, ArtTarget, ArtistID, ArtistInfos, GenreID, GenreInfos, SortedMap,
        Track, TrackID, TrackTags,
    },
};

use super::pagination::{paginate, paginate_mapped_slice, Paginated, PaginationInput};

#[derive(SimpleObject)]
pub struct IndexInfos {
    pub fingerprint: String,
    pub albums_count: usize,
    pub artists_count: usize,
    pub albums_artists_count: usize,
    pub tracks_count: usize,
}

#[ComplexObject]
impl Track {
    async fn id(&self) -> TrackID {
        self.id
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

    async fn art(&self, ctx: &Context<'_>) -> Option<Art> {
        graphql_index!(ctx)
            .arts
            .get(&ArtTarget::AlbumCover(self.get_id()).to_id())
            .cloned()
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

    async fn art(&self, ctx: &Context<'_>) -> Option<Art> {
        graphql_index!(ctx)
            .arts
            .get(&ArtTarget::Artist(self.get_id()).to_id())
            .cloned()
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
