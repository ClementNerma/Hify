use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::Mutex,
};

use anyhow::{anyhow, bail, Context, Result};
use log::{error, info};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde::Deserialize;

use crate::{index::TrackMetadata, utils::logging::files_progress_bar};

use super::file::{process_analyzed_file, ExifToolFile};

pub fn is_audio_file(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();

    let audio_ext = match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => ext.to_ascii_lowercase(),
        None => return false,
    };

    if matches!(
        audio_ext.as_str(),
        "mpeg" | "mp4" | "alac" | "webm" | "aiff" | "dsf"
    ) {
        error!(
            "Warning: in file '{}': file format unsupported by web players: {audio_ext}",
            path.to_string_lossy()
        );

        return false;
    }

    matches!(
        audio_ext.as_str(),
        "mp3" | "flac" | "wav" | "aac" | "ogg" | "m4a" | "opus"
    )
}

pub fn run_on(files: &[PathBuf]) -> Result<Vec<(PathBuf, TrackMetadata)>> {
    if files.is_empty() {
        info!("Nothing to do!");
        return Ok(vec![]);
    }

    info!("Starting analysis...");

    let successes = Mutex::new(vec![]);
    let errors = Mutex::new(vec![]);

    let pb = files_progress_bar(files.len());

    files
        .par_iter()
        .map(|file| {
            let output = Command::new("exiftool")
                .args(["-n", "-json"])
                .arg(file)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .context("Failed to run ExifTool")?;

            let status = output.status;

            pb.inc(1);

            if !status.success() {
                bail!(
                    "ExifTool failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }

            let stdout = String::from_utf8_lossy(&output.stdout);

            let parsed_output = serde_json::from_str::<ExifToolOutput>(&stdout).map_err(|e| {
                anyhow!(
                    "Failed to parse ExifTool output: {}\n\n{}",
                    e,
                    stdout
                        .lines()
                        .enumerate()
                        .skip(if e.line() < 15 { 0 } else { e.line() - 15 })
                        .take(15)
                        .map(|(i, line)| format!("Line {: >5} *| {line}", i + 1))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            })?;

            let mut analyzed = parsed_output.0;

            if analyzed.len() != 1 {
                bail!(
                    "ExifTool was expected to return one element, but it returned {} instead.",
                    analyzed.len()
                );
            }

            let analyzed = analyzed.pop().unwrap();

            match process_analyzed_file(analyzed) {
                Ok(data) => successes.lock().unwrap().push((file.clone(), data)),
                Err(err) => {
                    pb.suspend(|| error!("Error in file '{}': {:?}", file.to_string_lossy(), err));
                    errors.lock().unwrap().push((file, err));
                }
            }

            Ok(())
        })
        .collect::<Result<()>>()?;

    pb.finish();

    let successes = successes.into_inner().unwrap();
    let errors = errors.into_inner().unwrap();

    let files_count = files.len();
    let results_count = successes.len() + errors.len();

    if results_count != files_count {
        bail!(
            "Found invalid number of results returned by ExifTool: expected {}, found {}",
            files_count,
            results_count
        );
    }

    if !errors.is_empty() {
        error!(
            "Failed with the following errors:\n{}",
            errors
                .iter()
                .map(|(success, err)| format!("* {}: {err:?}", success.to_string_lossy()))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    Ok(successes)
}

#[derive(Deserialize)]
pub struct ExifToolOutput(pub Vec<ExifToolFile>);
