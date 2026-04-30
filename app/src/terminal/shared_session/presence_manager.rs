//! Stub — shared session presence manager removed from OpenWarp.
#![allow(dead_code, unused_imports, unused_variables)]

use pathfinder_color::ColorU;
use session_sharing_protocol::common::{
    ParticipantId, ParticipantInfo, ParticipantList, ParticipantPresenceUpdate,
    Role, RoleRequestId, Selection,
};
use warpui::{AppContext, Entity, ModelContext, SingletonEntity};

use crate::auth::UserUid;
use crate::editor::{CursorColors, PeerSelectionData};
use crate::terminal::model::terminal_model::BlockIndex;
use crate::terminal::model::blocks::BlockList;

pub const MUTED_PARTICIPANT_COLOR: ColorU = ColorU {
    r: 176,
    g: 176,
    b: 176,
    a: 255,
};

pub const MUTED_AVATAR_BORDER_COLOR: ColorU = ColorU {
    r: 138,
    g: 138,
    b: 138,
    a: 255,
};

pub fn text_selection_color(participant_color: ColorU) -> ColorU {
    participant_color
}

#[derive(Clone, Debug)]
pub struct Participant {
    pub info: ParticipantInfo,
    pub color: ColorU,
    pub role: Option<Role>,
}

impl Participant {
    pub fn id(&self) -> &ParticipantId {
        &self.info.id
    }

    pub fn display_name(&self) -> &str {
        &self.info.profile_data.display_name
    }
}

pub struct ParticipantAtSelectedBlock<'a> {
    pub participant: &'a Participant,
    pub is_top_of_continuous_selection: bool,
    pub is_bottom_of_continuous_selection: bool,
    pub should_show_avatar: bool,
}

pub enum PresenceManagerEvent {}

pub struct PresenceManager {
    _participant_id: ParticipantId,
}

impl PresenceManager {
    pub fn new_for_sharer(id: ParticipantId, _firebase_uid: UserUid) -> Self {
        Self {
            _participant_id: id,
        }
    }

    pub fn new_for_viewer(id: ParticipantId, _firebase_uid: UserUid, _ctx: &mut ModelContext<Self>) -> Self {
        Self {
            _participant_id: id,
        }
    }

    pub fn id(&self) -> ParticipantId {
        self._participant_id.clone()
    }

    pub fn firebase_uid(&self) -> UserUid {
        UserUid::default()
    }

    pub fn sharer_id(&self) -> ParticipantId {
        self._participant_id.clone()
    }

    pub fn role(&self) -> Option<Role> {
        None
    }

    pub fn viewer_role(&self, _viewer_id: &ParticipantId) -> Option<Role> {
        None
    }

    pub fn get_role_request(&self, _participant_id: &ParticipantId) -> Option<&RoleRequestId> {
        None
    }

    pub fn get_sharer(&self) -> Option<&Participant> {
        None
    }

    pub fn all_present_participants(&self) -> std::iter::Empty<&Participant> {
        std::iter::empty()
    }

    pub fn get_participant(&self, _id: &ParticipantId) -> Option<&Participant> {
        None
    }

    pub fn get_participants_at_selected_block(
        &self,
        _block_index: BlockIndex,
        _block_list: &BlockList,
    ) -> Vec<ParticipantAtSelectedBlock<'_>> {
        Vec::new()
    }

    pub fn update_participants(
        &mut self,
        _participants: ParticipantList,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn update_participant_presence(
        &mut self,
        _update: ParticipantPresenceUpdate,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn is_reconnecting(&self) -> bool {
        false
    }

    pub fn set_is_reconnecting(&mut self, _is_reconnecting: bool) {}

    pub fn peer_selections_for_block(
        &self,
        _block_index: BlockIndex,
    ) -> Vec<PeerSelectionData> {
        Vec::new()
    }
}

impl Entity for PresenceManager {
    type Event = PresenceManagerEvent;
}
