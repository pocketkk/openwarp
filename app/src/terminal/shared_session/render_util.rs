//! Stub — shared session render utilities removed.
#![allow(dead_code, unused_imports, unused_variables)]

use pathfinder_color::ColorU;
use warpui::{
    elements::{ChildAnchor, ParentAnchor},
    AppContext, Element,
};

use crate::appearance::Appearance;
use super::presence_manager::Participant;

pub fn shared_session_indicator_color(appearance: &Appearance) -> ColorU {
    appearance.theme().terminal_colors().normal.red.into()
}

pub const SHARED_SESSION_AVATAR_DIAMETER: f32 = 20.;

#[derive(Clone, Debug)]
pub struct ParticipantAvatarParams {
    pub display_name: String,
    pub image_url: Option<String>,
    pub participant_color: ColorU,
    pub is_muted: bool,
    pub tooltip_parent_anchor: ParentAnchor,
    pub tooltip_child_anchor: ChildAnchor,
}

impl ParticipantAvatarParams {
    pub fn new(participant: &Participant, is_self_reconnecting: bool) -> Self {
        Self {
            display_name: participant.info.profile_data.display_name.clone(),
            image_url: participant.info.profile_data.photo_url.clone(),
            participant_color: participant.color,
            is_muted: is_self_reconnecting,
            tooltip_parent_anchor: ParentAnchor::TopRight,
            tooltip_child_anchor: ChildAnchor::BottomRight,
        }
    }
}
