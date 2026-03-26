use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use anyhow::{Context, Result};
use colored::Colorize;
use image::DynamicImage;
use log::{debug, info};

use crate::{
    arts::{LARGE_ART_SIDE_PX, manager::ArtSize, tools::assemble_four_images},
    index::{AlbumID, GenreID, IndexCache},
    utils::{TaskRunner, iter_stable_hash},
};

use super::ArtsManager;

// NOTE: should only be called *AFTER* album arts have been generated
pub fn generate_genres_art(
    index: &IndexCache,
    album_arts: &ArtsManager<AlbumID>,
    genre_arts: &ArtsManager<GenreID>,
) -> Result<()> {
    debug!("-> Checking genres that require new art generation...");

    let mut genre_album_arts = vec![];

    for genre_id in index.genres.keys() {
        let first_albums_with_arts = index
            .genres_albums
            .get(genre_id)
            .unwrap()
            .iter()
            .filter(|album_id| album_arts.has(**album_id))
            .take(4)
            .copied()
            .collect::<Vec<_>>();

        if first_albums_with_arts.is_empty() {
            if genre_arts.has(*genre_id) {
                genre_arts.delete(*genre_id)?;
            }

            continue;
        }

        let img_hash = iter_stable_hash(
            first_albums_with_arts
                .iter()
                .map(|album_id| album_arts.get_art_source_data(*album_id).unwrap()),
        );

        if genre_arts.has_with_source_data(*genre_id, img_hash) {
            continue;
        }

        genre_album_arts.push((*genre_id, first_albums_with_arts, img_hash));
    }

    if genre_album_arts.is_empty() {
        return Ok(());
    }

    info!(
        "-> Generating {} genre arts...",
        genre_album_arts.len().to_string().bright_yellow()
    );

    let mut tasks = TaskRunner::new();

    let total = Arc::new(AtomicUsize::new(0));

    for (genre_id, first_albums, img_hash) in genre_album_arts {
        let album_arts = album_arts.clone();
        let genre_arts = genre_arts.clone();
        let total = Arc::clone(&total);

        tasks.spawn(move || {
            // TODO: prefer images with 1:1 aspect ratio (or closest to it)
            let images = first_albums
                .into_iter()
                .map(|album_id| album_arts.get_art_path(album_id, ArtSize::Large).unwrap())
                .map(|art| {
                    image::open(&art)
                        .with_context(|| {
                            format!("Failed to open album art image at path: {}", art.display())
                        })
                        .map(DynamicImage::into_rgb8)
                })
                .collect::<Result<Vec<_>, _>>()?;

            assert!((1..=4).contains(&images.len()));

            let img = match images.as_slice() {
                [single] => single.clone(),

                [top_left, top_right] => assemble_four_images(
                    top_left,
                    top_right,
                    top_right,
                    top_left,
                    LARGE_ART_SIDE_PX / 2,
                )?,

                [top_left, top_right, bottom_left] => assemble_four_images(
                    top_left,
                    top_right,
                    bottom_left,
                    top_left,
                    LARGE_ART_SIDE_PX / 2,
                )?,

                [top_left, top_right, bottom_left, bottom_right] => assemble_four_images(
                    top_left,
                    top_right,
                    bottom_left,
                    bottom_right,
                    LARGE_ART_SIDE_PX / 2,
                )?,

                [] | [_, _, _, _, ..] => unreachable!(),
            };

            assert!(genre_arts.register(genre_id, img_hash, &img)?);

            let curr = total.fetch_add(1, Ordering::SeqCst) + 1;

            if curr.is_multiple_of(100) {
                debug!(
                    "--> Generated {} genre arts so far...",
                    curr.to_string().bright_yellow()
                );
            }

            Ok(())
        });
    }

    tasks.join_all()?;

    info!(
        "-> Successfully generated {} genre arts",
        total.load(Ordering::SeqCst).to_string().bright_yellow()
    );

    Ok(())
}
