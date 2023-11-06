use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::model::send_message::SendMessageRequest;

// TODO Flex Messageの実装
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub struct BotSendMessageTable {
    communication_type: BotSendCommunicationTypeTable,
    sending_type: BotSendSendingTypeTable,
    sending_method: BotSendSendingMethodTable,
    sender: BotSendSenderTable,
    pub message: Vec<SendMessageRequest>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BotSendSenderTable {
    id: i64,
    name: String,
    picture_url: String,
    email: String,
    sender_role: BotSendSenderRoleTable,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
enum BotSendSenderRoleTable {
    Sender,
}
