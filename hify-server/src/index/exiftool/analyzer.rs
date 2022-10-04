use std::{
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    time::Instant,
};

use anyhow::{anyhow, bail, Context, Result};
use once_cell::sync::Lazy;
use pomsky_macro::pomsky;
use regex::Regex;
use serde::Deserialize;

use crate::{index::TrackMetadata, utils::progress::display_progress};

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
        eprintln!(
            "Warning: in file '{}': file format unsupported by web players: {audio_ext}",
            path.to_string_lossy()
        );

        return false;
    }

    matches!(
        audio_ext.as_str(),
        "mp3" | "flac" | "wav" | "aac" | "ogg" | "m4a"
    )
}

pub fn run_on(files: &[PathBuf]) -> Result<Vec<(PathBuf, TrackMetadata)>> {
    let started = Instant::now();
    let mut previous = 0;

    if files.is_empty() {
        println!("        Nothing to do!");
        return Ok(vec![]);
    }

    print!("        Starting analysis...");

    let files_count = files.len();

    const FILES_PER_CHUNK: usize = 100;

    let mut successes = vec![];
    let mut errors = vec![];

    for (chunk_num, files) in files.chunks(FILES_PER_CHUNK).enumerate() {
        let chunk_start = FILES_PER_CHUNK * chunk_num;

        let mut handle = Command::new("exiftool")
            .args(&["-n", "-json", "-progress"])
            .args(files)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to launch ExifTool")?;

        let stdout_reader = BufReader::new(
            handle
                .stdout
                .take()
                .context("Failed to get command's STDOUT")?,
        );

        let stderr_reader = BufReader::new(
            handle
                .stderr
                .take()
                .context("Failed to get the command's SDTERR")?,
        );

        let stdout_lines = Arc::new(Mutex::new(vec![]));
        let stderr_lines = Arc::new(Mutex::new(vec![]));

        let stdout_lines_for_reader = Arc::clone(&stdout_lines);
        let stderr_lines_for_reader = Arc::clone(&stderr_lines);

        std::thread::spawn(move || {
            for line in stdout_reader.lines() {
                match line {
                    Ok(line) => stdout_lines_for_reader.lock().unwrap().push(line),
                    Err(err) => {
                        eprintln!("{err:?}");
                        // TODO
                    }
                }
            }
        });

        std::thread::spawn(move || {
            for line in stderr_reader.lines() {
                match line {
                    Ok(line) => match PARSE_PROGRESS_LINE.captures(&line) {
                        Some(m) => {
                            let current = m
                                .name("current")
                                .unwrap()
                                .as_str()
                                .parse::<usize>()
                                .unwrap();

                            let elapsed = started.elapsed().as_secs();

                            if elapsed != previous || current == files_count {
                                previous = elapsed;
                                display_progress(elapsed, chunk_start + current, files_count, 0);
                            }
                        }
                        None => {
                            stderr_lines_for_reader.lock().unwrap().push(line);
                        }
                    },
                    Err(err) => {
                        eprintln!("{err:?}");
                        // TODO
                    }
                }
            }
        });

        let status = handle.wait().with_context(|| {
            format!(
                "ExifTool failed: {}",
                stderr_lines.lock().unwrap().join("\n")
            )
        })?;

        if !status.success() {
            bail!(
                "ExifTool failed: {}",
                stderr_lines.lock().unwrap().join("\n")
            );
        }

        let stdout_lines = stdout_lines.lock().unwrap().join("\n");

        let parsed_output = serde_json::from_str::<ExifToolOutput>(&stdout_lines).map_err(|e| {
            anyhow!(
                "Failed to parse ExifTool output: {}\n\n{}",
                e,
                stdout_lines
                    .lines()
                    .enumerate()
                    .skip(if e.line() < 15 { 0 } else { e.line() - 15 })
                    .take(15)
                    .map(|(i, line)| format!("Line {: >5} *| {line}", i + 1))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        })?;

        for (i, analyzed) in parsed_output.0.into_iter().enumerate() {
            match process_analyzed_file(analyzed) {
                Ok(data) => successes.push((files.get(i).unwrap().clone(), data)),
                Err(err) => {
                    let file = files.get(i).unwrap();
                    eprintln!("Error in file '{}': {:?}", file.to_string_lossy(), err);
                    errors.push((file, err));
                }
            }
        }
    }

    println!();

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
        bail!(
            "Failed with the following errors:\n\n{}",
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

static PARSE_PROGRESS_LINE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(pomsky!(
        Start
            "======== " Codepoint+ " [" :current([digit]+) "/" [digit]+ "]"
        End
    ))
    .unwrap()
});
