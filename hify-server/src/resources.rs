use std::{fs, path::PathBuf, sync::Arc};

use anyhow::{Context, Result, bail};

use crate::{
    arts::ItemArtsManager,
    index::{AlbumID, ArtistID, Index, TracksList},
    userdata::UserDataWrapper,
};

// TODO: waveform
// TODO: lyrics

#[derive(Clone)]
pub struct ResourceManager {
    pub tracks_file: PathBuf,
    pub user_data_file: PathBuf,
    pub album_arts: Arc<ItemArtsManager<AlbumID>>,
    pub artist_arts: Arc<ItemArtsManager<ArtistID>>,
}

impl ResourceManager {
    pub fn load(path: PathBuf) -> Result<Self> {
        Ok(Self {
            tracks_file: path.join("tracks.json"),
            user_data_file: path.join("userdata.json"),
            album_arts: Arc::new(ItemArtsManager::load(path.join("arts").join("albums"))?),
            artist_arts: Arc::new(ItemArtsManager::load(path.join("arts").join("artists"))?),
        })
    }

    pub fn load_user_data(&self) -> Result<UserDataWrapper> {
        UserDataWrapper::new_create(self.user_data_file.clone())
    }

    pub fn load_index(&self) -> Result<Option<Index>> {
        if self.tracks_file.is_dir() {
            bail!("Tracks index file must not be a directory");
        } else if !self.tracks_file.exists() {
            return Ok(None);
        };

        let tracks_list_json =
            fs::read_to_string(&self.tracks_file).context("Failed to read tracks index file")?;

        let tracks_list = serde_json::from_str::<TracksList>(&tracks_list_json)
            .context("Failed to parse tracks list file")?;

        Ok(Some(Index::build(tracks_list)))
    }

    pub fn save_index(&self, index: &Index) -> Result<()> {
        let tracks_list = index.tracks.values().cloned().collect::<Vec<_>>();

        let json = serde_json::to_string(&tracks_list).unwrap();

        fs::write(&self.tracks_file, json).context("Failed to write tasks index file")
    }
}
