use async_graphql::InputObject;
use rand::{seq::SliceRandom, thread_rng};

use crate::index::{Index, Track};

#[derive(InputObject)]
pub struct MixerParams {
    min_note: Option<u8>,
    max_tracks: Option<u8>,
}

pub fn generate_mix(index: &Index, params: MixerParams) -> Vec<Track> {
    let param_min_note = params.min_note.unwrap_or(80);
    let param_max_tracks = usize::from(params.max_tracks.unwrap_or(200));

    let mut tracks: Vec<_> = index
        .tracks
        .values()
        .filter(|track| match track.metadata.tags.note {
            None => param_min_note == 0,
            Some(note) => note >= param_min_note,
        })
        .collect();

    tracks.shuffle(&mut thread_rng());

    tracks.into_iter().take(param_max_tracks).cloned().collect()
}
