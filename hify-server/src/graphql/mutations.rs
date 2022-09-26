use async_graphql::{Context, Object};

use super::GraphQLContext;
use crate::index::{build_index, TrackID};

pub struct MutationRoot;

#[Object]

impl MutationRoot {
    async fn generate_index(&self, ctx: &Context<'_>) -> Result<bool, String> {
        let ctx = ctx.data::<GraphQLContext>().unwrap();

        // TODO
        build_index(ctx.app_state.index.read().await.from.clone(), None)
            .map_err(|err| format!("{err:?}"))?;

        Ok(true)
    }

    async fn history_push(&self, ctx: &Context<'_>, track_id: String) -> bool {
        let ctx = ctx.data::<GraphQLContext>().unwrap();
        let mut user_data = ctx.app_state.user_data.write().await;
        user_data.history_push(TrackID(track_id));
        true
    }
}
