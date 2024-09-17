use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context, Result};
use tokio::{
    fs::{self, File},
    io::{BufReader, BufWriter},
};

use crate::index::IdType;

// TODO: cleanup resources when they don't exist anymore in the index!
// TODO: replace command-line flag '--rebuild-arts' with one to rebuild resources (e.g. '--rebuild-resources')
#[derive(Clone)]
pub struct ResourceManager {
    // TODO: manage generated arts
    // TODO: manage generated waveforms?
    storage_path: Arc<PathBuf>,
}

impl ResourceManager {
    pub fn new(storage_path: PathBuf) -> Self {
        Self {
            storage_path: Arc::new(storage_path),
        }
    }

    fn resource_path<R: ManagedResource>(&self, id: R::Id) -> PathBuf {
        self.storage_path
            .join(R::ID)
            .join(id.encode())
            .with_extension(R::FILE_EXT.unwrap_or_default())
    }

    pub async fn store<R: ManagedResource>(&self, id: R::Id, resource: R) -> Result<PathBuf> {
        let path = self.resource_path::<R>(id);

        let resource_dir = path.parent().unwrap();

        if !resource_dir.is_dir() {
            fs::create_dir(&resource_dir).await.with_context(|| {
                format!(
                    "Failed to create directory for resource '{}' at path '{}'",
                    R::ID,
                    resource_dir.display()
                )
            })?;
        }

        resource.encode_to_file(&path).await?;

        Ok(path)
    }

    pub fn has<R: ManagedResource>(&self, id: R::Id) -> bool {
        self.resource_path::<R>(id).is_file()
    }

    pub fn get_path_of<R: ManagedResource>(&self, id: R::Id) -> Option<PathBuf> {
        let path = self.resource_path::<R>(id);

        if path.is_file() {
            Some(path)
        } else {
            None
        }
    }

    // Currently unused
    #[expect(dead_code)]
    pub async fn retrieve<R: ManagedResource>(&self, id: R::Id) -> Result<Option<R>> {
        let Some(path) = self.get_path_of::<R>(id) else {
            return Ok(None);
        };

        R::decode_from_file(&path).await.map(Some)
    }
}

pub trait ManagedResource {
    const ID: &'static str;
    const FILE_EXT: Option<&'static str>;

    type Id: IdType;

    async fn encode(&self, file: BufWriter<File>) -> Result<()>
    where
        Self: Sized;

    async fn decode(file: BufReader<File>) -> Result<Self>
    where
        Self: Sized;

    async fn encode_to_file(&self, path: &Path) -> Result<()>
    where
        Self: Sized,
    {
        let file = File::create(&path).await.with_context(|| {
            format!(
                "Failed to create file for resource type '{}' at path: {}",
                Self::ID,
                path.display()
            )
        })?;

        self.encode(BufWriter::new(file)).await.with_context(|| {
            format!(
                "Failed to store resource type '{}' at path: {}",
                Self::ID,
                path.display()
            )
        })?;

        Ok(())
    }

    async fn decode_from_file(path: &Path) -> Result<Self>
    where
        Self: Sized,
    {
        let file = File::open(&path).await.with_context(|| {
            format!(
                "Failed to open file for resource type '{}' at path: {}",
                Self::ID,
                path.display()
            )
        })?;

        Self::decode(BufReader::new(file)).await.with_context(|| {
            format!(
                "Failed to decode resource type '{}' at path: {}",
                Self::ID,
                path.display()
            )
        })
    }
}
