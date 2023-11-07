use serde::{Deserialize, Serialize};

use super::{event::EventTable, send_message::bot::BotSendMessageTable};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum MessagesTable {
    Event(EventTable),
    BotSendMessage(BotSendMessageTable),
}
