//! Stub — shared session replay agent conversations removed from OpenWarp.
#![allow(dead_code, unused_imports, unused_variables)]

use crate::ai::agent::conversation::AIConversation;
use warp_multi_agent_api::ResponseEvent;

pub fn reconstruct_response_events_from_conversations(
    _conversations: &[AIConversation],
) -> Vec<ResponseEvent> {
    Vec::new()
}
