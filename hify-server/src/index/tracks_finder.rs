use anyhow::{Context, Result, anyhow};
use log::info;

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::Metadata,
    panic::AssertUnwindSafe,
    path::{Path, PathBuf},
    time::{Instant, SystemTime},
};
use walkdir::WalkDir;

use crate::{
    logging::spinner,
    runner::{TaskSet, TaskSetOptions},
};

use super::{TracksList, data::Track, metadata};

pub fn log(time: Instant, message: &str) {
    info!("[{: >4}s] {message}", time.elapsed().as_secs());
}

pub fn analyze_tracks_in(dir: &Path, from: Option<&TracksList>) -> Result<TracksList> {
    let from = from.map(|list| list.0.as_slice()).unwrap_or_default();

    let started = Instant::now();

    log(started, "Looking for audio files...");

    let files = build_files_list(dir).context("Failed to build files list")?;

    log(started, &format!("Found a total of {} files", files.len()));

    let files = files
        .into_iter()
        .filter(|(path, _)| metadata::is_audio_file(path))
        .collect::<HashMap<_, _>>();

    log(
        started,
        &format!("...of which {} are audio files.", files.len()),
    );

    // Remove deleted tracks
    let (mut tracks, deleted_tracks) = from
        .iter()
        .partition::<Vec<_>, _>(|track| files.contains_key(&dir.join(&track.relative_path)));

    if !deleted_tracks.is_empty() {
        log(
            started,
            &format!("...detected {} deleted tracks.", deleted_tracks.len()),
        );
    }

    let tracks_files_mtime: HashMap<_, _> = tracks
        .iter()
        .map(|track| (track.relative_path.as_path(), track.mtime))
        .collect();

    let file_times = files
        .iter()
        .map(|(path, times)| (path.clone(), *times))
        .filter(|(path, times)| {
            match tracks_files_mtime.get(path.strip_prefix(dir).expect(
                "Internal error: audio file path couldn't be stripped of the base directory",
            )) {
                None => true,
                Some(old_mtime) => old_mtime != &times.mtime,
            }
        })
        .collect::<BTreeMap<_, _>>();

    log(
        started,
        &format!("...and {} new or modified tracks.", file_times.len()),
    );

    log(started, "Extracting audio metadata...");

    // Run analysis tool on all new and modified files
    let analyzed = metadata::analyze_audio_files(file_times.keys().cloned().collect::<Vec<_>>())?;

    // Turn the analyzed files into tracks
    let analyzed = analyzed
        .into_iter()
        .map(|(path, metadata)| {
            Track::new(
                path.strip_prefix(dir)
                    .expect("Internal error: track path couldn't be stripped of the base directory")
                    .to_path_buf(),
                *file_times.get(&path).unwrap(),
                metadata,
            )
        })
        .collect::<Vec<_>>();

    // Remove previous versions of analyzed files
    let analyzed_file_paths = analyzed
        .iter()
        .map(|track| &track.relative_path)
        .collect::<HashSet<_>>();

    tracks.retain(|track| !analyzed_file_paths.contains(&track.relative_path));

    // Remove previous versions of analyzed tracks
    let analyzed_ids = analyzed
        .iter()
        .map(|track| &track.id)
        .collect::<HashSet<_>>();

    tracks.retain(|track| !analyzed_ids.contains(&track.id));

    // Add new (or modified) tracks
    let tracks = tracks
        .into_iter()
        .cloned()
        .chain(analyzed)
        .collect::<Vec<_>>();

    log(
        started,
        &format!("Collected {} tracks, generating cache...", tracks.len()),
    );

    log(started, "Index has been generated.");

    Ok(TracksList(tracks))
}

#[derive(Clone, Copy)]
pub struct FileTimes {
    pub ctime: Option<SystemTime>,
    pub mtime: SystemTime,
}

fn build_files_list(from: &Path) -> Result<HashMap<PathBuf, FileTimes>> {
    let spinner = spinner("[{elapsed_precise}] Found {pos} files");

    let mut runner = TaskSet::<Result<Option<(PathBuf, FileTimes)>>>::new();

    for item in WalkDir::new(from).min_depth(1) {
        let spinner = spinner.clone();

        // TODO: ensure AssertUnwindSafe is correct here
        runner.add(AssertUnwindSafe(move || {
            let item = item.map_err(|err| anyhow!("Failed to read directory entry: {err:?}"))?;
            let mt = item.metadata().with_context(|| {
                format!("Failed to get metadata for path: {}", item.path().display())
            })?;

            if !mt.is_file() {
                return Ok(None);
            }

            spinner.inc(1);
            Ok(Some((item.path().to_path_buf(), get_file_times(&mt)?)))
        }));
    }

    let mut files = HashMap::new();

    for result in runner.run(TaskSetOptions::default()) {
        if let Some((path, times)) = result?? {
            files.insert(path, times);
        }
    }

    spinner.finish_and_clear();

    Ok(files)
}

fn get_file_times(mt: &Metadata) -> Result<FileTimes> {
    let ctime = mt.created().ok();

    let mtime = mt
        .modified()
        .context("Failed to get the file's modification time")?;

    Ok(FileTimes { ctime, mtime })
}
