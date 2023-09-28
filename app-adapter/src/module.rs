use domain::model::{
    event::Event, talk_room::TalkRoom, user::User, user_auth::UserAuthData,
};
use domain::repository::{
    event::EventRepository, talk_room::TalkRoomRepository, user::UserRepository,
    user_auth::UserAuthRepository,
};
use crate::persistance::{firestore::Firestore, mysql::Db};
use crate::repository::{
    DatabaseRepositoryImpl, DbFirestoreRepositoryImpl, FirestoreRepositoryImpl,
    HttpClientRepositoryImpl,
};
use reqwest::Client;

pub trait RepositoriesModuleExt {
    type UserAuthRepo: UserAuthRepository;
    type UserRepo: UserRepository;
    type TalkRoomRepo: TalkRoomRepository;
    type EventRepo: EventRepository;
    fn user_auth_repository(&self) -> &Self::UserAuthRepo;
    fn user_repository(&self) -> &Self::UserRepo;
    fn talk_room_repository(&self) -> &Self::TalkRoomRepo;
    fn event_repository(&self) -> &Self::EventRepo;
}

pub struct RepositoriesModule {
    user_auth_repository: HttpClientRepositoryImpl<UserAuthData>,
    user_repository: DatabaseRepositoryImpl<User>,
    talk_room_repository: DbFirestoreRepositoryImpl<TalkRoom>,
    event_repository: FirestoreRepositoryImpl<Event>,
}

impl RepositoriesModuleExt for RepositoriesModule {
    type UserAuthRepo = HttpClientRepositoryImpl<UserAuthData>;
    type UserRepo = DatabaseRepositoryImpl<User>;
    type TalkRoomRepo = DbFirestoreRepositoryImpl<TalkRoom>;
    type EventRepo = FirestoreRepositoryImpl<Event>;

    fn user_auth_repository(&self) -> &Self::UserAuthRepo {
        &self.user_auth_repository
    }
    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }
    fn talk_room_repository(&self) -> &Self::TalkRoomRepo {
        &self.talk_room_repository
    }
    fn event_repository(&self) -> &Self::EventRepo {
        &self.event_repository
    }
}

impl RepositoriesModule {
    pub fn new(client: Client, db: Db, firestore: Firestore) -> Self {
        let user_auth_repository = HttpClientRepositoryImpl::new(client);
        let user_repository = DatabaseRepositoryImpl::new(db.clone());
        let talk_room_repository = DbFirestoreRepositoryImpl::new(db, firestore.clone());
        let event_repository = FirestoreRepositoryImpl::new(firestore);

        Self {
            user_auth_repository,
            user_repository,
            talk_room_repository,
            event_repository,
        }
    }
}
