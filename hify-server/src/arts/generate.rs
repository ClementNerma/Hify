use std::{collections::HashSet, path::Path};

use anyhow::Result;
use log::info;

use crate::{
    arts::{albums::find_album_covers, artists::generate_artist_arts},
    index::Index,
    resources::ResourceManager,
    runner::{TaskSet, TaskSetOptions},
};

pub fn generate_arts(
    music_dir: &Path,
    index: &Index,
    prev_index: Option<&Index>,
    res_manager: &ResourceManager,
) -> Result<()> {
    let albums_new_index = index.albums_infos.keys().collect::<HashSet<_>>();
    let albums_prev_index = prev_index.map_or_else(Default::default, |prev_index| {
        prev_index.albums_infos.keys().collect::<HashSet<_>>()
    });

    info!("> Looking for album covers...");

    let album_covers = find_album_covers(music_dir, index.clone())?;

    info!("|-> Generating miniatures for album covers...");

    let album_covers = album_covers
        .into_iter()
        .filter(|(album_id, hash, _)| {
            res_manager
                .album_arts
                .get_hash(*album_id)
                .is_none_or(|existing_hash| *hash != existing_hash)
        })
        .collect::<Vec<_>>();

    let mut runner = TaskSet::new();

    for (item_id, source_hash, content) in album_covers {
        let album_arts_manager = res_manager.album_arts.clone();
        runner.add(move || album_arts_manager.register_art(item_id, source_hash, content));
    }

    for result in runner.run(TaskSetOptions::with_progress_bar()) {
        result??;
    }

    for deleted_album in &albums_prev_index - &albums_new_index {
        res_manager.album_arts.delete_arts(*deleted_album)?;
    }

    let artists_new_index = index.artists_infos.keys().collect::<HashSet<_>>();
    let artists_prev_index = prev_index.map_or_else(Default::default, |prev_index| {
        prev_index.artists_infos.keys().collect::<HashSet<_>>()
    });

    info!("> Generating miniatures for artists...");

    for deleted_artist in &artists_prev_index - &artists_new_index {
        res_manager.artist_arts.delete_arts(*deleted_artist)?;
    }

    generate_artist_arts(index, res_manager)?;

    info!("> Done generating arts!");

    Ok(())
}
