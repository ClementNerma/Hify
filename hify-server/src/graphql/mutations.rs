use async_graphql::{Context, Object};

use super::{EmptyAnswer, GraphQLContext, EMPTY_ANSWER};
use crate::{
    graphql_ctx_member,
    index::{build_index, Index, Rating, TrackID},
    userdata::{OneListening, PlaylistID, PlaylistTracksAction},
};

pub struct MutationRoot;

#[Object]

impl MutationRoot {
    async fn update_index(&self, ctx: &Context<'_>) -> Result<EmptyAnswer, String> {
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

        Ok(EMPTY_ANSWER)
    }

    async fn log_listening(
        &self,
        ctx: &Context<'_>,
        track_id: TrackID,
        duration_s: u32,
    ) -> EmptyAnswer {
        graphql_ctx_member!(ctx, app_state.user_data, write)
            .log_listening(OneListening::new_now(track_id, duration_s));

        EMPTY_ANSWER
    }

    async fn set_track_rating(
        &self,
        ctx: &Context<'_>,
        track_id: TrackID,
        rating: Rating,
    ) -> EmptyAnswer {
        graphql_ctx_member!(ctx, app_state.user_data, write)
            .set_track_rating(track_id, Some(rating));

        EMPTY_ANSWER
    }

    async fn remove_track_rating(&self, ctx: &Context<'_>, track_id: TrackID) -> EmptyAnswer {
        graphql_ctx_member!(ctx, app_state.user_data, write).set_track_rating(track_id, None);

        EMPTY_ANSWER
    }

    async fn create_playlist(
        &self,
        ctx: &Context<'_>,
        name: String,
        tracks: Vec<TrackID>,
    ) -> PlaylistID {
        graphql_ctx_member!(ctx, app_state.user_data, write).create_playlist(name, tracks)
    }

    async fn edit_playlist(
        &self,
        ctx: &Context<'_>,
        playlist_id: PlaylistID,
        action: PlaylistTracksAction,
    ) -> Result<EmptyAnswer, &'static str> {
        graphql_ctx_member!(ctx, app_state.user_data, write)
            .edit_playlist(playlist_id, action)
            .map(|()| EMPTY_ANSWER)
    }

    async fn delete_playlist(
        &self,
        ctx: &Context<'_>,
        playlist_id: PlaylistID,
    ) -> Result<EmptyAnswer, &'static str> {
        graphql_ctx_member!(ctx, app_state.user_data, write)
            .delete_playlist(playlist_id)
            .map(|()| EMPTY_ANSWER)
    }
}
