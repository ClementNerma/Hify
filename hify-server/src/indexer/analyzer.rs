use std::{fs::File, path::Path};

use anyhow::{Context, Result, bail};
use symphonia::core::{
    codecs::audio::well_known::{
        CODEC_ID_AAC, CODEC_ID_FLAC, CODEC_ID_MP3, CODEC_ID_OPUS, CODEC_ID_VORBIS,
    },
    formats::{
        FormatOptions, Track, TrackType,
        probe::Hint,
        well_known::{FORMAT_ID_FLAC, FORMAT_ID_ISOMP4, FORMAT_ID_MP3, FORMAT_ID_OGG},
    },
    io::{MediaSourceStream, MediaSourceStreamOptions},
    meta::MetadataOptions,
    units::Timestamp,
};

use crate::{
    index::{TrackAudioCodec, TrackMetadata},
    indexer::tags::convert_symphonia_metadata,
};

use super::tags::TrackStrTags;

pub fn analyze_file(path: &Path) -> Result<(TrackMetadata, TrackStrTags)> {
    let src = File::open(path).context("Failed")?;

    let mss = MediaSourceStream::new(Box::new(src), MediaSourceStreamOptions::default());

    let mut hint = Hint::new();
    hint.with_extension(
        path.extension()
            .context("File does not have an extension")?
            .to_str()
            .context("File extension contains invalid UTF-8 characters")?,
    );

    let mut format_reader = symphonia::default::get_probe()
        .probe(
            &hint,
            mss,
            FormatOptions::default(),
            MetadataOptions::default(),
        )
        .context("Found unsupported codec")?;

    let track_types = format_reader
        .tracks()
        .iter()
        .filter_map(Track::track_type)
        .collect::<Vec<_>>();

    match format_reader.format_info().format {
        FORMAT_ID_OGG => {
            if !matches!(
                track_types.as_slice(),
                &[TrackType::Video, TrackType::Audio]
                    | &[TrackType::Audio, TrackType::Video]
                    | &[TrackType::Audio]
            ) {
                bail!(
                    "Expected one audio and optionally one video track for format OGG, but found {track_types:?}"
                );
            }
        }

        FORMAT_ID_MP3 | FORMAT_ID_FLAC | FORMAT_ID_ISOMP4 => {
            if !matches!(track_types.as_slice(), &[TrackType::Audio]) {
                bail!(
                    "Expected one audio track for format {}, but found {track_types:?}",
                    format_reader.format_info().format,
                );
            }
        }

        _ => bail!(
            "Found unsupported format: {}",
            format_reader.format_info().format
        ),
    }

    let track = format_reader
        .first_track(TrackType::Audio)
        .cloned()
        .unwrap();

    if track.track_type() != Some(TrackType::Audio) {
        bail!("Expected audio track, but found {:?}", track.track_type());
    }

    // Prefer metadata that's provided in the container format, over other tags found during the
    // probe operation.
    let mut mt = format_reader.metadata();

    while mt.current().is_none() {
        if mt.pop().is_none() {
            bail!("No metadata was found in audio file");
        }
    }

    // Get the newest metadata revision, which should be the one that contains the most complete set of tags.
    let rev = mt.skip_to_latest().cloned().unwrap();

    let codec_params = track
        .codec_params
        .clone()
        .context("Codec parameters are missing")?;

    let codec_params = codec_params.audio().unwrap();

    let codec = match codec_params.codec {
        CODEC_ID_MP3 => TrackAudioCodec::MP3,
        CODEC_ID_FLAC => TrackAudioCodec::FLAC,
        CODEC_ID_AAC => TrackAudioCodec::AAC,
        CODEC_ID_VORBIS => TrackAudioCodec::VORBIS,
        CODEC_ID_OPUS => TrackAudioCodec::OPUS,
        _ => bail!("Found unknown codec: {}", codec_params.codec),
    };

    let time_base = track
        .time_base
        .context("No time base found for audio file")?;

    let duration = {
        let dur = track
            .duration
            .context("Missing pre-computed duration in audio file")?;
        let dur = i64::try_from(dur.get()).unwrap();
        let ts = Timestamp::new(dur);
        time_base.calc_time(ts).unwrap()
    };

    let (dur_secs, dur_nanos) = duration.parts();

    Ok((
        TrackMetadata {
            audio_codec: codec,
            duration_s: u32::try_from(dur_secs)
                .context("Audio track is longer than 2^32-1 seconds!")?
                + u32::from(dur_nanos > 500_000_000),
        },
        convert_symphonia_metadata(&rev)?,
    ))
}

// fn analyze_file_duration(
//     reader: &mut Box<dyn FormatReader>,
//     track_id: u32,
//     time_base: TimeBase,
// ) -> Result<Time, Error> {
//     let mut raw_dur = Duration::new(0);

//     loop {
//         // Read next packet
//         let packet = match reader.next_packet() {
//             Ok(Some(packet)) => packet,

//             Ok(None) => {
//                 todo!() // seek
//             }

//             Err(err) => match err {
//                 Error::IoError(err)
//                     if err.kind() == std::io::ErrorKind::UnexpectedEof
//                         && err.to_string() == "end of stream" =>
//                 {
//                     // Do not treat "end of stream" as a fatal error. It's the currently only way a
//                     // format reader can indicate the media is complete.
//                     let ts = Timestamp::new(i64::try_from(raw_dur.get()).unwrap());
//                     break Ok(time_base.calc_time(ts).unwrap());
//                 }

//                 _ => break Err(err),
//             },
//         };

//         // This should always be true since we previously ensured there was only one track in the audio file
//         assert_eq!(packet.track_id, track_id);

//         // Gapless playback is not enabled, so these values should be zero
//         assert_eq!(packet.trim_start.get(), 0);
//         assert_eq!(packet.trim_end.get(), 0);

//         // Add packet duration to the total
//         raw_dur = raw_dur.checked_add(packet.dur).unwrap();
//     }
// }
