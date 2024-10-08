use async_graphql::{Context, Object, SimpleObject};

use super::{EmptyScalar, GraphQLContext};
use crate::{
    graphql_ctx_member, graphql_index, graphql_user_data,
    index::{build_index, Index, Rating, Track, TrackID},
    library::mixer::{self, MixParams},
    userdata::{MixID, OneListening, PlaylistEditAction, PlaylistID},
};

pub struct MutationRoot;

#[Object]

impl MutationRoot {
    async fn update_index(&self, ctx: &Context<'_>) -> Result<EmptyScalar, String> {
        let ctx = ctx.data::<GraphQLContext>().unwrap();

        let current = Index::clone(&*ctx.app_state.index.read().await);

        let index = build_index(
            current.from.clone(),
            Some(current),
            &ctx.app_state.resource_manager,
        )
        .await
        .map_err(|err| format!("{err:?}"))?;

        (ctx.save_index)(&index)?;

        // Clear the serach cache
        ctx.search_cache.write().await.clear();

        // Cleanup user data (delete dangling data from removed tracks)
        ctx.app_state.user_data.write().await.cleanup(&index);

        // Update the index
        *ctx.app_state.index.write().await = index;

        Ok(EmptyScalar)
    }

    async fn log_listening(
        &self,
        ctx: &Context<'_>,
        track_id: TrackID,
        duration_s: u32,
    ) -> EmptyScalar {
        graphql_ctx_member!(ctx, app_state.user_data, write)
            .log_listening(OneListening::new_now(track_id, duration_s));

        EmptyScalar
    }

    async fn set_track_rating(
        &self,
        ctx: &Context<'_>,
        track_id: TrackID,
        rating: Rating,
    ) -> EmptyScalar {
        graphql_ctx_member!(ctx, app_state.user_data, write)
            .set_track_rating(track_id, Some(rating));

        EmptyScalar
    }

    async fn remove_track_rating(&self, ctx: &Context<'_>, track_id: TrackID) -> EmptyScalar {
        graphql_ctx_member!(ctx, app_state.user_data, write).set_track_rating(track_id, None);

        EmptyScalar
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
        action: PlaylistEditAction,
    ) -> Result<EmptyScalar, &'static str> {
        graphql_ctx_member!(ctx, app_state.user_data, write)
            .edit_playlist(playlist_id, action)
            .map(|()| EmptyScalar)
    }

    async fn delete_playlist(
        &self,
        ctx: &Context<'_>,
        playlist_id: PlaylistID,
    ) -> Result<EmptyScalar, &'static str> {
        graphql_ctx_member!(ctx, app_state.user_data, write)
            .delete_playlist(playlist_id)
            .map(|()| EmptyScalar)
    }

    async fn generate_mix(
        &self,
        ctx: &Context<'_>,
        params: MixParams,
        max_tracks: usize,
    ) -> Result<CreatedMix, &'static str> {
        let index = graphql_index!(ctx);

        let mut mix = mixer::generate_mix(&index, &*graphql_user_data!(ctx), params)?;

        let first_tracks = mix.next_tracks(max_tracks, |track_id| {
            index.tracks.get(&track_id).unwrap().clone()
        });

        let mix_id = mix.id();

        graphql_ctx_member!(ctx, app_state.user_data, write).register_mix(mix);

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
    ) -> Result<Vec<Track>, &'static str> {
        let index = graphql_index!(ctx);

        graphql_ctx_member!(ctx, app_state.user_data, write).get_next_tracks_of_mix(
            mix_id,
            max_tracks,
            |track_id| index.tracks.get(&track_id).unwrap().clone(),
        )
    }
}

#[derive(SimpleObject)]
pub struct CreatedMix {
    mix_id: MixID,
    first_tracks: Vec<Track>,
}
