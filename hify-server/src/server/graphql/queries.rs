use juniper::{graphql_object, FieldResult};

use crate::{graphql_into, index::Track};

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
}
