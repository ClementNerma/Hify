use juniper::graphql_object;

use crate::index::Library;

use super::graphql::GraphQLContext;

pub struct QueryRoot;

#[graphql_object(context = GraphQLContext)]
impl QueryRoot {
    async fn index(ctx: &GraphQLContext, fingerprint: String) -> Option<Library> {
        ctx.index
            .read()
            .await
            .clone()
            .filter(|index| index.creation_time != fingerprint)
    }
}
