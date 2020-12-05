/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::{BsonData, structs::chat::Chatlog};
use loco_protocol_derive::LocoResponse;

/// Send when new user join.
#[derive(Debug, Clone, Serialize, Deserialize, BsonData, LocoResponse)]
pub struct NewMem {
    
    /// Join feed chat.(?)
    #[serde(rename = "chatLog")]
    pub chat_log: Chatlog

}