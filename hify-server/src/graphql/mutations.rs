use async_graphql::{Context, Object};

use super::GraphQLContext;
use crate::index::{build_index, TrackID};

pub struct MutationRoot;

#[Object]

impl MutationRoot {
    async fn generate_index(&self, ctx: &Context<'_>) -> bool {
        let ctx = ctx.data::<GraphQLContext>().unwrap();
        let index = build_index(ctx.app_state.index.read().await.from.clone());
        *ctx.app_state.index.write().await = index;
        true
    }

    async fn history_push(&self, ctx: &Context<'_>, track_id: String) -> bool {
        let ctx = ctx.data::<GraphQLContext>().unwrap();
        let mut user_data = ctx.app_state.user_data.write().await;
        user_data.history_push(TrackID(track_id));
        true
    }
}
