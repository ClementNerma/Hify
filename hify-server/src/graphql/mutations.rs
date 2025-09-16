use std::sync::Arc;

use async_graphql::{Context, Object, SimpleObject};

use super::{EmptyScalar, GraphQLContext};
use crate::{
    changes::detect_changes,
    graphql_ctx_member, graphql_index, graphql_user_data,
    index::{Rating, Track, TrackID},
    library::mixer::{self, MixParams},
    userdata::{MixID, OneListening, PlaylistEditAction, PlaylistID},
};

pub struct MutationRoot;

#[Object]

impl MutationRoot {
    async fn update_index(&self, ctx: &Context<'_>) -> Result<EmptyScalar, String> {
        let ctx = ctx.data::<GraphQLContext>().unwrap();
        let app_state = Arc::clone(&ctx.app_state);

        let index = tokio::task::spawn_blocking(move || {
            let index = app_state.index.blocking_read();
            let mut user_data = app_state.user_data.blocking_write();

            detect_changes(
                &app_state.music_dir,
                &mut user_data,
                &app_state.resource_manager,
                Some(&index),
            )
            .map_err(|err| format!("{err:?}"))
        })
        .await
        .map_err(|err| format!("Failed to analyze audio library: {err:?}"))?
        .map_err(|err| format!("Failed to analyze audio library: {err:?}"))?;

        // Update the index
        *ctx.app_state.index.write().await = index;

        // Clear the search cache
        ctx.search_cache.write().await.clear();

        Ok(EmptyScalar)
    }

    async fn log_listening(
        &self,
        ctx: &Context<'_>,
        track_id: TrackID,
        duration_s: u32,
    ) -> Result<EmptyScalar, String> {
        graphql_ctx_member!(ctx, app_state.user_data, write)
            .log_listening(OneListening::new_now(track_id, duration_s))
            .await
            .map(|()| EmptyScalar)
    }

    async fn set_track_rating(
        &self,
        ctx: &Context<'_>,
        track_id: TrackID,
        rating: Rating,
    ) -> Result<EmptyScalar, String> {
        graphql_ctx_member!(ctx, app_state.user_data, write)
            .set_track_rating(track_id, Some(rating))
            .await
            .map(|()| EmptyScalar)
    }

    async fn remove_track_rating(
        &self,
        ctx: &Context<'_>,
        track_id: TrackID,
    ) -> Result<EmptyScalar, String> {
        graphql_ctx_member!(ctx, app_state.user_data, write)
            .set_track_rating(track_id, None)
            .await
            .map(|()| EmptyScalar)
    }

    async fn create_playlist(
        &self,
        ctx: &Context<'_>,
        name: String,
        tracks: Vec<TrackID>,
    ) -> Result<PlaylistID, String> {
        graphql_ctx_member!(ctx, app_state.user_data, write)
            .create_playlist(name, tracks)
            .await
    }

    async fn edit_playlist(
        &self,
        ctx: &Context<'_>,
        playlist_id: PlaylistID,
        action: PlaylistEditAction,
    ) -> Result<EmptyScalar, String> {
        graphql_ctx_member!(ctx, app_state.user_data, write)
            .edit_playlist(playlist_id, action)
            .await
            .map(|()| EmptyScalar)
    }

    async fn delete_playlist(
        &self,
        ctx: &Context<'_>,
        playlist_id: PlaylistID,
    ) -> Result<EmptyScalar, String> {
        graphql_ctx_member!(ctx, app_state.user_data, write)
            .delete_playlist(playlist_id)
            .await
            .map(|()| EmptyScalar)
    }

    async fn generate_mix(
        &self,
        ctx: &Context<'_>,
        params: MixParams,
        max_tracks: usize,
    ) -> Result<CreatedMix, String> {
        let index = graphql_index!(ctx);

        let mut mix = mixer::generate_mix(&index, &*graphql_user_data!(ctx), params)?;

        let first_tracks = mix.next_tracks(max_tracks, |track_id| {
            index.tracks.get(&track_id).unwrap().clone()
        });

        let mix_id = mix.id();

        let mut user_data = graphql_ctx_member!(ctx, app_state.user_data, write);

        user_data.register_mix(mix).await?;

        Ok(CreatedMix {
            mix_id,
            first_tracks,
        })
    }

    async fn get_next_tracks_of_mix(
        &self,
        ctx: &Context<'_>,
        mix_id: MixID,
        max_tracks: usize,
    ) -> Result<Vec<Track>, String> {
        let index = graphql_index!(ctx);

        graphql_ctx_member!(ctx, app_state.user_data, write)
            .get_next_tracks_of_mix(mix_id, max_tracks, |track_id| {
                index.tracks.get(&track_id).unwrap().clone()
            })
            .await
    }

    async fn delete_mix(&self, ctx: &Context<'_>, mix_id: MixID) -> Result<EmptyScalar, String> {
        let mut user_data = graphql_ctx_member!(ctx, app_state.user_data, write);
        user_data.delete_mix(mix_id).await.map(|()| EmptyScalar)
    }
}

#[derive(SimpleObject)]
pub struct CreatedMix {
    mix_id: MixID,
    first_tracks: Vec<Track>,
}
