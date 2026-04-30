//! Stub: Cloud action confirmation dialog removed (cloud-only feature).
#![allow(dead_code, unused)]

use warpui::{
    elements::MouseStateHandle,
    AppContext, Element, Entity, SingletonEntity, TypedActionView, View, ViewContext,
};

pub enum CloudActionConfirmationDialogEvent {
    Cancel,
    Confirm,
}

#[derive(Debug)]
pub enum CloudActionConfirmationDialogAction {
    Cancel,
    Confirm,
}

#[derive(Default)]
pub enum CloudActionConfirmationDialogVariant {
    LeaveTeam,
    DeleteTeam,
    #[default]
    None,
}

pub struct CloudActionConfirmationDialog {
    _unused: (),
}

impl CloudActionConfirmationDialog {
    pub fn new() -> Self {
        Self { _unused: () }
    }

    pub fn set_variant(&mut self, _variant: CloudActionConfirmationDialogVariant) {}

    pub fn set_confirmation_button_enabled(&mut self, _enabled: bool) {}
}

impl Entity for CloudActionConfirmationDialog {
    type Event = CloudActionConfirmationDialogEvent;
}

impl View for CloudActionConfirmationDialog {
    fn ui_name() -> &'static str {
        "CloudActionConfirmationDialog"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        warpui::elements::Empty::new().finish()
    }
}

impl TypedActionView for CloudActionConfirmationDialog {
    type Action = CloudActionConfirmationDialogAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            CloudActionConfirmationDialogAction::Cancel => {
                ctx.emit(CloudActionConfirmationDialogEvent::Cancel)
            }
            CloudActionConfirmationDialogAction::Confirm => {
                ctx.emit(CloudActionConfirmationDialogEvent::Confirm)
            }
        }
    }
}
