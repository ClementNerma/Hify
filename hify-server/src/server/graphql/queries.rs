use juniper::{graphql_object, FieldResult};

use crate::{
    graphql_into,
    index::{AlbumID, ArtistID, AudioFormat, Track, TrackDate, TrackID, TrackMetadata, TrackTags},
};

use super::{utils::GraphQLInto, GraphQLContext};

pub struct QueryRoot;

#[graphql_object(context = GraphQLContext)]
impl QueryRoot {
    async fn index() -> IndexGraph {
        IndexGraph
    }
}

pub struct IndexGraph;

#[graphql_object(context = GraphQLContext)]
impl IndexGraph {
    async fn fingerprint<'c>(&self, context: &'c GraphQLContext) -> String {
        let index = context.index.read().await;
        index.fingerprint.clone()
    }

    async fn albums(
        &self,
        context: &GraphQLContext,
        from: i32,
        take: i32,
    ) -> FieldResult<Vec<AlbumID>> {
        let index = context.index.read().await;
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

    async fn album(&self, context: &GraphQLContext, id: String) -> FieldResult<Option<AlbumID>> {
        let index = context.index.read().await;
        Ok(Some(AlbumID(id)).filter(|id| index.cache.albums_infos.contains_key(id)))
    }

    async fn artists(
        &self,
        context: &GraphQLContext,
        from: i32,
        take: i32,
    ) -> FieldResult<Vec<ArtistID>> {
        let index = context.index.read().await;
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

    async fn artist(&self, context: &GraphQLContext, id: String) -> FieldResult<Option<ArtistID>> {
        let index = context.index.read().await;
        Ok(Some(ArtistID(id)).filter(|id| index.cache.artists_infos.contains_key(id)))
    }

    async fn album_artists(
        &self,
        context: &GraphQLContext,
        from: i32,
        take: i32,
    ) -> FieldResult<Vec<ArtistID>> {
        let index = context.index.read().await;
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

    async fn tracks(
        &self,
        context: &GraphQLContext,
        from: i32,
        take: i32,
    ) -> FieldResult<Vec<Track>> {
        let index = context.index.read().await;
        let tracks = index
            .tracks
            .iter()
            .skip(graphql_into!(from))
            .take(graphql_into!(take))
            .cloned()
            .collect();
        Ok(tracks)
    }

    async fn track(&self, context: &GraphQLContext, id: String) -> FieldResult<Option<Track>> {
        let index = context.index.read().await;
        let track_index = index.cache.tracks_index.get(&TrackID(id));
        Ok(track_index.map(|track_index| index.tracks.get(*track_index).unwrap().clone()))
    }
}

#[graphql_object(context = GraphQLContext)]
impl Track {
    fn id(&self) -> &str {
        self.id.0.as_str()
    }

    fn path(&self) -> &str {
        &self.path
    }

    fn metadata(&self) -> &TrackMetadata {
        &self.metadata
    }

    fn tags(&self) -> &TrackTags {
        &self.metadata.tags
    }

    fn album(&self) -> Option<AlbumID> {
        self.metadata
            .tags
            .get_album_infos()
            .map(|infos| infos.get_id())
    }
}

#[graphql_object]
impl TrackMetadata {
    fn format(&self) -> AudioFormat {
        self.format
    }

    fn size(&self) -> i32 {
        self.size
    }

    fn duration(&self) -> f64 {
        self.duration
    }

    fn bitrate(&self) -> i32 {
        self.bitrate
    }
}

#[graphql_object]
impl TrackTags {
    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn artists(&self) -> &[String] {
        self.artists.as_slice()
    }

    fn composers(&self) -> &[String] {
        self.composers.as_slice()
    }

    fn album(&self) -> Option<&str> {
        self.album.as_deref()
    }

    fn album_artists(&self) -> &[String] {
        self.album_artists.as_slice()
    }

    fn disc(&self) -> Option<i32> {
        self.disc
    }

    fn track_no(&self) -> Option<i32> {
        self.track_no
    }

    fn date(&self) -> Option<TrackDate> {
        self.date
    }

    fn genre(&self) -> Option<&str> {
        self.genre.as_deref()
    }
}

#[graphql_object(context = GraphQLContext)]
impl TrackID {
    fn id(&self) -> &str {
        &self.0
    }

    async fn infos(&self, context: &GraphQLContext) -> Track {
        let index = context.index.read().await;
        let track_index = index.cache.tracks_index.get(self).unwrap();
        index.tracks.get(*track_index).unwrap().clone()
    }
}

#[graphql_object(context = GraphQLContext)]
impl AlbumID {
    fn id(&self) -> &str {
        &self.0
    }

    async fn name(&self, context: &GraphQLContext) -> String {
        let index = context.index.read().await;
        let album_infos = index.cache.albums_infos.get(self).unwrap();
        album_infos.name.clone()
    }

    async fn album_artists(&self, context: &GraphQLContext) -> Vec<String> {
        let index = context.index.read().await;
        let album_infos = index.cache.albums_infos.get(self).unwrap();
        album_infos.album_artists.clone()
    }

    async fn tracks(&self, context: &GraphQLContext) -> Vec<TrackID> {
        let index = context.index.read().await;
        let album_tracks = index.cache.albums_tracks.get(self).unwrap();
        album_tracks.iter().cloned().collect()
    }
}

#[graphql_object(context = GraphQLContext)]
impl ArtistID {
    fn id(&self) -> &str {
        &self.0
    }

    async fn name(&self, context: &GraphQLContext) -> String {
        let index = context.index.read().await;
        let album_infos = index.cache.artists_infos.get(self).unwrap();
        album_infos.name.clone()
    }

    async fn albums(&self, context: &GraphQLContext) -> Option<Vec<AlbumID>> {
        let index = context.index.read().await;
        let albums_ids = index.cache.albums_artists_albums.get(self)?;
        Some(albums_ids.iter().cloned().collect())
    }

    async fn album_participations(&self, context: &GraphQLContext) -> Option<Vec<AlbumID>> {
        let index = context.index.read().await;
        let albums_ids = index.cache.artists_albums.get(self).unwrap();
        Some(albums_ids.iter().cloned().collect())
    }
}
