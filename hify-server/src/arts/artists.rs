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
    index::{AlbumID, ArtistID, IndexCache},
    utils::{TaskRunner, iter_stable_hash},
};

use super::ArtsManager;

// NOTE: should only be called *AFTER* album arts have been generated
#[allow(clippy::too_many_lines)]
pub fn generate_artists_art(
    index: &IndexCache,
    album_arts: &ArtsManager<AlbumID>,
    artist_arts: &ArtsManager<ArtistID>,
) -> Result<()> {
    debug!("-> Checking artists that require new art generation...");

    let mut artist_album_arts = vec![];

    for artist_id in index.artists.keys() {
        let artist_in_albums = index
            .artists_albums
            .get(artist_id)
            .unwrap()
            .iter()
            .chain(
                index
                    .artists_album_participations
                    .get(artist_id)
                    .unwrap()
                    .iter(),
            )
            .take(4)
            .copied()
            .collect::<Vec<_>>();

        assert!(!artist_in_albums.is_empty());

        if artist_in_albums.is_empty() {
            if artist_arts.has(*artist_id) {
                artist_arts.delete(*artist_id)?;
            }

            continue;
        }

        let img_hash = iter_stable_hash(
            artist_in_albums
                .iter()
                .map(|album_id| album_arts.get_art_source_data(*album_id).unwrap()),
        );

        if artist_arts.has_with_source_data(*artist_id, img_hash) {
            continue;
        }

        artist_album_arts.push((*artist_id, artist_in_albums, img_hash));
    }

    if artist_album_arts.is_empty() {
        return Ok(());
    }

    info!(
        "-> Generating {} artist arts...",
        artist_album_arts.len().to_string().bright_yellow()
    );

    let mut tasks = TaskRunner::new();

    let total = Arc::new(AtomicUsize::new(0));

    for (artist_id, first_albums, img_hash) in artist_album_arts {
        let album_arts = album_arts.clone();
        let artist_arts = artist_arts.clone();
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

            assert!(artist_arts.register(artist_id, img_hash, &img)?);

            let curr = total.fetch_add(1, Ordering::SeqCst) + 1;

            if curr.is_multiple_of(100) {
                debug!(
                    "--> Generated {} artist arts so far...",
                    curr.to_string().bright_yellow()
                );
            }

            Ok(())
        });
    }

    tasks.join_all()?;

    info!(
        "-> Successfully generated {} artist arts",
        total.load(Ordering::SeqCst).to_string().bright_yellow()
    );

    Ok(())
}
