use std::collections::{hash_map::Entry, HashMap};

use serde::{Deserialize, Serialize};

use crate::index::TrackID;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserData {
    history: Vec<TrackID>,
    history_size: usize,
    listenings: HashMap<TrackID, u32>,
}

impl UserData {
    pub fn new(history_size: usize) -> Self {
        Self {
            history: vec![],
            history_size,
            listenings: HashMap::new(),
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
        match self.inner.history.iter().position(|t| *t == track_id) {
            Some(index) => {
                self.inner.history.remove(index);
            }

            None => {
                if self.inner.history.len() == self.inner.history_size {
                    self.inner.history.remove(0);
                }
            }
        }

        self.inner.history.insert(0, track_id.clone());

        match self.inner.listenings.entry(track_id) {
            Entry::Occupied(mut occ) => *occ.get_mut() += 1,
            Entry::Vacant(vac) => {
                vac.insert(1);
            }
        };

        (self.on_change)(&self.inner);
    }
}
