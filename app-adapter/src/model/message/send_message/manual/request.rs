use serde::{Deserialize, Serialize};

use crate::model::message::send_message::request::SendMessageContentRequest;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ManualSendMessageRequest {
    to: String,
    messages: Vec<SendMessageContentRequest>,
}
