//! Stub — shared session adapter removed from OpenWarp.
#![allow(dead_code, unused_imports, unused_variables)]

use chrono::{DateTime, Local};
use session_sharing_protocol::common::SessionId;
use warpui::{AppContext, Element, ModelHandle, ViewHandle};

use crate::banner::{Banner, BannerTextContent};
use crate::terminal::shared_session::participant_avatar_view::ParticipantAvatarView;
use crate::terminal::shared_session::presence_manager::PresenceManager;
use crate::terminal::view::TerminalAction;

pub enum Kind {
    Viewer(()),
    Sharer(()),
    Stub,
}

impl Kind {
    pub fn as_viewer(&self) -> Option<&()> {
        None
    }
    pub fn as_viewer_mut(&mut self) -> Option<&mut ()> {
        None
    }
    pub fn as_sharer(&self) -> Option<&()> {
        None
    }
    pub fn as_sharer_mut(&mut self) -> Option<&mut ()> {
        None
    }
    pub fn is_sharer(&self) -> bool {
        false
    }
    pub fn is_viewer(&self) -> bool {
        false
    }
}

pub struct Adapter {
    kind: Kind,
    presence_manager: ModelHandle<PresenceManager>,
    session_id: SessionId,
    started_at: DateTime<Local>,
}

impl Adapter {
    pub fn kind(&self) -> &Kind {
        &self.kind
    }
    pub fn presence_manager(&self) -> &ModelHandle<PresenceManager> {
        &self.presence_manager
    }
    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }
    pub fn started_at(&self) -> &DateTime<Local> {
        &self.started_at
    }
    pub fn pane_header_viewer_avatars(&self, _app: &AppContext) -> Vec<ViewHandle<ParticipantAvatarView>> {
        Vec::new()
    }
    pub fn presence_avatars(&self, _app: &AppContext) -> Vec<ViewHandle<ParticipantAvatarView>> {
        Vec::new()
    }
    pub fn reconnecting_banner(&self) -> Option<&ViewHandle<Banner<TerminalAction>>> {
        None
    }
}
