//! Stub — shared session manager removed.
#![allow(dead_code, unused_imports, unused_variables)]

use session_sharing_protocol::common::SessionId;
use warpui::{AppContext, Entity, EntityId, ModelContext, SingletonEntity, ViewHandle, WeakViewHandle, WindowId};

use crate::terminal::TerminalView;

pub struct Manager;

pub enum ManagerEvent {
    ShareAttempted,
    StartedShare {
        session_id: SessionId,
        window_id: WindowId,
    },
    JoinedSession {
        session_id: SessionId,
        view_id: EntityId,
    },
    StoppedShare,
    FailedToShare {
        window_id: WindowId,
    },
}

impl Manager {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }

    pub fn is_some_session_being_shared(&self) -> bool {
        false
    }

    pub fn is_some_session_being_viewed(&self) -> bool {
        false
    }

    pub fn session_id(&self, _terminal_view_id: &EntityId) -> Option<SessionId> {
        None
    }

    pub fn ended_session_id(&self, _terminal_view_id: &EntityId) -> Option<SessionId> {
        None
    }

    pub fn shared_view_by_id(
        &self,
        _terminal_view_id: &EntityId,
        _ctx: &AppContext,
    ) -> Option<ViewHandle<TerminalView>> {
        None
    }

    pub fn shared_view_ids(&self) -> std::iter::Empty<EntityId> {
        std::iter::empty()
    }

    pub fn joined_view_ids(&self) -> std::iter::Empty<EntityId> {
        std::iter::empty()
    }

    pub fn shared_views<'a>(&'a self, _ctx: &'a AppContext) -> std::iter::Empty<ViewHandle<TerminalView>> {
        std::iter::empty()
    }

    pub fn started_share(
        &mut self,
        _terminal_view: WeakViewHandle<TerminalView>,
        _session_id: SessionId,
        _window_id: WindowId,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn joined_share(
        &mut self,
        _terminal_view: WeakViewHandle<TerminalView>,
        _session_id: SessionId,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn left_share(&mut self, _terminal_view_id: EntityId) {}

    pub fn stopped_share(&mut self, _terminal_view_id: EntityId, _ctx: &mut ModelContext<Self>) {}

    pub fn share_failed(&mut self, _window_id: WindowId, _ctx: &mut ModelContext<Self>) {}

    pub fn stop_all_shared_sessions(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn rejoin_all_shared_sessions(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn clear_joined(&mut self) {}
}

impl Entity for Manager {
    type Event = ManagerEvent;
}

impl SingletonEntity for Manager {}
