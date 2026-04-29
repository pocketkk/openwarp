//! Stub: shared-objects-creation-denied body removed in OpenWarp fork.
#![allow(unused, dead_code)]

use crate::drive::DriveObjectType;
use crate::workspaces::workspace::CustomerType;
use warpui::{
    elements::{Element, Empty, ParentElement},
    AppContext, Entity, TypedActionView, View, ViewContext,
};

pub struct SharedObjectsCreationDeniedBody;

#[derive(Debug, Clone, Copy)]
pub enum SharedObjectsCreationDeniedBodyAction {
    Upgrade,
    ManageBilling,
}

pub enum SharedObjectsCreationDeniedBodyEvent {
    Upgrade,
    ManageBilling,
}

impl SharedObjectsCreationDeniedBody {
    pub fn new(_object_type: Option<DriveObjectType>) -> Self {
        Self
    }

    pub fn update_state(
        &mut self,
        _object_type: DriveObjectType,
        _has_admin_permissions: bool,
        _is_delinquent_due_to_payment_issue: bool,
        _customer_type: CustomerType,
        _ctx: &mut ViewContext<Self>,
    ) {
    }
}

impl Entity for SharedObjectsCreationDeniedBody {
    type Event = SharedObjectsCreationDeniedBodyEvent;
}

impl View for SharedObjectsCreationDeniedBody {
    fn ui_name() -> &'static str {
        "SharedObjectsCreationDeniedBody"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for SharedObjectsCreationDeniedBody {
    type Action = SharedObjectsCreationDeniedBodyAction;
}
