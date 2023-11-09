use domain::model::message::send_message::SendMessages;
use serde::{Deserialize, Serialize};

use self::{
    bot::{request::BotSendMessageRequest, table::BotSendMessageTable},
    manual::{request::ManualSendMessageRequest, table::ManualSendMessageTable},
};

pub mod bot;
pub mod manual;
pub mod request;
pub mod table;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SendMessageRequest {
    Manual(ManualSendMessageRequest),
    Bot(BotSendMessageRequest),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum SendMessageTable {
    Bot(BotSendMessageTable),
    Manual(ManualSendMessageTable),
}

impl SendMessageTable {
    pub fn into_messages(&self, id: &String) -> SendMessages {
        match self {
            SendMessageTable::Bot(table) => table.into_messages(id),
            SendMessageTable::Manual(table) => table.into_messages(id),
        }
    }
}
