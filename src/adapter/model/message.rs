use chrono::{DateTime, Local};

pub struct MessageTable {
    documentId: String,
    talkRoomId: String,
    replyToken: Option<String>,
    webhookEventId: Option<String>,
    deliveryContext: Option<String>,
    communicationType: CommunicationType,
    sendingType: SendingType,
    sendingMethod: SendingMethod,
    sender: Option<Sender>,
    eventType: Option<EventType>,
    messages: Vec<MessageContent>,
    createdAt: DateTime<Local>,
    updatedAt: DateTime<Local>,
}

pub struct DeliveryContext {
    is_redelivery: bool,
}

enum CommunicationType {
    Send(String),
    Receive(String),
}

enum SendingType {
    Manual(String),
    Bot(String),
}

enum SendingMethod {
    Reply(String),
    Push(String),
}

enum EventType {
    Message(String),
    Follow(String),
    Unfollow(String),
    Postback(String),
    VideoPlayComplete(String),
}
