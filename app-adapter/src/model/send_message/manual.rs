use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::model::send_message::SendMessageRequest;

// TODO Flex Messageの実装
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub struct ManualSendMessageTable {
    communication_type: ManualSendCommunicationTypeTable,
    sending_type: ManualSendSendingTypeTable,
    sending_method: ManualSendSendingMethodTable,
    sender: ManualSendSenderTable,
    pub message: Vec<SendMessageRequest>,
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
