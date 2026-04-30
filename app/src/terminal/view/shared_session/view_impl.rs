//! Stub — shared session view implementation removed from OpenWarp.
//! These no-op methods satisfy callers that haven't been cleaned up yet.

#![allow(dead_code, unused_imports, unused_variables)]

use session_sharing_protocol::common::{
    ParticipantId, ParticipantPresenceUpdate, Role, RoleRequestId, RoleRequestResponse, WindowSize,
};
use session_sharing_protocol::sharer::{RoleUpdateReason, SessionSourceType};
use warpui::elements::Empty;
use warpui::{AppContext, Element, ModelHandle, ViewContext};

use crate::auth::UserUid;
use crate::menu::MenuItem;
use crate::terminal::model::terminal_model::BlockIndex;
use crate::terminal::shared_session::presence_manager::PresenceManager;
use crate::terminal::shared_session::{
    SharedSessionActionSource, SharedSessionScrollbackType, SharedSessionStatus,
};
use crate::terminal::view::{TerminalAction, TerminalView};
use crate::terminal::TerminalModel;

use super::adapter::{Adapter, Kind};

impl TerminalView {
    pub fn sharer_session_kind(&self) -> Option<&Kind> {
        None
    }

    pub fn shared_session_sharer(&self) -> Option<&()> {
        None
    }

    pub fn shared_session_viewer(&self) -> Option<&()> {
        None
    }

    pub fn shared_session_viewer_mut(&mut self) -> Option<&mut ()> {
        None
    }

    pub fn shared_session_presence_manager(&self) -> Option<ModelHandle<PresenceManager>> {
        None
    }

    pub fn shared_session_id(&self) -> Option<&session_sharing_protocol::common::SessionId> {
        None
    }

    pub(crate) fn is_shared_session_for_ambient_agent(&self) -> bool {
        false
    }

    pub fn update_session_link_permissions(
        &mut self,
        _role: Option<Role>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn update_session_team_permissions(
        &mut self,
        _role: Option<Role>,
        _team_uid: String,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn update_role(
        &mut self,
        _participant_id: ParticipantId,
        _role: Role,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn update_role_for_user(
        &mut self,
        _user_uid: UserUid,
        _role: Role,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn update_role_for_pending_user(
        &mut self,
        _email: String,
        _role: Role,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn add_guests(
        &mut self,
        _emails: Vec<String>,
        _role: Role,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn remove_guest(&mut self, _user_uid: UserUid, _ctx: &mut ViewContext<Self>) {}

    pub fn remove_pending_guest(&mut self, _email: String, _ctx: &mut ViewContext<Self>) {}

    pub fn on_role_requested(
        &mut self,
        _participant_id: ParticipantId,
        _role_request_id: RoleRequestId,
        _role: Role,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn on_role_request_cancelled(
        &mut self,
        _participant_id: ParticipantId,
        _role_request_id: RoleRequestId,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn open_share_session_modal(
        &mut self,
        _source: SharedSessionActionSource,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn open_share_session_denied_modal(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn attempt_to_share_session(
        &mut self,
        _scrollback_type: SharedSessionScrollbackType,
        _source: Option<SharedSessionActionSource>,
        _source_type: SessionSourceType,
        _auto_share: bool,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn on_session_share_started(
        &mut self,
        _sharer_id: ParticipantId,
        _sharer_firebase_uid: UserUid,
        _scrollback_type: SharedSessionScrollbackType,
        _session_id: session_sharing_protocol::common::SessionId,
        _source_type: SessionSourceType,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn stop_sharing_session(
        &mut self,
        _source: SharedSessionActionSource,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn on_session_share_ended(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn insert_conversation_ended_tombstone(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn on_shared_session_reconnection_status_changed(
        &mut self,
        _is_reconnecting: bool,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn session_sharing_context_menu_items(
        &self,
        _model: &TerminalModel,
        _is_disabled: bool,
    ) -> Vec<MenuItem<TerminalAction>> {
        Vec::new()
    }

    pub fn get_shared_session_presence_selection(
        &self,
        _ctx: &AppContext,
    ) -> Option<session_sharing_protocol::common::Selection> {
        None
    }

    pub fn pane_header_overflow_menu_toggled(
        &mut self,
        _is_open: bool,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn open_shared_session_viewer_role_menu(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn make_all_shared_session_participants_readers(
        &mut self,
        _reason: RoleUpdateReason,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn request_shared_session_role(&mut self, _role: Role, _ctx: &mut ViewContext<Self>) {}

    pub fn open_shared_session_on_desktop(
        &mut self,
        _source: SharedSessionActionSource,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn cancel_shared_session_role_request(
        &mut self,
        _role_request_id: RoleRequestId,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn respond_to_shared_session_role_request(
        &mut self,
        _participant_id: ParticipantId,
        _role_request_id: RoleRequestId,
        _response: RoleRequestResponse,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn copy_shared_session_link(
        &mut self,
        _source: SharedSessionActionSource,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn on_participant_presence_updated(
        &mut self,
        _update: &ParticipantPresenceUpdate,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn on_participant_role_changed(
        &mut self,
        _participant_id: &ParticipantId,
        _role: Role,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(crate) fn is_viewer_driven_sizing_eligible(
        &mut self,
        _check_feature: bool,
        _ctx: &mut ViewContext<Self>,
    ) -> bool {
        false
    }

    pub(crate) fn restore_pty_to_sharer_size(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub(in crate::terminal::view) fn force_report_viewer_terminal_size(
        &mut self,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(crate) fn resize_from_viewer_report(
        &mut self,
        _window_size: WindowSize,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn render_input_request_edit_access_button(
        &self,
        _ctx: &AppContext,
    ) -> Box<dyn Element> {
        Empty::new().finish()
    }

    pub fn handle_inactivity_modal_event(&mut self, _ctx: &mut ViewContext<Self>) {}
}
