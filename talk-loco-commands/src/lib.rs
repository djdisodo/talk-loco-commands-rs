/*
 * Created on Mon Nov 30 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

//! Official client/server compatible Loco commands implementation.
//! Check request, response module for command datas.
//! Check structs module for types used in command datas.

use serde::{Serialize, Deserialize};

pub mod structs;

pub use talk_loco_commands_derive::BsonData;

pub mod booking;
pub mod chat;
pub mod checkin;

mod ping;

pub use ping::Ping;

pub trait BsonData<'a>: Serialize + Deserialize<'a>  {}