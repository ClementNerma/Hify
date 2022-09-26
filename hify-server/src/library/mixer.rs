use async_graphql::InputObject;
use rand::{seq::SliceRandom, thread_rng};

use crate::index::{Index, Track};

#[derive(InputObject)]
pub struct MixerParams {
    min_rating: Option<u8>,
    max_tracks: Option<u8>,
}

pub fn generate_mix(index: &Index, params: MixerParams) -> Vec<Track> {
    let param_min_rating = params.min_rating.unwrap_or(80);
    let param_max_tracks = usize::from(params.max_tracks.unwrap_or(200));

    let mut tracks: Vec<_> = index
        .tracks
        .values()
        .filter(|track| match track.metadata.tags.rating {
            None => param_min_rating == 0,
            Some(rating) => rating >= param_min_rating,
        })
        .collect();

    tracks.shuffle(&mut thread_rng());

    tracks.into_iter().take(param_max_tracks).cloned().collect()
}
