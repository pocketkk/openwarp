//! Stub — shared session AI agent removed from OpenWarp.
#![allow(dead_code, unused_imports, unused_variables)]

use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use prost::Message;
use warp_multi_agent_api::ResponseEvent;

pub fn encode_agent_response_event(event: &ResponseEvent) -> String {
    let bytes = event.encode_to_vec();
    STANDARD_NO_PAD.encode(bytes)
}
