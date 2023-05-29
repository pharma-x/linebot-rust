use derive_new::new;

#[derive(new, Debug, Clone)]
pub enum EventType {
    Message,
    Follow,
    Unfollow,
    Postback,
    VideoPlayComplete,
}

#[derive(new, Debug, Clone)]
pub struct DeliveryContext {
    is_redelivery: bool,
}
