//! Stub — shared session participant avatar view removed from OpenWarp.
#![allow(dead_code, unused_imports, unused_variables)]

use session_sharing_protocol::common::Role;
use warpui::{
    elements::{Empty, MouseStateHandle},
    AppContext, Element, Entity, View, ViewContext, ViewHandle,
};

use crate::menu::Menu;
use crate::pane_group::PaneHeaderAction;
use crate::terminal::view::TerminalAction;

pub enum ParticipantAvatarEvent {}

pub struct ParticipantAvatarView;

impl Entity for ParticipantAvatarView {
    type Event = ParticipantAvatarEvent;
}

impl View for ParticipantAvatarView {
    fn ui_name() -> &'static str {
        "ParticipantAvatarView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

pub fn render_participants_and_role_elements(
    _participants: Vec<ViewHandle<ParticipantAvatarView>>,
    _role: Option<Role>,
    _mouse_state_handle: MouseStateHandle,
    _menu_handle: Option<ViewHandle<Menu<PaneHeaderAction<TerminalAction, TerminalAction>>>>,
    _is_menu_open: bool,
    _hide_role_change_button: bool,
    _app: &AppContext,
) -> Box<dyn Element> {
    Empty::new().finish()
}
