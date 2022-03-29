use juniper::graphql_object;

use crate::index::Index;

use super::graphql::GraphQLContext;

pub struct QueryRoot;

#[graphql_object(context = GraphQLContext)]
impl QueryRoot {
    async fn index(ctx: &GraphQLContext, fingerprint: String) -> Option<Index> {
        ctx.index
            .read()
            .await
            .clone()
            .filter(|index| index.creation_time != fingerprint)
    }
}
