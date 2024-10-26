use crate::{index::Index, userdata::UserData};

pub fn check_correctness(index: &Index, user_data: &UserData) -> Result<(), Vec<String>> {
    let mut errors = vec![];

    macro_rules! error {
        ($message: tt, $($params: tt)*) => {{
            errors.push(format!($message, $($params)*));
        }}
    }

    let Index {
        from: _,
        fingerprint: _,
        tracks,
        cache: _, // TODO: check this as well
        album_arts,
    } = index;

    //
    // Check tracks ID
    //

    for (id, track) in tracks.iter() {
        if track.id != *id {
            error!(
                "Track registered with ID '{id:?}' in map but has ID '{:?}' instead",
                track.id
            );
        }
    }

    //
    // Check album arts
    //

    for (album_id, relative_path) in album_arts.iter() {
        if !index.cache.albums_infos.contains_key(album_id) {
            error!(
                "Art at path '{}' registered for unknown album ID '{album_id:?}'",
                relative_path.display()
            );
        }
    }

    //
    // Check overlaps in listening history
    //

    let history = user_data.history().entries();

    for (i, entry) in history.iter().enumerate().skip(1) {
        if let Some(overlapping_for) = entry.is_overlapping_prev(&history[i - 1]) {
            error!(
                "Entries {i} and {} overlap in listening history (of about {overlapping_for}):\n* {:?}\n* {entry:?}",
                i + 1,
                history[i - 1],
            );
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
