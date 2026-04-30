//! Stub: Oz launch modal removed (cloud-only feature).
#![allow(dead_code, unused)]

use super::{CTAButton, CheckboxConfig, LaunchModalEvent, Slide};
use crate::ui_components::icons::Icon;
use markdown_parser::FormattedTextLine;
use warpui::assets::asset_cache::AssetSource;
use warpui::AppContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OzLaunchSlide {
    Placeholder,
}

impl Slide for OzLaunchSlide {
    fn modal_title(&self) -> String {
        String::new()
    }

    fn modal_subtext_paragraphs(&self) -> Vec<FormattedTextLine> {
        vec![]
    }

    fn first() -> Self {
        OzLaunchSlide::Placeholder
    }

    fn next(&self) -> Option<Self> {
        None
    }

    fn prev(&self) -> Option<Self> {
        None
    }

    fn display_text(&self) -> Option<&'static str> {
        None
    }

    fn short_label(&self) -> &'static str {
        ""
    }

    fn title(&self) -> &'static str {
        ""
    }

    fn title_icon(&self) -> Option<Icon> {
        None
    }

    fn content(&self) -> &'static str {
        ""
    }

    fn image(&self) -> AssetSource {
        AssetSource::Bundled { path: "" }
    }

    fn all() -> Vec<Self> {
        vec![OzLaunchSlide::Placeholder]
    }

    fn cta_button(&self) -> CTAButton<Self> {
        CTAButton::close("Close")
    }
}

pub fn init(app: &mut warpui::AppContext) {
    super::init::<OzLaunchSlide>(app);
}
