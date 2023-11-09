use crate::gateway::HttpClientRepositoryImpl;
use crate::persistance::{firestore::Firestore, mysql::Db};
use crate::repository::{DatabaseRepositoryImpl, DbFirestoreRepositoryImpl};
use domain::gateway::{send_message::SendMessageGateway, user_auth::UserAuthGateway};
use domain::model::message::send_message::SendMessage;
use domain::model::{talk_room::TalkRoom, user::User, user_auth::UserAuthData};
use domain::repository::{talk_room::TalkRoomRepository, user::UserRepository};
use reqwest::Client;

pub trait AdaptersModuleExt {
    type UserAuthRepo: UserAuthGateway;
    type UserRepo: UserRepository;
    type TalkRoomRepo: TalkRoomRepository;
    type SendMessageGate: SendMessageGateway;
    fn user_auth_gateway(&self) -> &Self::UserAuthRepo;
    fn user_repository(&self) -> &Self::UserRepo;
    fn talk_room_repository(&self) -> &Self::TalkRoomRepo;
    fn send_message_gateway(&self) -> &Self::SendMessageGate;
}

pub struct AdaptersModule {
    user_auth_gateway: HttpClientRepositoryImpl<UserAuthData>,
    user_repository: DatabaseRepositoryImpl<User>,
    talk_room_repository: DbFirestoreRepositoryImpl<TalkRoom>,
    send_message_gateway: HttpClientRepositoryImpl<SendMessage>,
}

impl AdaptersModuleExt for AdaptersModule {
    type UserAuthRepo = HttpClientRepositoryImpl<UserAuthData>;
    type UserRepo = DatabaseRepositoryImpl<User>;
    type TalkRoomRepo = DbFirestoreRepositoryImpl<TalkRoom>;
    type SendMessageGate = HttpClientRepositoryImpl<SendMessage>;

    fn user_auth_gateway(&self) -> &Self::UserAuthRepo {
        &self.user_auth_gateway
    }
    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }
    fn talk_room_repository(&self) -> &Self::TalkRoomRepo {
        &self.talk_room_repository
    }
    fn send_message_gateway(&self) -> &Self::SendMessageGate {
        &self.send_message_gateway
    }
}

impl AdaptersModule {
    pub fn new(client: Client, db: Db, firestore: Firestore) -> Self {
        let user_auth_gateway = HttpClientRepositoryImpl::new(client.clone());
        let user_repository = DatabaseRepositoryImpl::new(db.clone());
        let talk_room_repository = DbFirestoreRepositoryImpl::new(db, firestore.clone());
        let send_message_gateway = HttpClientRepositoryImpl::new(client);

        Self {
            user_auth_gateway,
            user_repository,
            talk_room_repository,
            send_message_gateway,
        }
    }
}

pub mod test {
    use super::AdaptersModuleExt;
    use domain::gateway::{send_message::MockSendMessageGateway, user_auth::MockUserAuthGateway};
    use domain::repository::{talk_room::MockTalkRoomRepository, user::MockUserRepository};

    pub struct TestAdaptersModule {
        user_auth_gateway: MockUserAuthGateway,
        user_repository: MockUserRepository,
        talk_room_repository: MockTalkRoomRepository,
        send_message_gateway: MockSendMessageGateway,
    }

    impl AdaptersModuleExt for TestAdaptersModule {
        type UserAuthRepo = MockUserAuthGateway;
        type UserRepo = MockUserRepository;
        type TalkRoomRepo = MockTalkRoomRepository;
        type SendMessageGate = MockSendMessageGateway;

        fn user_auth_gateway(&self) -> &Self::UserAuthRepo {
            &self.user_auth_gateway
        }
        fn user_repository(&self) -> &Self::UserRepo {
            &self.user_repository
        }
        fn talk_room_repository(&self) -> &Self::TalkRoomRepo {
            &self.talk_room_repository
        }
        fn send_message_gateway(&self) -> &Self::SendMessageGate {
            &self.send_message_gateway
        }
    }

    impl TestAdaptersModule {
        pub fn new(
            user_auth_gateway: MockUserAuthGateway,
            user_repository: MockUserRepository,
            talk_room_repository: MockTalkRoomRepository,
            send_message_gateway: MockSendMessageGateway,
        ) -> Self {
            Self {
                user_auth_gateway,
                user_repository,
                talk_room_repository,
                send_message_gateway,
            }
        }
    }
}
