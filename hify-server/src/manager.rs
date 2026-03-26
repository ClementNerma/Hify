use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
    time::Instant,
};

use anyhow::{Context, Result, anyhow, bail};
use colored::Colorize;
use log::{debug, info, trace};
use tokio::sync::{RwLock, RwLockReadGuard};

use crate::{
    arts::{ArtSize, ArtsManager, generate_album_arts, generate_artists_art, generate_genres_art},
    index::{
        AlbumID, ArtistID, GenreID, Index, IndexCache, Rating, TrackID, assert_index_correctness,
    },
    indexer,
};

pub type Ratings = HashMap<TrackID, Rating>;

pub struct DataManager {
    music_dir: PathBuf,

    index_path: PathBuf,
    index: RwLock<Index>,
    index_cache: RwLock<IndexCache>,
    index_update_barrier: Mutex<()>,

    ratings_path: PathBuf,
    ratings: RwLock<Ratings>,

    album_arts: ArtsManager<AlbumID>,
    artist_arts: ArtsManager<ArtistID>,
    genre_arts: ArtsManager<GenreID>,
}

impl DataManager {
    pub fn load(data_dir: &Path, music_dir: PathBuf) -> Result<Self> {
        info!("Starting up...");

        if !music_dir.exists() {
            fs::create_dir(data_dir).context("Failed to create the data directory")?;
        }

        let index_path = data_dir.join("index.json");

        let index = if index_path.exists() {
            info!("> Loading library file...");

            let library_str =
                fs::read_to_string(&index_path).context("Failed to read library file")?;

            serde_json::from_str::<Index>(&library_str).context("Failed to parse library file")?
        } else {
            info!("> No library file found");

            Index::default()
        };

        assert_index_correctness(&index);

        let ratings_path = data_dir.join("ratings.json");

        let ratings = if ratings_path.exists() {
            debug!("> Loading ratings file...");

            let ratings_str =
                fs::read_to_string(&ratings_path).context("Failed to read ratings file")?;

            serde_json::from_str::<Ratings>(&ratings_str).context("Failed to parse ratings file")?
        } else {
            debug!("> No ratings file found, starting with empty ratings.");

            HashMap::new()
        };

        debug!("> Building index cache...");
        let index_cache = IndexCache::build(&index);

        info!("> Successfully loaded all data!");

        let generated_dir = data_dir.join("generated");

        if !generated_dir.exists() {
            fs::create_dir(&generated_dir).context("Failed to create data generation directory")?;
        }

        let generated_arts_dir = generated_dir.join("arts");

        if !generated_arts_dir.exists() {
            fs::create_dir(&generated_arts_dir)
                .context("Failed to create arts generation directory")?;
        }

        Ok(Self {
            music_dir,

            index_path,
            index: RwLock::new(index),
            index_cache: RwLock::new(index_cache),
            index_update_barrier: Mutex::new(()),

            ratings_path,
            ratings: RwLock::new(ratings),

            // TODO: check if some arts are missing
            album_arts: ArtsManager::open(generated_arts_dir.join("albums"))?,
            artist_arts: ArtsManager::open(generated_arts_dir.join("artists"))?,
            genre_arts: ArtsManager::open(generated_arts_dir.join("genres"))?,
        })
    }

    pub fn music_dir(&self) -> &Path {
        &self.music_dir
    }

    // TODO: warn if dangling ratings
    pub fn update_index(&self) -> Result<()> {
        let _permit = self
            .index_update_barrier
            .try_lock()
            .map_err(|_| anyhow!("An index update is already pending"))?;

        let start = Instant::now();

        info!("Updating index...");

        let new_index =
            indexer::analyze_tracks_in(&self.music_dir, Some(&self.index_cache.blocking_read()))
                .context("Failed to analyze tracks")?;

        let index_updated = new_index.is_some();

        let index = new_index.unwrap_or_else(|| self.index.blocking_read().clone());
        let index_cache = IndexCache::build(&index);

        if index_updated {
            assert_index_correctness(&index);

            info!("--> Serializing...");

            let index_str = serde_json::to_string_pretty(&index)
                .context("Failed to serialize index")
                .unwrap();

            info!("--> Writing to disk...");

            fs::write(&self.index_path, index_str).context("Failed to write index file")?;
        }

        generate_album_arts(&index_cache, &self.music_dir, &self.album_arts)?;
        generate_artists_art(&index_cache, &self.album_arts, &self.artist_arts)?;
        generate_genres_art(&index_cache, &self.album_arts, &self.genre_arts)?;

        if index_updated {
            info!(
                "-> Index was successfully updated, took {}",
                format!(
                    "{}m {}s",
                    start.elapsed().as_secs() / 60,
                    start.elapsed().as_secs() % 60
                )
                .bright_yellow()
            );

            info!("--> Updating memory...");

            *self.index_cache.blocking_write() = index_cache;
            *self.index.blocking_write() = index;
        }

        Ok(())
    }

    pub fn get_art(&self, entity: Entity, size: ArtSize) -> Result<PathBuf> {
        match entity {
            Entity::Artist(artist_id) => self.artist_arts.get_art_path(artist_id, size),
            Entity::Album(album_id) => self.album_arts.get_art_path(album_id, size),
            Entity::Genre(genre_id) => self.genre_arts.get_art_path(genre_id, size),
        }
    }

    //
    // Async-friendly getters
    //

    pub async fn index(&self) -> RwLockReadGuard<'_, IndexCache> {
        self.index_cache.read().await
    }

    pub async fn ratings(&self) -> RwLockReadGuard<'_, Ratings> {
        self.ratings.read().await
    }

    async fn replace_rating(&self, track_id: TrackID, rating: Option<Rating>) -> Result<()> {
        if !self.index_cache.read().await.tracks.contains_key(&track_id) {
            bail!("Provided track ID was not found");
        }

        // Keep the lock until we finish writing to disk to avoid fs-related data races
        let mut ratings = self.ratings.write().await;

        match rating {
            Some(rating) => {
                ratings.insert(track_id, rating);
            }

            None => {
                ratings.remove(&track_id);
            }
        }

        let ratings_str =
            serde_json::to_string(&*ratings).context("Failed to serialize ratings")?;

        fs::write(&self.ratings_path, &ratings_str).context("Failed to write ratings file")?;

        trace!(
            "> Wrote to ratings file (~ {} Kb)",
            ratings_str.len() / 1024
        );

        Ok(())
    }

    pub async fn set_track_rating(&self, track_id: TrackID, rating: Rating) -> Result<()> {
        self.replace_rating(track_id, Some(rating)).await
    }

    pub async fn remove_track_rating(&self, track_id: TrackID) -> Result<()> {
        self.replace_rating(track_id, None).await
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Entity {
    Artist(ArtistID),
    Album(AlbumID),
    Genre(GenreID),
}
