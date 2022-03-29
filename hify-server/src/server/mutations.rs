use juniper::graphql_object;

use crate::{builder::build_index, index::Library};

use super::graphql::GraphQLContext;

pub struct MutationRoot;

#[graphql_object(context = GraphQLContext)]

impl MutationRoot {
    async fn generate_index(ctx: &mut GraphQLContext) -> Library {
        let index = build_index(&ctx.root_path);
        *ctx.index.write().await = Some(index.clone());
        index
    }
}
