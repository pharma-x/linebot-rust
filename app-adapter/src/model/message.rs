use serde::{Deserialize, Serialize};

use crate::model::message::{event::EventTable, send_message::SendMessageTable};

pub mod event;
pub mod send_message;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum MessagesTable {
    Event(EventTable),
    SendMessage(SendMessageTable),
}
