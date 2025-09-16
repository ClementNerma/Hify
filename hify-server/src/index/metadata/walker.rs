use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use log::{error, info};

use crate::{
    index::{TrackMetadata, metadata::analyzer::analyze_file},
    runner::{TaskSet, TaskSetOptions},
};

pub fn analyze_audio_files(files: Vec<PathBuf>) -> Result<Vec<(PathBuf, TrackMetadata)>> {
    if files.is_empty() {
        info!("Nothing to do!");
        return Ok(vec![]);
    }

    let mut runner = TaskSet::new();

    for file in &files {
        let file = file.to_owned();

        runner.add(move || {
            analyze_file(&file)
                .with_context(|| {
                    format!("Failed to analyze audio file at path: {}", file.display())
                })
                .map(|mt| (file, mt))
        });
    }

    let mut successes = vec![];
    let mut errors = 0;

    for result in runner.run(TaskSetOptions::with_progress_bar()) {
        match result.context("Failed to join task")? {
            Ok(data) => successes.push(data),
            Err(err) => {
                error!("* {err:?}");
                errors += 1;
            }
        }
    }

    assert_eq!(successes.len() + errors, files.len());

    if errors > 0 {
        bail!("Failed with {errors} errors")
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
