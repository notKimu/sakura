use phf::{phf_map, Map};
use serde_json::Value;
use crate::{events::messages::on_message, discord::api::ApiClient};

use super::ready::on_ready;

pub static EVENT_MAP: Map<&'static str,fn(Value, ApiClient, redis::Client) -> ()> = phf_map! {
    "READY" =>  on_ready,
    "MESSAGE_CREATE" =>  on_message,
};