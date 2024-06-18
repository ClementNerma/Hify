use std::ops::Deref;

use anyhow::{Context, Result};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
};

use crate::index::ArtistID;

use super::manager::ManagedResource;

pub struct ArtistArt(pub Vec<u8>);

impl Deref for ArtistArt {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ManagedResource for ArtistArt {
    const ID: &'static str = "artist_art";
    const FILE_EXT: Option<&'static str> = Some("png");

    type Id = ArtistID;

    async fn encode(&self, mut file: BufWriter<File>) -> Result<()>
    where
        Self: Sized,
    {
        file.write_all(self).await?;
        Ok(())
    }

    async fn decode(mut file: BufReader<File>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut buf = vec![];

        file.read_to_end(&mut buf)
            .await
            .context("Failed to read image file")?;

        Ok(Self(buf))
    }
}
