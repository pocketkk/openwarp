//! Stub: Enable auto-reload modal removed (billing/cloud-only feature).
#![allow(dead_code, unused)]

use warpui::{AppContext, Element, Entity, View, ViewContext};

use crate::view_components::ToastFlavor;

#[derive(Clone, Debug)]
pub enum EnableAutoReloadModalEvent {
    Close,
    ShowToast {
        message: String,
        flavor: ToastFlavor,
    },
}

pub struct EnableAutoReloadModal {
    _unused: (),
}

impl EnableAutoReloadModal {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self { _unused: () }
    }

    pub fn set_selected_denomination_by_credits(
        &mut self,
        _credits: i32,
        _ctx: &mut ViewContext<Self>,
    ) {
    }
}

impl Entity for EnableAutoReloadModal {
    type Event = EnableAutoReloadModalEvent;
}

impl warpui::TypedActionView for EnableAutoReloadModal {
    type Action = ();
}

impl View for EnableAutoReloadModal {
    fn ui_name() -> &'static str {
        "EnableAutoReloadModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        warpui::elements::Empty::new().finish()
    }
}
