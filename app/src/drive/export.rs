//! Stub: Export manager removed (cloud-only feature).
//! ExportManager is kept as a no-op singleton since it's referenced in many places.
#![allow(dead_code, unused)]

use std::path::PathBuf;

use warpui::{AppContext, Entity, ModelContext, SingletonEntity, WindowId};

use super::CloudObjectTypeAndId;

/// No-op export event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportEvent {
    Canceled(ExportId),
    Failed { id: ExportId },
    Completed { id: ExportId, path: PathBuf },
}

/// Identifier for an export (stubbed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExportId;

/// Singleton model for exporting from Warp Drive (stubbed - no-op).
pub struct ExportManager {
    _unused: (),
}

impl ExportManager {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self { _unused: () }
    }

    /// No-op export.
    pub fn export(
        &mut self,
        _window_id: WindowId,
        _objects: &[CloudObjectTypeAndId],
        _ctx: &mut ModelContext<Self>,
    ) {
        log::info!("Export is disabled in OpenWarp");
    }
}

impl Entity for ExportManager {
    type Event = ExportEvent;
}

impl SingletonEntity for ExportManager {}

/// Replaces characters that are not allowed in a path name.
pub fn safe_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| {
            if c.is_control()
                || matches!(c, '/' | ':' | '#' | '*' | '<' | '>' | '?' | '\\' | '|')
            {
                '_'
            } else {
                c
            }
        })
        .collect()
}
