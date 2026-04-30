//! Stub — role change modal removed from OpenWarp.
#![allow(dead_code, unused_imports, unused_variables)]

use session_sharing_protocol::common::{ParticipantId, Role, RoleRequestId};
use warpui::{elements::Empty, AppContext, Element, Entity, View, ViewContext};

use crate::pane_group::TerminalPaneId;
use super::render_util::ParticipantAvatarParams;

#[derive(Debug, Clone)]
pub enum RoleChangeOpenSource {
    ViewerRequest {
        role: Role,
    },
    SharerResponse {
        participant_id: ParticipantId,
        role_request_id: RoleRequestId,
        role: Role,
    },
    SharerGrant {
        participant_id: ParticipantId,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum RoleChangeCloseSource {
    ViewerRequest,
    SharerResponse,
    SharerGrant,
}

#[derive(Debug, Clone)]
pub enum RoleChangeModalEvent {
    CancelRequest {
        terminal_pane_id: TerminalPaneId,
        role_request_id: RoleRequestId,
    },
    ApproveRequest {
        terminal_pane_id: TerminalPaneId,
        participant_id: ParticipantId,
        role_request_id: RoleRequestId,
        role: Role,
    },
    DenyRequest {
        terminal_pane_id: TerminalPaneId,
        participant_id: ParticipantId,
        role_request_id: RoleRequestId,
    },
    Close {
        source: RoleChangeCloseSource,
    },
    CancelGrant,
    GrantRole {
        terminal_pane_id: TerminalPaneId,
        participant_id: ParticipantId,
        dont_show_again: bool,
    },
}

pub struct RoleChangeModal;

impl Entity for RoleChangeModal {
    type Event = RoleChangeModalEvent;
}

impl RoleChangeModal {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn all_child_modals_are_closed(&self) -> bool {
        true
    }

    pub fn close_for_viewer_request(&mut self, _ctx: &mut ViewContext<Self>) {}
    pub fn close_for_sharer_response(&mut self, _ctx: &mut ViewContext<Self>) {}
    pub fn close_for_sharer_grant(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn set_role_request_id(&mut self, _id: RoleRequestId) {}

    pub fn open_for_viewer_request(
        &mut self,
        _terminal_pane_id: TerminalPaneId,
        _display_name: String,
        _role: Role,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    #[allow(clippy::too_many_arguments)]
    pub fn open_for_sharer_response(
        &mut self,
        _terminal_pane_id: TerminalPaneId,
        _participant_id: ParticipantId,
        _firebase_uid: String,
        _role_request_id: RoleRequestId,
        _params: ParticipantAvatarParams,
        _role: Role,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn open_for_sharer_grant(
        &mut self,
        _terminal_pane_id: TerminalPaneId,
        _participant_id: ParticipantId,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn remove_role_request(
        &mut self,
        _role_request_id: RoleRequestId,
        _ctx: &mut ViewContext<Self>,
    ) {
    }
}

impl View for RoleChangeModal {
    fn ui_name() -> &'static str {
        "SharedSessionRoleChangeModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
