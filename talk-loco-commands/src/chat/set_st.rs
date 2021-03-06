/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::BsonData;
use loco_protocol_derive::LocoPacketPair;

#[derive(LocoPacketPair)]
#[loco_packet_pair(SetStRequest, SetStResponse)]
pub struct SetSt;

/// Update client status
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct SetStRequest {

	/// Status
	///
	/// * Unlocked = 1
	/// * Locked = 2
	#[serde(rename = "st")]
	pub status: i8

}

/// Update client status
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct SetStResponse;