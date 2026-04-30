//! Stub: Empty trash confirmation dialog removed (cloud-only feature).
#![allow(dead_code, unused)]

use warpui::{
    elements::MouseStateHandle,
    AppContext, Element, Entity, SingletonEntity, TypedActionView, View, ViewContext,
};

pub enum EmptyTrashConfirmationEvent {
    Confirm,
    Cancel,
}

#[derive(Debug)]
pub enum EmptyTrashConfirmationAction {
    Confirm,
    Cancel,
}

pub struct EmptyTrashConfirmationDialog {
    _unused: (),
}

impl EmptyTrashConfirmationDialog {
    pub fn new() -> Self {
        Self { _unused: () }
    }
}

impl Entity for EmptyTrashConfirmationDialog {
    type Event = EmptyTrashConfirmationEvent;
}

impl View for EmptyTrashConfirmationDialog {
    fn ui_name() -> &'static str {
        "EmptyTrashConfirmationDialog"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        warpui::elements::Empty::new().finish()
    }
}

impl TypedActionView for EmptyTrashConfirmationDialog {
    type Action = EmptyTrashConfirmationAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            EmptyTrashConfirmationAction::Confirm => ctx.emit(EmptyTrashConfirmationEvent::Confirm),
            EmptyTrashConfirmationAction::Cancel => ctx.emit(EmptyTrashConfirmationEvent::Cancel),
        }
    }
}
