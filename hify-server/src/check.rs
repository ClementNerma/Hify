use crate::{index::Index, userdata::UserDataWrapper};

pub fn check_correctness(index: &Index, user_data: &UserDataWrapper) -> Result<(), Vec<String>> {
    let mut errors = vec![];

    macro_rules! error {
        ($message: tt, $($params: tt)*) => {{
            errors.push(format!($message, $($params)*));
        }}
    }

    //
    // Check tracks ID
    //

    for (id, track) in index.tracks.iter() {
        if track.id != *id {
            error!(
                "Track registered with ID '{id:?}' in map but has ID '{:?}' instead",
                track.id
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
