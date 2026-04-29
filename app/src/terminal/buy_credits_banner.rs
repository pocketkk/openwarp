//! Stub: buy-credits banner removed in OpenWarp fork.
#![allow(unused, dead_code)]

use warpui::{
    elements::{Element, Empty, ParentElement},
    AppContext, Entity, TypedActionView, View, ViewContext,
};

pub struct BuyCreditsBanner;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuyCreditsBannerEvent {
    OpenBillingAndUsage,
    RefocusInput,
    OpenAutoReloadModal { purchased_credits: i32 },
    ShowAutoReloadError { error_message: String },
}

/// No-op action for the stub view.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuyCreditsBannerAction;

impl BuyCreditsBanner {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn is_denomination_dropdown_open(&self, _app: &AppContext) -> bool {
        false
    }
}

impl Entity for BuyCreditsBanner {
    type Event = BuyCreditsBannerEvent;
}

impl View for BuyCreditsBanner {
    fn ui_name() -> &'static str {
        "BuyCreditsBanner"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for BuyCreditsBanner {
    type Action = BuyCreditsBannerAction;
}
