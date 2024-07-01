use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use log::{error, info};

use crate::{
    helpers::logging::progress_bar,
    index::{metadata::analyzer::analyze_file, TrackMetadata},
};

pub async fn analyze_audio_files(files: Vec<PathBuf>) -> Result<Vec<(PathBuf, TrackMetadata)>> {
    if files.is_empty() {
        info!("Nothing to do!");
        return Ok(vec![]);
    }

    let pb = progress_bar(files.len());

    let mut tasks = vec![];

    for file in &files {
        let file_bis = file.clone();
        let pb = pb.clone();

        tasks.push((
            file.clone(),
            tokio::task::spawn_blocking(move || {
                let ret = analyze_file(file_bis);
                pb.inc(1);
                ret
            }),
        ));
    }

    let mut successes = vec![];
    let mut errors = vec![];

    for (file, task) in tasks {
        match task.await.context("Failed to join task")? {
            Ok(data) => successes.push((file, data)),
            Err(err) => errors.push((file, err)),
        }
    }

    pb.finish();

    assert_eq!(successes.len() + errors.len(), files.len());

    if !errors.is_empty() {
        error!(
            "Failed with the following errors:\n{}",
            errors
                .iter()
                .map(|(file, err)| format!("* {}: {err:?}", file.to_string_lossy()))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    Ok(successes)
}

pub fn is_audio_file(path: impl AsRef<Path>) -> bool {
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

    matches!(
        audio_ext.as_str(),
        "mp3" | "flac" | "wav" | "aac" | "m4a" | "ogg" | "opus"
    )
}
