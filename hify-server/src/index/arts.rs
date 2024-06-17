use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{anyhow, Context, Result};
use image::{
    codecs::png::PngEncoder,
    imageops::{resize, FilterType},
    GenericImage, ImageBuffer,
};
use tokio::{fs, task::spawn_blocking};

use crate::{
    helpers::async_batch::AsyncContextualRunner,
    resources::{ArtistArt, ResourceManager},
};

use super::{AlbumID, AlbumInfos, ArtistID, IndexCache, SortedMap, Track, TrackID};

static COVER_FILENAMES: &[&str] = &["cover", "folder"];
static COVER_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png"];

async fn find_album_art(
    album_id: AlbumID,
    base_dir: &Path,
    tracks: &SortedMap<TrackID, Track>,
    cache: &IndexCache,
) -> Result<Option<PathBuf>> {
    let album_tracks_ids = cache.albums_tracks.get(&album_id).unwrap();

    // Cannot fail as albums need at least one track to be registered
    let first_track_id = album_tracks_ids.first().unwrap();

    let track_path = base_dir.join(&tracks.get(first_track_id).unwrap().relative_path);

    for dir in track_path.ancestors().skip(1) {
        let mut dir_iter = fs::read_dir(dir).await.with_context(|| {
            format!(
                "Failed to read directory during art discovery: {}",
                dir.display()
            )
        })?;

        loop {
            let Some(item) = dir_iter.next_entry().await? else {
                break;
            };

            for filename in COVER_FILENAMES {
                for extension in COVER_EXTENSIONS {
                    if item.file_name().to_string_lossy().to_ascii_lowercase()
                        == format!("{filename}.{extension}")
                    {
                        let relative_path = item.path()
                            .strip_prefix(base_dir)
                            .expect("Internal error: art path couldn't be stripped of the base directory")
                            .to_path_buf();

                        return Ok(Some(relative_path));
                    }
                }
            }
        }
    }

    Ok(None)
}

#[derive(Clone)]
pub struct AlbumsArtFinder {
    base_dir: PathBuf,
    tracks: Arc<SortedMap<TrackID, Track>>,
    cache: Arc<IndexCache>,
}

impl AlbumsArtFinder {
    pub fn new(base_dir: PathBuf, tracks: SortedMap<TrackID, Track>, cache: IndexCache) -> Self {
        Self {
            base_dir,
            tracks: Arc::new(tracks),
            cache: Arc::new(cache),
        }
    }
}

impl AsyncContextualRunner for AlbumsArtFinder {
    type Input = AlbumInfos;
    type Output = (AlbumID, PathBuf);

    async fn run(
        self,
        album: Self::Input,
        warn: impl Fn(&str) + Send,
    ) -> Result<Option<Self::Output>> {
        let album_id = album.get_id();

        let relative_path =
            find_album_art(album_id, &self.base_dir, &self.tracks, &self.cache).await?;

        if relative_path.is_none() {
            warn(&format!(
                "Warning: no album art found for album '{}' by '{}'",
                album.name,
                album
                    .album_artists
                    .iter()
                    .map(|artist| artist.name.clone())
                    .collect::<Vec<_>>()
                    .join(" / ")
            ));
        }

        Ok(relative_path.map(|path| (album_id, path)))
    }
}

// TODO: first decode images for every single album,
//       THEN use them to generate cover arts for artists
#[derive(Clone)]
pub struct ArtistsArtsGenerator {
    base_dir: PathBuf,
    album_arts: Arc<HashMap<AlbumID, PathBuf>>,
    cache: Arc<IndexCache>,
    res_manager: Arc<ResourceManager>,
}

impl ArtistsArtsGenerator {
    pub fn new(
        base_dir: PathBuf,
        album_arts: HashMap<AlbumID, PathBuf>,
        cache: IndexCache,
        res_manager: ResourceManager,
    ) -> Self {
        Self {
            base_dir,
            album_arts: Arc::new(album_arts),
            cache: Arc::new(cache),
            res_manager: Arc::new(res_manager),
        }
    }

    fn create_image_blocking(&self, artist_id: ArtistID) -> Result<Option<Vec<u8>>> {
        // TODO: improve syntax
        let empty_map = SortedMap::<AlbumID, AlbumInfos>::empty();

        let albums_id = self
            .cache
            .artists_albums
            .get(&artist_id)
            .map(|albums| albums.keys())
            .unwrap_or_else(|| empty_map.keys())
            .chain(
                self.cache
                    .artists_album_participations
                    .get(&artist_id)
                    .map(|albums| albums.keys())
                    .unwrap_or_else(|| empty_map.keys()),
            );

        let album_arts = albums_id
            .filter_map(|album_id| self.album_arts.get(album_id))
            .map(|relative_path| {
                let path = self.base_dir.join(relative_path);

                image::open(&path).map(Some).map_err(|err| {
                    anyhow!(
                        "Failed to open art file at path '{}': {err}",
                        path.display()
                    )
                })
            })
            .take(4)
            .filter_map(|result| result.transpose())
            .collect::<Result<Vec<_>, _>>()?;

        if album_arts.is_empty() {
            return Ok(None);
        }

        let image = match album_arts.len() {
            1 => {
                let art = &album_arts[0];

                let mut image = ImageBuffer::new(art.width(), art.height());

                image
                    .copy_from(art, 0, 0)
                    .context("Failed to copy single cover art into artist's one")?;

                image
            }

            _ => {
                let (top_left, top_right, bottom_left, bottom_right) = match album_arts.as_slice() {
                    [ref left, ref right] => (left, right, right, left),

                    [ref top_left, ref top_right, ref bottom_left] => {
                        (top_left, top_right, bottom_left, top_left)
                    }

                    [ref top_left, ref top_right, ref bottom_left, ref bottom_right] => {
                        (top_left, top_right, bottom_left, bottom_right)
                    }

                    _ => unreachable!(),
                };

                // TODO: dynamic dimensions
                // TODO: choose how to handle images with different aspect ratios
                let mut image = ImageBuffer::new(1000, 1000);

                let resize = |image| resize(image, 500, 500, FilterType::Lanczos3);

                image
                    .copy_from(&resize(top_left), 0, 0)
                    .and_then(|()| image.copy_from(&resize(top_right), 500, 0))
                    .and_then(|()| image.copy_from(&resize(bottom_left), 0, 500))
                    .and_then(|()| image.copy_from(&resize(bottom_right), 500, 500))
                    .map_err(|err| {
                        anyhow!("Failed to copy album cover arts into the artist's one: {err}")
                    })?;

                image
            }
        };

        let mut image_buf = vec![];

        image
            .write_with_encoder(PngEncoder::new(&mut image_buf))
            .context("Failed to encode artist's art image")?;

        Ok(Some(image_buf))
    }
}

impl AsyncContextualRunner for ArtistsArtsGenerator {
    type Input = ArtistID;
    type Output = ();

    async fn run(self, artist_id: Self::Input, _: impl Fn(&str)) -> Result<Option<Self::Output>> {
        let res_manager = self.res_manager.clone();

        let image_buf = spawn_blocking(move || self.create_image_blocking(artist_id))
            .await
            .context("Failed to run artist art generation")??;

        if let Some(image_buf) = image_buf {
            res_manager.store(artist_id, ArtistArt(image_buf)).await?;
        }

        Ok(None)
    }
}
