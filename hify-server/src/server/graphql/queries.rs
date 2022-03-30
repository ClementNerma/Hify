use juniper::graphql_object;

use super::GraphQLContext;

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
        context.index.read().await.fingerprint.clone()
    }
}
