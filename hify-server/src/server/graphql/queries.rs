use juniper::{graphql_object, FieldResult};

use crate::{
    graphql_into,
    index::{AudioFormat, Track, TrackDate},
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

    fn format(&self) -> AudioFormat {
        self.metadata.format
    }

    fn size(&self) -> i32 {
        self.metadata.size
    }

    fn duration(&self) -> f64 {
        self.metadata.duration
    }

    fn bitrate(&self) -> i32 {
        self.metadata.bitrate
    }

    fn title(&self) -> Option<&str> {
        self.metadata.tags.title.as_deref()
    }

    fn artist(&self) -> Option<&str> {
        self.metadata.tags.artist.as_deref()
    }

    fn composer(&self) -> Option<&str> {
        self.metadata.tags.composer.as_deref()
    }

    fn album(&self) -> Option<&str> {
        self.metadata.tags.album.as_deref()
    }

    fn album_artist(&self) -> Option<&str> {
        self.metadata.tags.album_artist.as_deref()
    }

    fn disc(&self) -> Option<i32> {
        self.metadata.tags.disc
    }

    fn track_no(&self) -> Option<i32> {
        self.metadata.tags.track_no
    }

    fn date(&self) -> Option<TrackDate> {
        self.metadata.tags.date
    }

    fn genre(&self) -> Option<&str> {
        self.metadata.tags.genre.as_deref()
    }
}
