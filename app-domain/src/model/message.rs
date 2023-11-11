use crate::model::message::{
    event::{Event, NewEvent},
    send_message::{NewSendMessages, SendMessages},
};

pub mod event;
pub mod send_message;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Messages {
    Event(Event),
    SendMessages(SendMessages),
}

// talkRoomのupdate時にも使う
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NewMessages {
    Event(NewEvent),
    SendMessages(NewSendMessages),
}
