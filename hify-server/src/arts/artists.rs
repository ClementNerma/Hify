use std::{
    hash::{DefaultHasher, Hash, Hasher},
    path::Path,
    sync::Arc,
};

use anyhow::{Context, Result};
use image::RgbImage;

use crate::{
    arts::{
        RegisterableArtType,
        tools::{assemble_four_images, resize_image, resize_image_constraint},
    },
    index::{ArtistID, Index},
    resources::ResourceManager,
    runner::{TaskSet, TaskSetOptions},
};

use super::ItemArtsManager;

pub fn generate_artist_arts(index: &Index, res_manager: &ResourceManager) -> Result<()> {
    let artists = index.artists_albums_and_participations.keys().copied();

    let mut runner = TaskSet::<Result<()>>::new();
    let index = Arc::new(index.clone());

    for artist_id in artists {
        let index = Arc::clone(&index);

        let albums_art_manager = Arc::clone(&res_manager.album_arts);
        let artists_art_manager = Arc::clone(&res_manager.artist_arts);

        runner.add(move || {
            let artist_album_arts = index
                .artists_albums_and_participations
                .get(&artist_id)
                .unwrap()
                .keys()
                .filter_map(|album_id| albums_art_manager.large_art(*album_id))
                .take(4)
                .collect::<Vec<_>>();

            // TODO: put '2000' in a constant
            match artist_album_arts.as_slice() {
                [] => Ok(()),

                [single] => assemble(
                    artist_id,
                    &artists_art_manager,
                    [single.as_path()],
                    |[single]| Ok(resize_image_constraint(&single, 2000).into_owned()),
                ),

                [left, right] => assemble(
                    artist_id,
                    &artists_art_manager,
                    [left.as_path(), right.as_path()],
                    |[left, right]| assemble_four_images(&left, &right, &right, &left, 2000),
                ),

                [top_left, top_right, bottom_left] => assemble(
                    artist_id,
                    &artists_art_manager,
                    [
                        top_left.as_path(),
                        top_right.as_path(),
                        bottom_left.as_path(),
                    ],
                    |[top_left, top_right, bottom_left]| {
                        assemble_four_images(&top_left, &top_right, &bottom_left, &top_left, 2000)
                    },
                ),

                [top_left, top_right, bottom_left, bottom_right, ..] => assemble(
                    artist_id,
                    &artists_art_manager,
                    [
                        top_left.as_path(),
                        top_right.as_path(),
                        bottom_left.as_path(),
                        bottom_right.as_path(),
                    ],
                    |[top_left, top_right, bottom_left, bottom_right]| {
                        assemble_four_images(
                            &top_left,
                            &top_right,
                            &bottom_left,
                            &bottom_right,
                            2000,
                        )
                    },
                ),
            }
        });
    }

    for result in runner.run(TaskSetOptions::with_progress_bar()) {
        result??;
    }

    Ok(())
}

fn assemble<const N: usize>(
    artist_id: ArtistID,
    artists_art_manager: &ItemArtsManager<ArtistID>,
    files: [&Path; N],
    assemble: impl FnOnce([RgbImage; N]) -> Result<RgbImage>,
) -> Result<()> {
    let load = |path: &Path| {
        image::open(path)
            .with_context(|| format!("Failed to open album art file at: {}", path.display()))
            .map(|img| resize_image(&img.into_rgb8(), 1000, 1000))
    };

    let mut hasher = DefaultHasher::new();

    for file in files {
        file.hash(&mut hasher);
    }

    let source_hash = hasher.finish();

    if artists_art_manager
        .get_hash(artist_id)
        .is_some_and(|existing_hash| source_hash == existing_hash)
    {
        return Ok(());
    }

    let files = files.into_iter().map(load).collect::<Result<Vec<_>>>()?;

    let image = assemble(files.try_into().unwrap())?;

    artists_art_manager.register_art(artist_id, source_hash, RegisterableArtType::Buffer(image))?;

    Ok(())
}
