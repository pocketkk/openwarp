//! Stub: Cloud object naming dialog removed (cloud-only feature).
#![allow(dead_code, unused)]

use warpui::{AppContext, Element, ViewHandle};

use crate::cloud_object::Space;
use crate::editor::EditorView;
use crate::server::ids::SyncId;

use super::{index::DriveIndexAction, DriveObjectType};

/// Stubbed naming dialog -- all methods are no-ops.
#[derive(Clone)]
pub struct CloudObjectNamingDialog {
    pub title_editor: ViewHandle<EditorView>,
    pub object_type: Option<DriveObjectType>,
    pub space: Option<Space>,
    pub open_for_folder_id: Option<SyncId>,
}

impl CloudObjectNamingDialog {
    pub fn new(title_editor: ViewHandle<EditorView>) -> Self {
        Self {
            title_editor,
            object_type: None,
            space: None,
            open_for_folder_id: None,
        }
    }

    pub fn close(&mut self, _app: &mut AppContext) {
        self.object_type = None;
        self.space = None;
        self.open_for_folder_id = None;
    }

    pub fn open(
        &mut self,
        _object_type: DriveObjectType,
        _space: Space,
        _initial_folder_id: Option<SyncId>,
        _is_rename: bool,
        _existing_name: Option<String>,
        _app: &mut AppContext,
    ) {
    }

    pub fn is_open(&self) -> bool {
        false
    }

    pub fn is_open_for_space(&self, _space: &Space) -> bool {
        false
    }

    pub fn is_open_for_folder(&self, _folder_id: SyncId) -> bool {
        false
    }

    pub fn current_primary_action(&self) -> Option<DriveIndexAction> {
        None
    }

    pub fn title(&self, _app: &AppContext) -> Option<String> {
        None
    }

    pub fn render(
        &self,
        _appearance: &crate::appearance::Appearance,
        _app: &AppContext,
    ) -> Box<dyn Element> {
        warpui::elements::Empty::new().finish()
    }
}
