use domain::model::message::Messages;
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

impl MessagesTable {
    pub fn into_messages(&self, document_id: &String) -> Messages {
        match self {
            MessagesTable::Event(table) => Messages::Event(table.into_event(document_id)),
            MessagesTable::SendMessage(table) => {
                Messages::SendMessages(table.into_messages(document_id))
            }
        }
    }
}
