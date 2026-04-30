//! Stub — shared session permissions manager removed.
#![allow(dead_code, unused_imports, unused_variables)]

use session_sharing_protocol::common::{Guest, PendingGuest, Role, SessionId, TeamAclData};
use warpui::{Entity, ModelContext, SingletonEntity};

use crate::drive::sharing::SharingAccessLevel;

pub struct SessionPermissionsManager;

pub enum SessionPermissionsEvent {
    GuestsUpdated {
        session_id: SessionId,
        guests: Vec<Guest>,
        pending_guests: Vec<PendingGuest>,
    },
    LinkPermissionsUpdated {
        session_id: SessionId,
        access_level: Option<SharingAccessLevel>,
    },
    TeamPermissionsUpdated {
        session_id: SessionId,
        team_acl: Option<TeamAclData>,
    },
}

impl SessionPermissionsManager {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }

    pub fn updated_guests(
        &mut self,
        _ctx: &mut ModelContext<Self>,
        _session_id: SessionId,
        _guests: Vec<Guest>,
        _pending_guests: Vec<PendingGuest>,
    ) {
    }

    pub fn updated_link_permissions(
        &mut self,
        _session_id: SessionId,
        _role: Option<Role>,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn updated_team_permissions(
        &mut self,
        _session_id: SessionId,
        _team_acl: Option<TeamAclData>,
        _ctx: &mut ModelContext<Self>,
    ) {
    }
}

impl Entity for SessionPermissionsManager {
    type Event = SessionPermissionsEvent;
}

impl SingletonEntity for SessionPermissionsManager {}
