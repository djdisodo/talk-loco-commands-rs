/*
 * Created on Wed Dec 02 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::BsonData;
use loco_protocol_derive::LocoResponse;

/// Send before server disconnect connection
#[derive(Debug, Clone, Serialize, Deserialize, BsonData, LocoResponse)]
pub struct KickOut {

    /// Kicked reasoon
    ///
    /// * Change server = 2
    /// * Login another = 0
    /// * Account deleted = 1
    pub reason: i16

}