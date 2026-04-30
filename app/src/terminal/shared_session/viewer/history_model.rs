//! Stub — shared session viewer history model removed from OpenWarp.
#![allow(dead_code)]

use crate::terminal::HistoryEntry;
use warpui::Entity;

#[derive(Default)]
pub struct SharedSessionHistoryModel {
    entries: Vec<HistoryEntry>,
}

impl SharedSessionHistoryModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entries(&self) -> impl Iterator<Item = &HistoryEntry> {
        self.entries.iter()
    }

    pub fn push(&mut self, entry: HistoryEntry) {
        self.entries.push(entry);
    }
}

impl Entity for SharedSessionHistoryModel {
    type Event = ();
}
