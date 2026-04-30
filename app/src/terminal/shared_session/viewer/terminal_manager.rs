//! Stub — shared session viewer terminal manager removed from OpenWarp.
//! This type exists only so call sites compile; construction panics at runtime
//! since shared-session viewing is disabled.

#![allow(dead_code, unused_imports, unused_variables)]

use std::any::Any;
use std::sync::Arc;

use parking_lot::FairMutex;
use pathfinder_geometry::vector::Vector2F;
use session_sharing_protocol::common::SessionId;
use warpui::{AppContext, ModelHandle, ViewHandle, WindowId};

use crate::pane_group::pane::DetachType;
use crate::pane_group::TerminalViewResources;
use crate::terminal::{TerminalModel, TerminalView};

pub struct TerminalManager {
    model: Arc<FairMutex<TerminalModel>>,
    view: ViewHandle<TerminalView>,
}

impl TerminalManager {
    pub fn new(
        _session_id: SessionId,
        _resources: TerminalViewResources,
        _initial_size: Vector2F,
        _window_id: WindowId,
        _ctx: &mut AppContext,
    ) -> Self {
        panic!("Shared session viewer has been removed from OpenWarp");
    }

    pub fn new_deferred(
        _resources: TerminalViewResources,
        _initial_size: Vector2F,
        _window_id: WindowId,
        _ctx: &mut AppContext,
    ) -> Self {
        panic!("Shared session viewer has been removed from OpenWarp");
    }

    pub fn connect_to_session(&mut self, _session_id: SessionId, _ctx: &mut AppContext) -> bool {
        false
    }
}

impl crate::terminal::TerminalManager for TerminalManager {
    fn model(&self) -> Arc<FairMutex<TerminalModel>> {
        self.model.clone()
    }

    fn view(&self) -> ViewHandle<TerminalView> {
        self.view.clone()
    }

    fn on_view_detached(&self, _detach_type: DetachType, _app: &mut AppContext) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
