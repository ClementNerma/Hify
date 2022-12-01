use async_graphql::{Context, Object};

use super::GraphQLContext;
use crate::{
    index::{build_index, Index, TrackID},
    userdata::OneListening,
};

pub struct MutationRoot;

#[Object]

impl MutationRoot {
    async fn update_index(&self, ctx: &Context<'_>) -> Result<bool, String> {
        let ctx = ctx.data::<GraphQLContext>().unwrap();

        let current = Index::clone(&*ctx.app_state.index.read().await);

        let index =
            build_index(current.from.clone(), Some(current)).map_err(|err| format!("{err:?}"))?;

        (ctx.save_index)(&index)?;

        ctx.app_state.user_data.write().await.cleanup(&index);

        *ctx.app_state.index.write().await = index;

        Ok(true)
    }

    async fn log_listening(&self, ctx: &Context<'_>, track_id: TrackID, duration_s: u32) -> bool {
        let ctx = ctx.data::<GraphQLContext>().unwrap();
        let mut user_data = ctx.app_state.user_data.write().await;
        user_data.log_listening(OneListening::new_now(track_id, duration_s));
        true
    }
}
