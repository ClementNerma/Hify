use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use log::error;

use crate::{index::TrackMetadata, utils::TaskRunner};

use super::{analyzer::analyze_file, tags::TrackStrTags};

pub fn analyze_audio_files(
    files: impl Iterator<Item = PathBuf>,
    dir: &Path,
) -> Result<Vec<(PathBuf, (TrackMetadata, TrackStrTags))>> {
    let mut tasks = TaskRunner::new();

    for file in files {
        let dir = dir.to_owned();

        tasks.spawn(move || {
            analyze_file(&dir.join(&file))
                .with_context(|| {
                    format!("Failed to analyze audio file at path: {}", file.display())
                })
                .map(|mt| (file, mt))
        });
    }

    tasks.join_all()
}

pub fn may_be_audio_file(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();

    let audio_ext = match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => ext.to_ascii_lowercase(),
        None => return false,
    };

    if matches!(
        audio_ext.as_str(),
        "mpeg" | "mp4" | "webm" | "alac" | "aiff" | "dsf"
    ) {
        error!(
            "Warning: in file '{}': file format unsupported by web players: {audio_ext}",
            path.to_string_lossy()
        );

        return false;
    }

    if matches!(audio_ext.as_str(), "aac" | "wav") {
        error!(
            "Warning: in file '{}': file format explicitly unsupported: {audio_ext}",
            path.to_string_lossy()
        );

        return false;
    }

    matches!(audio_ext.as_str(), "mp3" | "flac" | "m4a" | "ogg" | "opus")
}
