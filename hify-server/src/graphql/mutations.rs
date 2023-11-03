use async_graphql::{Context, Object};

use super::GraphQLContext;
use crate::{
    graphql_ctx_member,
    index::{build_index, Index, Rating, TrackID},
    userdata::{OneListening, PlaylistID},
};

pub struct MutationRoot;

#[Object]

impl MutationRoot {
    async fn update_index(&self, ctx: &Context<'_>) -> Result<bool, String> {
        let ctx = ctx.data::<GraphQLContext>().unwrap();

        let current = Index::clone(&*ctx.app_state.index.read().await);

        let index = build_index(current.from.clone(), Some(current))
            .await
            .map_err(|err| format!("{err:?}"))?;

        (ctx.save_index)(&index)?;

        // Clear the serach cache
        ctx.app_state.search_cache.write().await.clear();

        // Cleanup user data (delete dangling data from removed tracks)
        ctx.app_state.user_data.write().await.cleanup(&index);

        // Update the index
        *ctx.app_state.index.write().await = index;

        Ok(true)
    }

    async fn log_listening(&self, ctx: &Context<'_>, track_id: TrackID, duration_s: u32) -> bool {
        graphql_ctx_member!(ctx, app_state.user_data, write)
            .log_listening(OneListening::new_now(track_id, duration_s));

        true
    }

    async fn set_track_rating(&self, ctx: &Context<'_>, track_id: TrackID, rating: Rating) -> bool {
        graphql_ctx_member!(ctx, app_state.user_data, write)
            .set_track_rating(track_id, Some(rating));

        true
    }

    async fn remove_track_rating(&self, ctx: &Context<'_>, track_id: TrackID) -> bool {
        graphql_ctx_member!(ctx, app_state.user_data, write).set_track_rating(track_id, None);

        true
    }

    async fn create_playlist(&self, ctx: &Context<'_>, name: String) -> PlaylistID {
        graphql_ctx_member!(ctx, app_state.user_data, write).create_playlist(name)
    }

    async fn add_track_to_playlist(
        &self,
        ctx: &Context<'_>,
        playlist_id: PlaylistID,
        track_id: TrackID,
        position: Option<usize>,
    ) -> Result<bool, &'static str> {
        graphql_ctx_member!(ctx, app_state.user_data, write)
            .add_track_to_playlist(playlist_id, track_id, position)
            .map(|()| true)
    }
}
