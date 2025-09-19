use std::path::Path;

use anyhow::{Result, anyhow};
use log::info;

use crate::{
    arts::generate_arts,
    index::{Index, TracksList, analyze_tracks_in},
    resources::ResourceManager,
    userdata::UserDataWrapper,
};

pub fn detect_changes(
    music_dir: &Path,
    user_data: &mut UserDataWrapper,
    res_manager: &ResourceManager,
    prev_index: Option<&Index>,
) -> Result<Index> {
    let prev_tracks_list =
        prev_index.map(|index| TracksList(index.tracks.values().cloned().collect::<Vec<_>>()));

    info!("> Searching for tracks...");
    let tracks_list = analyze_tracks_in(music_dir, prev_tracks_list.as_ref())?;

    info!("> Building index...");
    let index = Index::build(tracks_list);

    info!("> Generating arts...");
    generate_arts(music_dir, &index, prev_index, res_manager)?;

    info!("> Saving index...");
    res_manager.save_index(&index)?;

    user_data
        .cleanup_sync(&index)
        .map_err(|err| anyhow!("Failed to cleanup user data: {err}"))?;

    Ok(index)
}
