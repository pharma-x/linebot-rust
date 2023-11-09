use chrono::{DateTime, Local};
use domain::model::message::send_message::{SendMessage, SendMessages};
use serde::{Deserialize, Serialize};

use crate::model::message::send_message::table::{message_type, SendMessageContentTable};

// TODO Flex Messageの実装
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManualSendMessageTable {
    // typeはmessageという値のみを取る
    #[serde(rename = "type")]
    #[serde(default = "message_type")]
    message_type: String,
    communication_type: ManualSendCommunicationTypeTable,
    sending_type: ManualSendSendingTypeTable,
    sending_method: ManualSendSendingMethodTable,
    sender: ManualSendSenderTable,
    pub messages: Vec<SendMessageContentTable>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ManualSendCommunicationTypeTable {
    Send,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ManualSendSendingTypeTable {
    Manual,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ManualSendSendingMethodTable {
    Reply,
    Push,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManualSendSenderTable {
    id: i64,
    name: String,
    picture_url: String,
    email: String,
    sender_role: ManualSendSenderRoleTable,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
enum ManualSendSenderRoleTable {
    Sender,
}

impl ManualSendMessageTable {
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
