use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use image::{codecs::webp::WebPEncoder, ImageBuffer, Rgba};

use crate::index::{ArtistID, IdType};

// TODO: waveform
// TODO: lyrics

#[derive(Clone)]
pub struct ResourceManager {
    // path: PathBuf,
    artist_arts_dir: PathBuf,
}

impl ResourceManager {
    pub fn new(path: PathBuf) -> Result<Self> {
        let artist_arts_dir = path.join("artist-arts");
        fs::create_dir_all(&artist_arts_dir).context("Failed to create artist arts directory")?;

        Ok(Self { artist_arts_dir })
    }

    fn _artist_art_path(&self, artist_id: ArtistID) -> PathBuf {
        self.artist_arts_dir
            .join(format!("{}.webp", artist_id.encode()))
    }

    // TODO: replace rgba with rgb?
    pub fn save_artist_art(
        &self,
        artist_id: ArtistID,
        image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<()> {
        let mut image_buf = vec![];

        image
            .write_with_encoder(WebPEncoder::new_lossless(&mut image_buf))
            .context("Failed to encode artist's art image")?;

        let path = self._artist_art_path(artist_id);

        fs::write(&path, image_buf)
            .with_context(|| format!("Failed to write image file at path: {}", path.display()))?;

        Ok(())
    }

    pub fn artist_art_path(&self, artist_id: ArtistID) -> Option<PathBuf> {
        let path = self._artist_art_path(artist_id);

        if path.is_file() {
            Some(path)
        } else {
            None
        }
    }
}
