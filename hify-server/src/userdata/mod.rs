use serde::{Deserialize, Serialize};

use crate::index::TrackID;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserData {
    history: Vec<TrackID>,
    history_size: usize,
}

impl UserData {
    pub fn new(history_size: usize) -> Self {
        Self {
            history: vec![],
            history_size,
        }
    }
}

pub struct UserDataWrapper {
    inner: UserData,
    on_change: Box<dyn Fn(&UserData) + Send + Sync>,
}

impl UserDataWrapper {
    pub fn new(inner: UserData, on_change: Box<dyn Fn(&UserData) + Send + Sync>) -> Self {
        Self { inner, on_change }
    }

    pub fn history(&self) -> &[TrackID] {
        &self.inner.history
    }

    pub fn history_push(&mut self, track_id: TrackID) {
        if self.inner.history.len() == self.inner.history_size {
            self.inner.history.remove(0);
        }

        self.inner.history.insert(0, track_id);

        (self.on_change)(&self.inner);
    }
}
