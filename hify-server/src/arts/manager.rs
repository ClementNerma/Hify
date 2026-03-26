use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    marker::PhantomData,
    path::PathBuf,
    sync::{Arc, LazyLock, RwLock},
};

use anyhow::{Context, Result, anyhow, bail};
use image::RgbImage;
use log::error;
use regex::Regex;
use serde::Deserialize;

use crate::{
    arts::{LARGE_ART_SIDE_PX, MEDIUM_ART_SIDE_PX, SMALL_ART_SIDE_PX, TINY_ART_SIDE_PX, tools},
    index::IdType,
    utils::{decode_base62_u64, encode_base62_u64},
};

static LARGE_WEBP_FILENAME: &str = "large.webp";
static MEDIUM_WEBP_FILENAME: &str = "medium.webp";
static SMALL_WEBP_FILENAME: &str = "small.webp";
static TINY_WEBP_FILENAME: &str = "tiny.webp";

static INCOMPLETE_DIR_NAME: &str = ".incomplete";

#[derive(Clone)]
pub struct ArtsManager<I: IdType> {
    arts: Arc<RwLock<HashMap<I, ArtDirForItem>>>,
    dir: PathBuf,
    incomplete_dir: PathBuf,
    _i: PhantomData<I>,
}

impl<I: IdType> ArtsManager<I> {
    pub fn open(dir: PathBuf) -> Result<Self> {
        if !dir.exists() {
            fs::create_dir_all(&dir).context("Failed to create art directory")?;

            return Ok(Self {
                arts: Arc::new(RwLock::new(HashMap::new())),
                incomplete_dir: dir.join(INCOMPLETE_DIR_NAME),
                dir,
                _i: PhantomData,
            });
        }

        let incomplete_dir = dir.join(INCOMPLETE_DIR_NAME);

        if incomplete_dir.exists() {
            fs::remove_dir_all(&incomplete_dir)
                .context("Failed to remove incomplete art directory")?;
        }

        let mut arts = HashMap::new();

        for entry in fs::read_dir(&dir).context("Failed to read art directory")? {
            let entry = entry.context("Failed to read art directory entry")?;

            let mt = fs::metadata(entry.path()).with_context(|| {
                format!(
                    "Failed to read metadata for arts directory entry: {}",
                    entry.path().display()
                )
            })?;

            if !mt.is_dir() {
                bail!(
                    "Invalid entry in arts directory: {}",
                    entry.file_name().display()
                );
            }

            let filename = entry.file_name();
            let filename = filename.to_str().with_context(|| {
                format!(
                    "Invalid UTF-8 filename in arts directory: {}",
                    entry.file_name().display()
                )
            })?;

            let parsed = FILENAME_PARSER.captures(filename).with_context(|| {
                format!("Invalid directory name in arts directory: {filename:?}")
            })?;

            let item_id = parsed.get(1).unwrap().as_str();
            let item_id = I::decode(item_id).with_context(|| {
                format!("Invalid ID in directory name in arts directory: {item_id}")
            })?;

            let for_data = parsed.get(2).unwrap().as_str();
            let for_data = decode_base62_u64(for_data)
                .map_err(|err| anyhow!(err))
                .with_context(|| {
                    format!("Invalid hash in directory name in arts directory: {for_data:?})")
                })?;

            let mut dir_entries = fs::read_dir(entry.path())
                .and_then(Iterator::collect::<Result<Vec<_>, _>>)
                .with_context(|| {
                    format!(
                        "Failed to read art subdirectory entries: {}",
                        entry.path().display()
                    )
                })?;

            dir_entries.sort_by_key(DirEntry::path);

            #[allow(clippy::indexing_slicing)]
            if dir_entries.len() != 4
                || dir_entries[0].file_name() != LARGE_WEBP_FILENAME
                || dir_entries[1].file_name() != MEDIUM_WEBP_FILENAME
                || dir_entries[2].file_name() != SMALL_WEBP_FILENAME
                || dir_entries[3].file_name() != TINY_WEBP_FILENAME
            {
                error!(
                    "Art directory for item {} has invalid content, deleting...",
                    item_id.encode()
                );

                fs::remove_dir_all(entry.path())?;

                continue;
            }

            arts.insert(
                item_id,
                ArtDirForItem {
                    for_data,
                    path: entry.path(),
                },
            );
        }

        Ok(Self {
            arts: Arc::new(RwLock::new(arts)),
            incomplete_dir,
            dir,
            _i: PhantomData,
        })
    }

    pub fn has(&self, item_id: I) -> bool {
        let arts = self.arts.read().unwrap();
        arts.contains_key(&item_id)
    }

    pub fn has_with_source_data(&self, item_id: I, source_data: u64) -> bool {
        let arts = self.arts.read().unwrap();

        arts.get(&item_id)
            .is_some_and(|art_dir| art_dir.for_data == source_data)
    }

    pub fn get_art_source_data(&self, item_id: I) -> Option<u64> {
        let arts = self.arts.read().unwrap();

        let art_dir = arts.get(&item_id)?;

        Some(art_dir.for_data)
    }

    pub fn get_art_path(&self, item_id: I, size: ArtSize) -> Result<PathBuf> {
        let arts = self.arts.read().unwrap();

        let art_dir = arts
            .get(&item_id)
            .with_context(|| format!("No art registered for item {}", item_id.encode()))?;

        Ok(art_dir.path.join(match size {
            ArtSize::Large => LARGE_WEBP_FILENAME,
            ArtSize::Medium => MEDIUM_WEBP_FILENAME,
            ArtSize::Small => SMALL_WEBP_FILENAME,
            ArtSize::Tiny => TINY_WEBP_FILENAME,
        }))
    }

    pub fn register(&self, item_id: I, for_data: u64, img: &RgbImage) -> Result<bool> {
        let art_dir = self.dir.join(format!(
            "{}--@--{}",
            item_id.encode(),
            encode_base62_u64(for_data)
        ));

        {
            let existing = self.arts.read().unwrap();

            if let Some(existing) = existing.get(&item_id) {
                if existing.for_data == for_data {
                    return Ok(false);
                }

                if art_dir.exists() {
                    fs::remove_dir_all(&art_dir).with_context(|| {
                        format!(
                            "Failed to remove previous art directory for item {item_id:?}: {}",
                            art_dir.display()
                        )
                    })?;
                }
            }
        }

        if art_dir.exists() {
            return Ok(false);
        }

        let incomplete_dir = self.incomplete_dir.join(format!(
            "{}--@--{}",
            item_id.encode(),
            encode_base62_u64(for_data)
        ));

        fs::create_dir_all(&incomplete_dir).with_context(|| {
            format!(
                "Failed to create art directory for item {item_id:?}: {}",
                incomplete_dir.display()
            )
        })?;

        let tiny = tools::resize_image(img, TINY_ART_SIDE_PX, TINY_ART_SIDE_PX);
        tools::save_image_webp(&incomplete_dir.join(TINY_WEBP_FILENAME), &tiny)?;

        let small = tools::resize_image(img, SMALL_ART_SIDE_PX, SMALL_ART_SIDE_PX);
        tools::save_image_webp(&incomplete_dir.join(SMALL_WEBP_FILENAME), &small)?;

        let medium = tools::resize_image(img, MEDIUM_ART_SIDE_PX, MEDIUM_ART_SIDE_PX);
        tools::save_image_webp(&incomplete_dir.join(MEDIUM_WEBP_FILENAME), &medium)?;

        let large = tools::resize_image(img, LARGE_ART_SIDE_PX, LARGE_ART_SIDE_PX);
        tools::save_image_webp(&incomplete_dir.join(LARGE_WEBP_FILENAME), &large)?;

        fs::rename(&incomplete_dir, &art_dir).with_context(|| {
            format!(
                "Failed to rename incomplete art directory for item {item_id:?}: {} -> {}",
                incomplete_dir.display(),
                art_dir.display()
            )
        })?;

        let mut arts = self.arts.write().unwrap();

        arts.insert(
            item_id,
            ArtDirForItem {
                for_data,
                path: art_dir,
            },
        );

        Ok(true)
    }

    pub fn delete(&self, item_id: I) -> Result<()> {
        let mut arts = self.arts.write().unwrap();

        let art_dir = arts.remove(&item_id).with_context(|| {
            format!("No art found for item {item_id:?} when trying to delete it")
        })?;

        fs::remove_dir_all(&art_dir.path).with_context(|| {
            format!(
                "Failed to remove art directory for item {item_id:?}: {}",
                art_dir.path.display()
            )
        })
    }
}

static FILENAME_PARSER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^([a-zA-Z0-9]+)--@--([a-zA-Z0-9]+)$").unwrap());

struct ArtDirForItem {
    for_data: u64,
    path: PathBuf,
}

// TODO: check if ALL these sizes are actually used
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ArtSize {
    Large,
    Medium,
    Small,
    Tiny,
}
