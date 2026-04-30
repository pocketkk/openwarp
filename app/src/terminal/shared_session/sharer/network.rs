//! Stub — shared session network removed from OpenWarp.
#![allow(dead_code, unused_imports, unused_variables)]

use std::sync::Arc;

use parking_lot::FairMutex;
use session_sharing_protocol::common::{
    ActivePrompt, AgentPromptFailureReason, AgentPromptRequest,
    AgentPromptRequestId, CommandExecutionFailureReason, CommandExecutionRequestId,
    ControlAction, ControlActionFailureReason, ControlActionRequestId,
    ParticipantId, ParticipantList, ParticipantPresenceUpdate,
    Role, RoleRequestId, RoleRequestResponse, Selection, SessionId,
    UniversalDeveloperInputContextUpdate, WindowSize,
    WriteToPtyFailureReason, WriteToPtyRequestId,
};
use session_sharing_protocol::sharer::{
    AddGuestsResponse, FailedToAddGuestsReason, FailedToInitializeSessionReason,
    Lifetime, LinkAccessLevelUpdateResponse, RemoveGuestResponse,
    RoleUpdateReason, SessionEndedReason, SessionSourceType,
    SessionTerminatedReason, TeamAccessLevelUpdateResponse, UpdatePendingUserRoleResponse,
};

use crate::auth::UserUid;
use crate::editor::{CrdtOperation, ReplicaId};
use crate::terminal::model::block::BlockId;
use crate::terminal::shared_session::SharedSessionScrollbackType;
use crate::terminal::TerminalModel;

use warpui::{Entity, ModelContext};

pub fn session_terminated_reason_string(
    _reason: &SessionTerminatedReason,
    _max_session_size: byte_unit::Byte,
) -> String {
    "Session ended.".to_string()
}

pub fn failed_to_initialize_session_user_error(
    _reason: &FailedToInitializeSessionReason,
) -> String {
    "Failed to initialize session.".to_string()
}

pub fn failed_to_add_guests_user_error(_reason: &FailedToAddGuestsReason) -> String {
    "Failed to add guests.".to_string()
}

pub struct Network;

impl Network {
    pub fn new(
        _model: Arc<FairMutex<TerminalModel>>,
        _ordered_events_rx: async_channel::Receiver<session_sharing_protocol::common::OrderedTerminalEventType>,
        _scrollback_type: SharedSessionScrollbackType,
        _active_prompt: ActivePrompt,
        _selection: Option<Selection>,
        _input_replica_id: ReplicaId,
        _terminal_view_id: warpui::EntityId,
        _universal_developer_input_context: session_sharing_protocol::common::UniversalDeveloperInputContext,
        _lifetime: Lifetime,
        _source_type: SessionSourceType,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        panic!("Shared session networking has been removed from OpenWarp");
    }

    pub fn is_connected(&self) -> bool {
        false
    }

    pub fn max_session_size(&self) -> byte_unit::Byte {
        byte_unit::Byte::from_u64(0)
    }

    pub fn end_session(&mut self, _reason: SessionEndedReason) {}
    pub fn send_active_prompt_update_if_changed(&mut self, _active_prompt: ActivePrompt) {}
    pub fn send_presence_selection_if_changed(&mut self, _selection: Option<Selection>) {}
    pub fn send_role_update(&mut self, _participant_id: ParticipantId, _role: Role) {}
    pub fn send_user_role_update(&mut self, _user_uid: UserUid, _role: Role) {}
    pub fn send_pending_user_role_update(&mut self, _email: String, _role: Role) {}
    pub fn send_add_guests(&mut self, _emails: Vec<String>, _role: Role) {}
    pub fn send_remove_guest(&mut self, _user_uid: UserUid) {}
    pub fn send_remove_pending_guest(&mut self, _email: String) {}
    pub fn send_make_all_participants_readers(&mut self, _reason: RoleUpdateReason) {}
    pub fn send_role_request_response(
        &mut self,
        _participant_id: ParticipantId,
        _request_id: RoleRequestId,
        _response: RoleRequestResponse,
    ) {
    }
    pub fn send_input_update<'a>(
        &mut self,
        _block_id: &BlockId,
        _operations: impl Iterator<Item = &'a CrdtOperation>,
    ) {
    }
    pub fn send_command_execution_rejection(
        &mut self,
        _id: CommandExecutionRequestId,
        _participant_id: ParticipantId,
        _reason: CommandExecutionFailureReason,
    ) {
    }
    pub fn send_write_to_pty_rejection(
        &mut self,
        _id: WriteToPtyRequestId,
        _reason: WriteToPtyFailureReason,
    ) {
    }
    pub fn send_agent_prompt_rejection(
        &mut self,
        _id: AgentPromptRequestId,
        _participant_id: ParticipantId,
        _reason: AgentPromptFailureReason,
    ) {
    }
    pub fn send_control_action_rejection(
        &mut self,
        _participant_id: ParticipantId,
        _request_id: ControlActionRequestId,
        _reason: ControlActionFailureReason,
    ) {
    }
    pub fn send_link_permission_update(&mut self, _role: Option<Role>) {}
    pub fn send_team_permission_update(&mut self, _role: Option<Role>, _team_uid: String) {}
    pub fn send_universal_developer_input_context_update(
        &mut self,
        _update: UniversalDeveloperInputContextUpdate,
    ) {
    }
    pub fn reconnect_websocket(&mut self, _ctx: &mut ModelContext<Self>) {}
}

pub enum NetworkEvent {
    SharedSessionCreatedSuccessfully {
        session_id: SessionId,
        sharer_id: ParticipantId,
        sharer_firebase_uid: UserUid,
    },
    FailedToCreateSharedSession {
        reason: FailedToInitializeSessionReason,
        cause: Option<Arc<anyhow::Error>>,
    },
    SessionTerminated {
        reason: SessionTerminatedReason,
    },
    Reconnecting,
    ParticipantListUpdated(Box<ParticipantList>),
    ParticipantPresenceUpdated(ParticipantPresenceUpdate),
    ReconnectedSuccessfully,
    FailedToReconnect,
    RoleRequested {
        participant_id: ParticipantId,
        role_request_id: RoleRequestId,
        role: Role,
    },
    RoleRequestCancelled {
        participant_id: ParticipantId,
        role_request_id: RoleRequestId,
    },
    ParticipantRoleChanged {
        participant_id: ParticipantId,
        role: Role,
    },
    InputUpdated {
        block_id: BlockId,
        operations: Vec<CrdtOperation>,
    },
    CommandExecutionRequested {
        id: CommandExecutionRequestId,
        participant_id: ParticipantId,
        block_id: BlockId,
        command: String,
    },
    WriteToPtyRequested {
        id: WriteToPtyRequestId,
        bytes: Vec<u8>,
    },
    AgentPromptRequested {
        id: AgentPromptRequestId,
        participant_id: ParticipantId,
        request: AgentPromptRequest,
    },
    LinkAccessLevelUpdateResponse {
        response: LinkAccessLevelUpdateResponse,
    },
    AddGuestsResponse {
        response: AddGuestsResponse,
    },
    RemoveGuestResponse {
        response: RemoveGuestResponse,
    },
    UpdatePendingUserRoleResponse {
        response: UpdatePendingUserRoleResponse,
    },
    TeamAccessLevelUpdateResponse {
        response: TeamAccessLevelUpdateResponse,
    },
    UniversalDeveloperInputContextUpdated(UniversalDeveloperInputContextUpdate),
    ControlActionRequested {
        participant_id: ParticipantId,
        request_id: ControlActionRequestId,
        action: ControlAction,
    },
    ViewerTerminalSizeReported {
        window_size: WindowSize,
    },
}

impl Entity for Network {
    type Event = NetworkEvent;
}
