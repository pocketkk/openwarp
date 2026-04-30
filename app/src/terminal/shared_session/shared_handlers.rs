//! Stub — shared session handlers removed from OpenWarp.
//! These functions were used to sync state between shared session participants.
//! Without sharing, they are no-ops, but callers still reference them.

#![allow(dead_code, unused_imports, unused_variables)]

use std::cell::Cell;
use std::rc::Rc;

use session_sharing_protocol::common::{
    CLIAgentSessionState, InputMode, SelectedAgentModel,
    SelectedConversation, ServerConversationToken, UniversalDeveloperInputContextUpdate,
};
use warpui::{AppContext, ModelHandle, WeakViewHandle};

use crate::ai::blocklist::agent_view::AgentViewController;
use crate::ai::blocklist::{BlocklistAIContextModel, BlocklistAIHistoryModel};
use crate::terminal::TerminalView;

pub(crate) struct ActiveRemoteUpdate;

#[derive(Clone)]
pub(crate) struct RemoteUpdateGuard {
    is_remote_update_in_progress: Rc<Cell<bool>>,
}

impl RemoteUpdateGuard {
    pub(crate) fn new() -> Self {
        Self {
            is_remote_update_in_progress: Rc::new(Cell::new(false)),
        }
    }

    pub(crate) fn should_broadcast(&self) -> bool {
        !self.is_remote_update_in_progress.get()
    }

    pub(crate) fn start_remote_update(&self) -> ActiveRemoteUpdate {
        self.is_remote_update_in_progress.set(true);
        ActiveRemoteUpdate
    }
}

impl Drop for ActiveRemoteUpdate {
    fn drop(&mut self) {
        // no-op stub
    }
}

pub(crate) fn apply_selected_agent_model_update(
    _terminal_view_id: warpui::EntityId,
    _selected_model: &SelectedAgentModel,
    _guard: &ActiveRemoteUpdate,
    _ctx: &mut AppContext,
) {
}

pub(crate) fn apply_input_mode_update(
    _weak_view_handle: &WeakViewHandle<TerminalView>,
    _input_mode: &InputMode,
    _guard: &ActiveRemoteUpdate,
    _ctx: &mut AppContext,
) {
}

pub(crate) fn apply_auto_approve_agent_actions_update(
    _weak_view_handle: &WeakViewHandle<TerminalView>,
    _auto_approve: bool,
    _guard: &ActiveRemoteUpdate,
    _ctx: &mut AppContext,
) {
}

pub(crate) fn apply_selected_conversation_update(
    _weak_view_handle: &WeakViewHandle<TerminalView>,
    _selected_conversation: &SelectedConversation,
    _guard: &ActiveRemoteUpdate,
    _ctx: &mut AppContext,
) {
}

pub(crate) fn build_selected_conversation_update(
    _agent_view_controller: &ModelHandle<AgentViewController>,
    _ai_context_model: &ModelHandle<BlocklistAIContextModel>,
    _ctx: &mut AppContext,
) -> Option<UniversalDeveloperInputContextUpdate> {
    None
}

pub(crate) fn apply_cli_agent_state_update(
    _weak_view_handle: &WeakViewHandle<TerminalView>,
    _state: &CLIAgentSessionState,
    _guard: &ActiveRemoteUpdate,
    _ctx: &mut AppContext,
) {
}
