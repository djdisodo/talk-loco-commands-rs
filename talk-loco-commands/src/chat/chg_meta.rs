/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::{BsonData, structs::chatroom::ChatroomMeta};
use loco_protocol_derive::LocoResponse;

/// Sync Chatroom meta update
#[derive(Debug, Clone, Serialize, Deserialize, BsonData, LocoResponse)]
pub struct ChgMeta {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Chatroom meta item. Update same type meta.
    pub meta: ChatroomMeta

}