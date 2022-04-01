use async_graphql::{ComplexObject, Context, Object, Result};

use crate::{
    graphql_index,
    index::{AlbumID, AlbumInfos, ArtistID, ArtistInfos, Track, TrackID},
    transparent_cursor_type,
};

use super::{
    pagination::{paginate, Paginated, PaginationInput},
    GraphQLContext,
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn index(&self) -> IndexGraph {
        IndexGraph
    }
}

pub struct IndexGraph;

#[Object]
impl IndexGraph {
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

    async fn album(&self, ctx: &Context<'_>, id: String) -> Option<AlbumID> {
        let index = graphql_index!(ctx);
        Some(AlbumID(id)).filter(|id| index.cache.albums_infos.contains_key(id))
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

    async fn album_artists(
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
        let index = graphql_index!(ctx);
        index.cache.artists_infos.get(&ArtistID(id)).cloned()
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
        let index = graphql_index!(ctx);
        index.tracks.get(&TrackID(id)).cloned()
    }
}

#[ComplexObject]
impl Track {
    async fn id(&self) -> &str {
        self.id.0.as_str()
    }

    async fn album(&self) -> Option<AlbumID> {
        self.metadata
            .tags
            .get_album_infos()
            .map(|infos| infos.get_id())
    }
}

#[Object]
impl TrackID {
    async fn id(&self) -> &str {
        &self.0
    }

    async fn infos(&self, ctx: &Context<'_>) -> Track {
        let index = graphql_index!(ctx);
        index.tracks.get(self).cloned().unwrap()
    }
}

transparent_cursor_type!(TrackID);

#[Object]
impl AlbumID {
    async fn id(&self) -> &str {
        &self.0
    }

    async fn name(&self, ctx: &Context<'_>) -> String {
        let index = graphql_index!(ctx);
        let album_infos = index.cache.albums_infos.get(self).unwrap();
        album_infos.name.clone()
    }

    async fn album_artists(&self, ctx: &Context<'_>) -> Vec<ArtistInfos> {
        let index = graphql_index!(ctx);
        let album_infos = index.cache.albums_infos.get(self).unwrap();
        album_infos.album_artists.clone()
    }

    async fn tracks(&self, ctx: &Context<'_>) -> Vec<TrackID> {
        let index = graphql_index!(ctx);
        let album_tracks = index.cache.albums_tracks.get(self).unwrap();
        album_tracks.iter().cloned().collect()
    }
}

transparent_cursor_type!(AlbumID);

#[Object]
impl ArtistID {
    async fn id(&self) -> &str {
        &self.0
    }

    async fn name(&self, ctx: &Context<'_>) -> String {
        let index = graphql_index!(ctx);
        let album_infos = index.cache.artists_infos.get(self).unwrap();
        album_infos.name.clone()
    }

    // TODO: pagination
    async fn albums(&self, ctx: &Context<'_>) -> Option<Vec<AlbumInfos>> {
        let index = graphql_index!(ctx);
        let albums = index.cache.albums_artists_albums.get(self)?;
        Some(albums.iter().cloned().collect())
    }

    // TODO: pagination
    async fn album_participations(&self, ctx: &Context<'_>) -> Option<Vec<AlbumInfos>> {
        let index = graphql_index!(ctx);
        let albums = index.cache.artists_albums.get(self).unwrap();
        Some(albums.iter().cloned().collect())
    }
}

transparent_cursor_type!(ArtistID);

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
}

#[Object]
impl ArtistInfos {
    async fn id(&self) -> String {
        self.get_id().0
    }

    async fn name(&self) -> &str {
        &self.name
    }
}
