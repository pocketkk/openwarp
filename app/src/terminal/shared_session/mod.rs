//! Shared session stubs — cloud session sharing has been removed from OpenWarp.

#![allow(dead_code, unused_imports, unused_variables)]

use serde::{Deserialize, Serialize};
use session_sharing_protocol::common::{Role, SessionId};
use session_sharing_protocol::sharer::SessionSourceType;
use warpui::{id, keymap::ContextPredicate, AppContext, ModelContext};

use super::model::terminal_model::BlockIndex;

pub mod ai_agent;
pub mod manager;
pub mod participant_avatar_view;
pub mod permissions_manager;
pub mod presence_manager;
pub mod render_util;
pub mod replay_agent_conversations;
pub mod role_change_modal;
pub mod settings;
pub mod share_modal;
pub(super) mod shared_handlers;
pub mod sharer;
pub mod viewer;

pub const COPY_LINK_TEXT: &str = "Sharing link copied";

#[derive(Debug, Clone, Default)]
pub enum IsSharedSessionCreator {
    Yes { source_type: SessionSourceType },
    #[default]
    No,
}

#[derive(Debug, Clone, Default)]
pub enum SharedSessionStatus {
    #[default]
    NotShared,
    ViewPending,
    ActiveViewer { role: Role },
    FinishedViewer,
    SharePendingPreBootstrap { source_type: SessionSourceType },
    SharePending,
    ActiveSharer,
}

impl SharedSessionStatus {
    pub fn reader() -> Self {
        Self::ActiveViewer { role: Role::Reader }
    }

    pub fn executor() -> Self {
        Self::ActiveViewer {
            role: Role::Executor,
        }
    }

    pub fn is_view_pending(&self) -> bool {
        matches!(self, Self::ViewPending)
    }

    pub fn is_active_viewer(&self) -> bool {
        matches!(self, Self::ActiveViewer { .. })
    }

    pub fn is_finished_viewer(&self) -> bool {
        matches!(self, Self::FinishedViewer)
    }

    pub fn is_viewer(&self) -> bool {
        self.is_view_pending() || self.is_active_viewer() || self.is_finished_viewer()
    }

    pub fn is_executor(&self) -> bool {
        matches!(self, Self::ActiveViewer { role } if role.can_execute())
    }

    pub fn is_reader(&self) -> bool {
        matches!(self, Self::ActiveViewer { role: Role::Reader })
    }

    pub fn is_share_pending(&self) -> bool {
        matches!(
            self,
            Self::SharePending | Self::SharePendingPreBootstrap { .. }
        )
    }

    pub fn is_active_sharer(&self) -> bool {
        matches!(self, Self::ActiveSharer)
    }

    pub fn is_sharer(&self) -> bool {
        self.is_share_pending() || self.is_active_sharer()
    }

    pub fn is_sharer_or_viewer(&self) -> bool {
        !matches!(self, Self::NotShared)
    }

    pub fn as_keymap_context(&self) -> &'static str {
        match self {
            Self::NotShared => "SharedSessionStatus_NotShared",
            Self::ViewPending => "SharedSessionStatus_ViewPending",
            Self::ActiveViewer { role: Role::Reader } => "SharedSessionStatus_Reader",
            Self::ActiveViewer {
                role: Role::Executor | Role::Full,
            } => "SharedSessionStatus_Executor",
            Self::FinishedViewer => "SharedSessionStatus_FinishedViewer",
            Self::SharePendingPreBootstrap { .. } => "SharedSessionStatus_SharePendingPreBootstrap",
            Self::SharePending => "SharedSessionStatus_SharePending",
            Self::ActiveSharer => "SharedSessionStatus_ActiveSharer",
        }
    }

    pub fn active_viewer_keymap_context() -> ContextPredicate {
        id!(Self::reader().as_keymap_context()) | id!(Self::executor().as_keymap_context())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SharedSessionScrollbackType {
    None,
    FromBlock { block_index: BlockIndex },
    All,
}

impl SharedSessionScrollbackType {
    pub fn first_block_index(self, _model: &super::TerminalModel) -> BlockIndex {
        BlockIndex::zero()
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum SharedSessionActionSource {
    BlocklistContextMenu {
        block_index: Option<BlockIndex>,
    },
    Tab,
    PaneHeader,
    CommandPalette,
    OnboardingBlock,
    Closed {
        is_confirm_close_session: bool,
    },
    InactivityModal,
    NonUser,
    SharingDialog,
    RightClickMenu,
    FooterChip,
}

/// Stub — returns empty string.
pub fn join_native_intent(session_id: &SessionId) -> String {
    String::new()
}

/// Stub — returns empty string.
pub fn join_link(session_id: &SessionId) -> String {
    String::new()
}

/// Stub — returns None.
pub fn connect_endpoint(_path: String) -> Option<String> {
    None
}

// Stub for from_session_sharing_block_point which was in the deleted selections.rs.
impl<T> super::model::terminal_model::WithinBlock<T> {
    pub fn from_session_sharing_block_point(
        _point: session_sharing_protocol::common::BlockPoint,
        _block_list: &super::model::blocks::BlockList,
    ) -> Option<Self> {
        None
    }
}

// Keep the From<&Role> impl for InteractionState since it's used in terminal/input.rs.
impl From<&Role> for crate::editor::InteractionState {
    fn from(value: &Role) -> crate::editor::InteractionState {
        match value {
            Role::Reader => crate::editor::InteractionState::Selectable,
            Role::Executor | Role::Full => crate::editor::InteractionState::Editable,
        }
    }
}
