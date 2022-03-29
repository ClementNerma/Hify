use juniper::graphql_object;

use super::graphql::GraphQLContext;

pub struct QueryRoot;

#[graphql_object(context = GraphQLContext)]
impl QueryRoot {
    async fn index_fingerprint(ctx: &GraphQLContext) -> Option<String> {
        ctx.index
            .read()
            .await
            .as_ref()
            .map(|index| index.creation_time.clone())
    }
}
