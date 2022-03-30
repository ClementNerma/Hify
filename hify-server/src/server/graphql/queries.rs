use juniper::{graphql_object, FieldResult};

use crate::{
    graphql_into,
    index::{AudioFormat, Track, TrackDate, TrackMetadata, TrackTags},
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

    async fn tracks<'c>(
        &self,
        context: &'c GraphQLContext,
        from: i32,
        take: i32,
    ) -> FieldResult<Vec<Track>> {
        let index = context.index.read().await;
        let tracks = index
            .tracks
            .iter()
            .skip(graphql_into!(from))
            .take(graphql_into!(take))
            .map(Track::clone)
            .collect();
        Ok(tracks)
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

    fn album(&self) -> Option<String> {
        self.metadata
            .tags
            .get_album_infos()
            .map(|infos| infos.get_id())
            .map(|id| id.0)
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

    fn artist(&self) -> Option<&str> {
        self.artist.as_deref()
    }

    fn composer(&self) -> Option<&str> {
        self.composer.as_deref()
    }

    fn album(&self) -> Option<&str> {
        self.album.as_deref()
    }

    fn album_artist(&self) -> Option<&str> {
        self.album_artist.as_deref()
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
