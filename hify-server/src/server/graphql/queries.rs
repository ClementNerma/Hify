use juniper::graphql_object;

use super::GraphQLContext;

pub struct QueryRoot;

#[graphql_object(context = GraphQLContext)]
impl QueryRoot {
    async fn index_fingerprint(ctx: &GraphQLContext) -> String {
        ctx.index.read().await.fingerprint.clone()
    }
}
