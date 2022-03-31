use async_graphql::{ComplexObject, Context, Object, Result};

use crate::{
    graphql_into,
    index::{AlbumID, ArtistID, Track, TrackID},
};

use super::{utils::GraphQLInto, GraphQLContext};

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
        let index = ctx.data::<GraphQLContext>().unwrap().index.read().await;
        index.fingerprint.clone()
    }

    async fn albums(&self, ctx: &Context<'_>, from: i32, take: i32) -> Result<Vec<AlbumID>> {
        let index = ctx.data::<GraphQLContext>().unwrap().index.read().await;
        let albums = index
            .cache
            .ordered_albums
            .iter()
            .skip(graphql_into!(from))
            .take(graphql_into!(take))
            .cloned()
            .collect();
        Ok(albums)
    }

    async fn album(&self, ctx: &Context<'_>, id: String) -> Result<Option<AlbumID>> {
        let index = ctx.data::<GraphQLContext>().unwrap().index.read().await;
        Ok(Some(AlbumID(id)).filter(|id| index.cache.albums_infos.contains_key(id)))
    }

    async fn artists(&self, ctx: &Context<'_>, from: i32, take: i32) -> Result<Vec<ArtistID>> {
        let index = ctx.data::<GraphQLContext>().unwrap().index.read().await;
        let artists = index
            .cache
            .ordered_artists
            .iter()
            .skip(graphql_into!(from))
            .take(graphql_into!(take))
            .cloned()
            .collect();
        Ok(artists)
    }

    async fn artist(&self, ctx: &Context<'_>, id: String) -> Result<Option<ArtistID>> {
        let index = ctx.data::<GraphQLContext>().unwrap().index.read().await;
        Ok(Some(ArtistID(id)).filter(|id| index.cache.artists_infos.contains_key(id)))
    }

    async fn album_artists(
        &self,
        ctx: &Context<'_>,
        from: i32,
        take: i32,
    ) -> Result<Vec<ArtistID>> {
        let index = ctx.data::<GraphQLContext>().unwrap().index.read().await;
        let artists = index
            .cache
            .ordered_albums_artists
            .iter()
            .skip(graphql_into!(from))
            .take(graphql_into!(take))
            .cloned()
            .collect();
        Ok(artists)
    }

    async fn tracks(&self, ctx: &Context<'_>, from: i32, take: i32) -> Result<Vec<Track>> {
        let index = ctx.data::<GraphQLContext>().unwrap().index.read().await;
        let tracks = index
            .tracks
            .iter()
            .skip(graphql_into!(from))
            .take(graphql_into!(take))
            .cloned()
            .collect();
        Ok(tracks)
    }

    async fn track(&self, ctx: &Context<'_>, id: String) -> Result<Option<Track>> {
        let index = ctx.data::<GraphQLContext>().unwrap().index.read().await;
        let track_index = index.cache.tracks_index.get(&TrackID(id));
        Ok(track_index.map(|track_index| index.tracks.get(*track_index).unwrap().clone()))
    }
}

#[ComplexObject]
impl Track {
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
        let index = ctx.data::<GraphQLContext>().unwrap().index.read().await;
        let track_index = index.cache.tracks_index.get(self).unwrap();
        index.tracks.get(*track_index).unwrap().clone()
    }
}

#[Object]
impl AlbumID {
    async fn id(&self) -> &str {
        &self.0
    }

    async fn name(&self, ctx: &Context<'_>) -> String {
        let index = ctx.data::<GraphQLContext>().unwrap().index.read().await;
        let album_infos = index.cache.albums_infos.get(self).unwrap();
        album_infos.name.clone()
    }

    async fn album_artists(&self, ctx: &Context<'_>) -> Vec<String> {
        let index = ctx.data::<GraphQLContext>().unwrap().index.read().await;
        let album_infos = index.cache.albums_infos.get(self).unwrap();
        album_infos.album_artists.clone()
    }

    async fn tracks(&self, ctx: &Context<'_>) -> Vec<TrackID> {
        let index = ctx.data::<GraphQLContext>().unwrap().index.read().await;
        let album_tracks = index.cache.albums_tracks.get(self).unwrap();
        album_tracks.iter().cloned().collect()
    }
}

#[Object]
impl ArtistID {
    async fn id(&self) -> &str {
        &self.0
    }

    async fn name(&self, ctx: &Context<'_>) -> String {
        let index = ctx.data::<GraphQLContext>().unwrap().index.read().await;
        let album_infos = index.cache.artists_infos.get(self).unwrap();
        album_infos.name.clone()
    }

    async fn albums(&self, ctx: &Context<'_>) -> Option<Vec<AlbumID>> {
        let index = ctx.data::<GraphQLContext>().unwrap().index.read().await;
        let albums_ids = index.cache.albums_artists_albums.get(self)?;
        Some(albums_ids.iter().cloned().collect())
    }

    async fn album_participations(&self, ctx: &Context<'_>) -> Option<Vec<AlbumID>> {
        let index = ctx.data::<GraphQLContext>().unwrap().index.read().await;
        let albums_ids = index.cache.artists_albums.get(self).unwrap();
        Some(albums_ids.iter().cloned().collect())
    }
}
