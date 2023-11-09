use chrono::{DateTime, Local};
use domain::model::message::send_message::{NewSendMessages, SendMessage, SendMessages};
use serde::{Deserialize, Serialize};

use crate::model::message::send_message::table::{message_type, SendMessageContentTable};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BotSendMessageTable {
    // typeはmessageという値のみを取る
    #[serde(rename = "type")]
    #[serde(default = "message_type")]
    message_type: String,
    communication_type: BotSendCommunicationTypeTable,
    sending_type: BotSendSendingTypeTable,
    sending_method: BotSendSendingMethodTable,
    pub messages: Vec<SendMessageContentTable>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl BotSendMessageTable {
    pub fn new(
        communication_type: BotSendCommunicationTypeTable,
        sending_type: BotSendSendingTypeTable,
        sending_method: BotSendSendingMethodTable,
        messages: Vec<SendMessageContentTable>,
        created_at: DateTime<Local>,
        updated_at: DateTime<Local>,
    ) -> Self {
        Self {
            message_type: message_type(),
            communication_type,
            sending_type,
            sending_method,
            messages,
            created_at,
            updated_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BotSendCommunicationTypeTable {
    Send,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BotSendSendingTypeTable {
    Bot,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BotSendSendingMethodTable {
    Reply,
    // Botでの返信でもreplyTokenを使わずにpushで送ることができる
    Push,
}

impl From<NewSendMessages> for BotSendMessageTable {
    fn from(s: NewSendMessages) -> Self {
        let created_at = *s.messages[0].created_at();
        BotSendMessageTable::new(
            BotSendCommunicationTypeTable::Send,
            BotSendSendingTypeTable::Bot,
            BotSendSendingMethodTable::Reply,
            s.messages
                .iter()
                .map(|m| m.clone().into())
                .collect::<Vec<SendMessageContentTable>>(),
            created_at,
            created_at,
        )
    }
}

impl BotSendMessageTable {
    pub fn into_messages(&self, document_id: &String) -> SendMessages {
        SendMessages {
            id: document_id
                .to_string()
                .try_into()
                .unwrap_or_else(|_| panic!("Failed to convert String {} to UUID", document_id)),
            messages: self
                .messages
                .iter()
                .map(|m| m.clone().into())
                .collect::<Vec<SendMessage>>(),
        }
    }
}
