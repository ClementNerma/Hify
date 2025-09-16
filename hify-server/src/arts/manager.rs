use std::{
    collections::HashMap,
    fs,
    hash::{DefaultHasher, Hash, Hasher},
    path::{Path, PathBuf},
    sync::{LazyLock, RwLock},
    time::{Duration, SystemTime},
};

use anyhow::{Context, Result, anyhow, bail};
use image::RgbImage;
use log::debug;
use regex::Regex;

use crate::{
    arts::tools::{resize_image_constraint, save_image_webp},
    index::IdType,
};

pub struct ItemArtsManager<I: IdType> {
    dir: PathBuf,
    arts: RwLock<HashMap<I, ItemArts>>,
}

impl<I: IdType> ItemArtsManager<I> {
    pub fn load(dir: PathBuf) -> Result<Self> {
        if !dir.is_dir() {
            fs::create_dir_all(&dir).context("Failed to create arts directory")?;

            return Ok(Self {
                dir,
                arts: RwLock::new(HashMap::new()),
            });
        }

        let mut arts = HashMap::<I, ItemArts>::new();

        for entry in fs::read_dir(&dir)? {
            let entry = entry?;

            if !entry.path().is_dir() {
                bail!(
                    "Found non-directory item in arts directory: {}",
                    entry.path().display()
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

            let id = parsed.get(1).unwrap().as_str();
            let id = I::decode(id)
                .map_err(|_| anyhow!("Invalid ID in directory name in arts directory: {id}"))?;

            let source_hash = parsed.get(2).unwrap().as_str();
            let source_hash = source_hash.parse::<u64>().with_context(|| {
                format!("Invalid hash in directory name in arts directory: {source_hash:?})")
            })?;

            let date = parsed.get(3).unwrap().as_str();
            let date = date.parse::<u64>().with_context(|| {
                format!("Invalid timestamp in directory name in arts directory: {date:?}")
            })?;
            let date = SystemTime::UNIX_EPOCH
                .checked_add(Duration::from_secs(date))
                .with_context(|| {
                    format!("Invalid timestamp in directory name in arts directory: {date:?}")
                })?;

            if let Some(other_arts) = arts.get(&id).cloned() {
                if date < other_arts.date {
                    debug!(
                        "> Cleaning up dangling arts directory: {}",
                        entry.path().display()
                    );

                    fs::remove_dir_all(entry.path())?;
                    continue;
                } else {
                    debug!(
                        "> Cleaning up dangling arts directory: {}",
                        other_arts.dir.display()
                    );

                    fs::remove_dir_all(entry.path())?;
                }
            }

            arts.insert(
                id,
                ItemArts {
                    dir: entry.path(),
                    source_hash,
                    date,
                },
            );
        }

        Ok(Self {
            dir,
            arts: RwLock::new(arts),
        })
    }

    pub fn has(&self, id: I) -> bool {
        self.arts.read().unwrap().contains_key(&id)
    }

    pub fn get_hash(&self, id: I) -> Option<u64> {
        self.arts
            .read()
            .unwrap()
            .get(&id)
            .map(|arts| arts.source_hash)
    }

    pub fn large_art(&self, id: I) -> Option<PathBuf> {
        // TODO: put filenames in constants!
        self.arts
            .read()
            .unwrap()
            .get(&id)
            .map(|arts| arts.dir.join("large.webp"))
    }

    pub fn medium_art(&self, id: I) -> Option<PathBuf> {
        self.arts
            .read()
            .unwrap()
            .get(&id)
            .map(|arts| arts.dir.join("medium.webp"))
    }

    pub fn small_art(&self, id: I) -> Option<PathBuf> {
        self.arts
            .read()
            .unwrap()
            .get(&id)
            .map(|arts| arts.dir.join("small.webp"))
    }

    pub fn register_art(
        &self,
        item_id: I,
        source_hash: u64,
        content: RegisterableArtType,
    ) -> Result<()> {
        if let Some(existing_hash) = self.get_hash(item_id) {
            if source_hash == existing_hash {
                return Ok(());
            }

            self.delete_arts(item_id)?;
        }

        if self
            .arts
            .read()
            .unwrap()
            .get(&item_id)
            .is_some_and(|item_arts| item_arts.source_hash == source_hash)
        {
            return Ok(());
        }

        let dir = self.dir.clone();

        let image = match content {
            RegisterableArtType::File(img_path) => {
                let image = image::open(&img_path).with_context(|| {
                    format!("Failed to open source image: {}", img_path.display())
                })?;

                image.into_rgb8()
            }

            RegisterableArtType::Buffer(image) => image,
        };

        let date = SystemTime::now();

        let item_arts_dir = dir.join(format!(
            "{}[{source_hash}]@{}",
            item_id.encode(),
            date.duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ));

        std::fs::create_dir_all(&item_arts_dir).with_context(|| {
            format!(
                "Failed to create item arts directory at: {}",
                item_arts_dir.display()
            )
        })?;

        // TODO: put the three sizes into constants
        save_image_webp(
            &resize_image_constraint(&image, 2000),
            &item_arts_dir.join("large.webp"),
        )?;

        save_image_webp(
            &resize_image_constraint(&image, 500),
            &item_arts_dir.join("medium.webp"),
        )?;

        save_image_webp(
            &resize_image_constraint(&image, 200),
            &item_arts_dir.join("small.webp"),
        )?;

        self.arts.write().unwrap().insert(
            item_id,
            ItemArts {
                dir: item_arts_dir,
                source_hash,
                date: SystemTime::now(),
            },
        );

        Ok(())
    }

    pub fn delete_arts(&self, id: I) -> Result<()> {
        {
            let arts = self.arts.read().unwrap();
            let arts = arts.get(&id).with_context(|| {
                format!("Unknown ID provided for arts deletion: {}", id.encode())
            })?;

            fs::remove_dir_all(&arts.dir).with_context(|| {
                format!(
                    "Failed to delete item arts directory at: {}",
                    arts.dir.display()
                )
            })?;
        }

        self.arts.write().unwrap().remove(&id);

        Ok(())
    }
}

#[derive(Clone)]
pub struct ItemArts {
    pub dir: PathBuf,
    pub source_hash: u64,
    pub date: SystemTime,
}

pub enum RegisterableArtType {
    File(PathBuf),
    Buffer(RgbImage),
}

static FILENAME_PARSER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^([a-zA-Z0-9]+)\\[([a-zA-Z0-9]+)\\]@([a-zA-Z0-9]+)$").unwrap());

pub fn hash_for_file(path: &Path) -> Result<u64> {
    let path = Path::to_owned(path);

    let mt = fs::metadata(&path)
        .with_context(|| format!("Failed to get metadata for image file: {}", path.display()))?;

    let mtime = mt.modified().with_context(|| {
        format!(
            "Failed to get modification time for image file: {}",
            path.display()
        )
    })?;

    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    mtime.hash(&mut hasher);
    mt.len().hash(&mut hasher);

    Ok(hasher.finish())
}
