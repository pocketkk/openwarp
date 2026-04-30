//! Stub — share session modal removed from OpenWarp.
#![allow(dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use parking_lot::FairMutex;
use warpui::{elements::Empty, AppContext, Element, Entity, EntityId, TypedActionView, View, ViewContext};

use crate::pane_group::TerminalPaneId;
use crate::terminal::TerminalModel;
use super::{SharedSessionActionSource, SharedSessionScrollbackType};

pub fn init(_app: &mut AppContext) {}

pub enum ShareSessionModalEvent {
    Close,
    StartSharing {
        terminal_pane_id: TerminalPaneId,
        scrollback_type: SharedSessionScrollbackType,
        source: SharedSessionActionSource,
    },
    Upgrade,
}

#[derive(Debug)]
pub enum ShareSessionModalAction {
    Cancel,
}

pub struct ShareSessionModal;

impl ShareSessionModal {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn open(
        &mut self,
        _terminal_pane_id: TerminalPaneId,
        _open_source: SharedSessionActionSource,
        _model: Arc<FairMutex<TerminalModel>>,
        _terminal_view_id: EntityId,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn open_denied(&mut self, _terminal_pane_id: TerminalPaneId, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for ShareSessionModal {
    type Event = ShareSessionModalEvent;
}

impl View for ShareSessionModal {
    fn ui_name() -> &'static str {
        "ShareSessionModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for ShareSessionModal {
    type Action = ShareSessionModalAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
