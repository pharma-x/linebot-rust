use chrono::{Local, TimeZone};

use crate::domain::{
    factory::talk_room::TalkRoomFactory,
    model::{
        event::NewEvent,
        talk_room::{TalkRoom, UpdateLatestMessage, UpdateTalkRoom},
    },
};

use super::FactoryImpl;

impl TalkRoomFactory for FactoryImpl<TalkRoom> {
    fn create_update_talk_room_event(
        &self,
        talk_room: TalkRoom,
        new_event: NewEvent,
    ) -> UpdateTalkRoom {
        match new_event {
            NewEvent::Follow(e) => {
                let latest_messaged_at = Local.timestamp_opt(e.timestamp, 0).unwrap();
                return UpdateTalkRoom {
                    id: talk_room.id,
                    primary_user_id: talk_room.primary_user_id,
                    display_name: talk_room.display_name,
                    rsvp: talk_room.rsvp,
                    pinned: talk_room.pinned,
                    follow: true,
                    latest_message: UpdateLatestMessage::Follow,
                    latest_messaged_at,
                    sort_time: talk_room.sort_time,
                    created_at: talk_room.created_at,
                    updated_at: latest_messaged_at,
                };
            }
            NewEvent::Unfollow(e) => {
                let latest_messaged_at = Local.timestamp_opt(e.timestamp, 0).unwrap();
                return UpdateTalkRoom {
                    id: talk_room.id,
                    primary_user_id: talk_room.primary_user_id,
                    display_name: talk_room.display_name,
                    rsvp: talk_room.rsvp,
                    pinned: talk_room.pinned,
                    follow: false,
                    latest_message: UpdateLatestMessage::Unfollow,
                    latest_messaged_at,
                    sort_time: talk_room.sort_time,
                    created_at: talk_room.created_at,
                    updated_at: latest_messaged_at,
                };
            }
            NewEvent::Postback(e) => {
                let latest_messaged_at = Local.timestamp_opt(e.timestamp, 0).unwrap();
                return UpdateTalkRoom {
                    id: talk_room.id,
                    primary_user_id: talk_room.primary_user_id,
                    display_name: talk_room.display_name,
                    rsvp: talk_room.rsvp,
                    pinned: talk_room.pinned,
                    follow: talk_room.follow,
                    latest_message: UpdateLatestMessage::Postback,
                    latest_messaged_at,
                    sort_time: talk_room.sort_time,
                    created_at: talk_room.created_at,
                    updated_at: latest_messaged_at,
                };
            }
            NewEvent::VideoPlayComplete(e) => {
                let latest_messaged_at = Local.timestamp_opt(e.timestamp, 0).unwrap();
                return UpdateTalkRoom {
                    id: talk_room.id,
                    primary_user_id: talk_room.primary_user_id,
                    display_name: talk_room.display_name,
                    rsvp: talk_room.rsvp,
                    pinned: talk_room.pinned,
                    follow: talk_room.follow,
                    latest_message: UpdateLatestMessage::VideoPlayComplete,
                    latest_messaged_at,
                    sort_time: talk_room.sort_time,
                    created_at: talk_room.created_at,
                    updated_at: latest_messaged_at,
                };
            }
            NewEvent::Message(e) => {
                let latest_messaged_at = Local.timestamp_opt(e.timestamp, 0).unwrap();
                return UpdateTalkRoom {
                    id: talk_room.id,
                    primary_user_id: talk_room.primary_user_id,
                    display_name: talk_room.display_name,
                    rsvp: true,
                    pinned: talk_room.pinned,
                    follow: talk_room.follow,
                    latest_message: UpdateLatestMessage::Message(e.message),
                    latest_messaged_at,
                    sort_time: talk_room.sort_time,
                    created_at: talk_room.created_at,
                    updated_at: latest_messaged_at,
                };
            }
        };
    }
}
