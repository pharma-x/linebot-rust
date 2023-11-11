use domain::model::message::send_message::{NewSendMessages, NewSendSendingType, SendMessages};
use serde::{Deserialize, Serialize};

use self::table::{BotSendMessageTable, ManualSendMessageTable, SendMessageContentTable};

pub mod request;
pub mod table;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum SendMessageTable {
    Bot(BotSendMessageTable),
    Manual(ManualSendMessageTable),
}

impl From<NewSendMessages> for SendMessageTable {
    fn from(s: NewSendMessages) -> Self {
        let created_at = *s.messages[0].created_at();
        match s.sending_type {
            NewSendSendingType::Bot => {
                let table = BotSendMessageTable::new(
                    s.sending_method.into(),
                    s.messages
                        .iter()
                        .map(|m| m.clone().into())
                        .collect::<Vec<SendMessageContentTable>>(),
                    created_at,
                    created_at,
                );
                SendMessageTable::Bot(table)
            }
            NewSendSendingType::Manual => {
                let table = ManualSendMessageTable::new(
                    s.sending_method.into(),
                    s.sender.unwrap().into(),
                    s.messages
                        .iter()
                        .map(|m| m.clone().into())
                        .collect::<Vec<SendMessageContentTable>>(),
                    created_at,
                    created_at,
                );
                SendMessageTable::Manual(table)
            }
        }
    }
}

impl SendMessageTable {
    pub fn into_messages(&self, id: &String) -> SendMessages {
        match self {
            SendMessageTable::Bot(table) => table.into_messages(id),
            SendMessageTable::Manual(table) => table.into_messages(id),
        }
    }
}
