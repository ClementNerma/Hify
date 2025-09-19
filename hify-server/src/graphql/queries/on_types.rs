use async_graphql::{ComplexObject, Context, Object};

use crate::{
    graphql_ctx, graphql_index, graphql_res_manager, graphql_user_data,
    index::{
        AlbumID, AlbumInfos, ArtistID, ArtistInfos, GenreID, GenreInfos, Rating, Track, TrackTags,
        ValueOrdMap,
    },
    userdata::{Playlist, PlaylistEntry},
};

use super::{
    super::pagination::{Paginated, PaginationInput, paginate, paginate_mapped_slice},
    TrackUsizeConnection, TrackUsizeEdge,
};

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
        let album_tracks = index.albums_tracks.get(&self.get_id()).unwrap();
        album_tracks
            .iter()
            .map(|track_id| index.tracks.get(track_id).unwrap().clone())
            .collect()
    }

    async fn year(&self, ctx: &Context<'_>) -> Option<u32> {
        let index = graphql_index!(ctx);
        let album_tracks = index.albums_tracks.get(&self.get_id()).unwrap();
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

    async fn genres(&self, ctx: &Context<'_>) -> Vec<GenreInfos> {
        let index = graphql_index!(ctx);

        let genres = index.albums_genres.get(&self.get_id()).unwrap();

        genres
            .iter()
            .map(|genre_id| index.genres_infos.get(genre_id).unwrap().clone())
            .collect::<Vec<_>>()
    }

    async fn has_art(&self, ctx: &Context<'_>) -> bool {
        graphql_res_manager!(ctx).album_arts.has(self.get_id())
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
                .artists_album_participations
                .get(&self.get_id())
                .unwrap_or(&ValueOrdMap::empty()),
            |album| album.get_id(),
        )
    }

    async fn album_tracks(
        &self,
        ctx: &Context<'_>,
        pagination: PaginationInput,
    ) -> Paginated<usize, Track, TrackUsizeConnection, TrackUsizeEdge> {
        let index = graphql_index!(ctx);

        let track_ids = index
            .artists_album_tracks
            .get(&self.get_id())
            .map(Vec::as_slice)
            .unwrap_or(&[]);

        paginate_mapped_slice(pagination, track_ids, |track| {
            index.tracks.get(track).unwrap().clone()
        })
    }

    async fn track_participations(
        &self,
        ctx: &Context<'_>,
        pagination: PaginationInput,
    ) -> Paginated<usize, Track, TrackUsizeConnection, TrackUsizeEdge> {
        let index = graphql_index!(ctx);

        let track_ids = index
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
            .artist_arts
            .has(self.get_id())
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
                .genres_albums
                .get(&self.get_id())
                .unwrap_or(&ValueOrdMap::empty()),
            |genre| genre.get_id(),
        )
    }

    async fn albums_count(&self, ctx: &Context<'_>) -> usize {
        graphql_index!(ctx)
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
